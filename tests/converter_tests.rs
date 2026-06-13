use image::{ImageFormat, Rgba, RgbaImage};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use webp_converter::converter::{
    collect_image_files, convert_to_webp, convert_to_webp_core, process_batch, resize_image,
    ConversionOptions, ConverterError, FileAction, OperationMode,
};

fn make_png(dir: &TempDir, name: &str) -> PathBuf {
    let path = dir.path().join(name);
    let img = RgbaImage::from_pixel(64, 64, Rgba([100, 150, 200, 255]));
    image::DynamicImage::ImageRgba8(img).save(&path).unwrap();
    path
}

fn make_png_with_alpha(dir: &TempDir, name: &str, alpha: u8) -> PathBuf {
    let path = dir.path().join(name);
    let img = RgbaImage::from_pixel(64, 64, Rgba([100, 150, 200, alpha]));
    image::DynamicImage::ImageRgba8(img).save(&path).unwrap();
    path
}

fn make_jpeg(dir: &TempDir, name: &str) -> PathBuf {
    let path = dir.path().join(name);
    let img = image::RgbImage::from_pixel(64, 64, image::Rgb([200, 100, 50]));
    image::DynamicImage::ImageRgb8(img)
        .save_with_format(&path, ImageFormat::Jpeg)
        .unwrap();
    path
}

fn make_bmp(dir: &TempDir, name: &str) -> PathBuf {
    let path = dir.path().join(name);
    let img = image::RgbImage::from_pixel(32, 32, image::Rgb([50, 100, 200]));
    image::DynamicImage::ImageRgb8(img)
        .save_with_format(&path, ImageFormat::Bmp)
        .unwrap();
    path
}

fn make_tiff(dir: &TempDir, name: &str) -> PathBuf {
    let path = dir.path().join(name);
    let img = image::RgbImage::from_pixel(32, 32, image::Rgb([80, 120, 160]));
    image::DynamicImage::ImageRgb8(img)
        .save_with_format(&path, ImageFormat::Tiff)
        .unwrap();
    path
}

fn default_opts() -> ConversionOptions {
    ConversionOptions::default()
}

// 1. JPEG → WebP core conversion
#[test]
fn test_jpeg_to_webp_core() {
    let dir = TempDir::new().unwrap();
    let input = make_jpeg(&dir, "input.jpg");
    let output = dir.path().join("output.webp");

    let metrics = convert_to_webp_core(&input, &output, &default_opts()).unwrap();
    assert!(output.exists(), "WebP output should exist");
    assert!(metrics.new_size > 0, "Output file should not be empty");
    assert_eq!(metrics.quality, 80);
    assert!(metrics.original_size > 0);
}

// 2. PNG → WebP with RGBA alpha preserved (lossless)
#[test]
fn test_png_to_webp_rgba_alpha_preserved() {
    let dir = TempDir::new().unwrap();
    let input = make_png_with_alpha(&dir, "input.png", 128);
    let output = dir.path().join("output.webp");

    let opts = ConversionOptions::new(80, true, false).unwrap();
    convert_to_webp_core(&input, &output, &opts).unwrap();
    assert!(output.exists());

    let result = image::open(&output).unwrap().into_rgba8();
    let pixel = result.get_pixel(0, 0);
    assert_eq!(
        pixel[3], 128,
        "Alpha channel should be preserved in lossless WebP"
    );
}

// 3. Fully transparent pixel alpha=0 survives lossless round-trip
#[test]
fn test_png_to_webp_full_transparency_preserved() {
    let dir = TempDir::new().unwrap();
    let input = make_png_with_alpha(&dir, "transparent.png", 0);
    let output = dir.path().join("transparent.webp");

    let opts = ConversionOptions::new(80, true, false).unwrap();
    convert_to_webp_core(&input, &output, &opts).unwrap();

    let result = image::open(&output).unwrap().into_rgba8();
    let pixel = result.get_pixel(0, 0);
    assert_eq!(
        pixel[3], 0,
        "Fully transparent pixel should remain transparent"
    );
}

// 4. Lossless output is valid WebP (loadable image)
#[test]
fn test_lossless_output_valid() {
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "input.png");
    let output = dir.path().join("lossless.webp");

    let opts = ConversionOptions::new(80, true, false).unwrap();
    convert_to_webp_core(&input, &output, &opts).unwrap();

    let img = image::open(&output).expect("Lossless WebP should be a valid image");
    assert!(img.width() > 0 && img.height() > 0);
}

// 5. Quality range: q=0, q=50, q=100 produce valid outputs
#[test]
fn test_quality_range() {
    let dir = TempDir::new().unwrap();
    let input = make_jpeg(&dir, "input.jpg");

    for q in [0u8, 50, 100] {
        let output = dir.path().join(format!("q{}.webp", q));
        let opts = ConversionOptions::new(q, false, false).unwrap();
        convert_to_webp_core(&input, &output, &opts).unwrap();
        assert!(output.exists(), "q={} output should exist", q);
        let img = image::open(&output).unwrap();
        assert!(img.width() > 0, "q={} output should be valid image", q);
    }
}

// 6. Invalid quality (>100) returns ConverterError::InvalidQuality
#[test]
fn test_invalid_quality() {
    let result = ConversionOptions::new(101, false, false);
    assert!(
        matches!(result, Err(ConverterError::InvalidQuality(101))),
        "Quality > 100 should return InvalidQuality error"
    );
}

// 7. Resize-only: output format preserved (PNG in → PNG out)
#[test]
fn test_resize_only_format_preserved() {
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "input.png");
    let output = dir.path().join("resized.png");

    resize_image(&input, &output).unwrap();
    assert!(output.exists(), "Resized PNG should exist");

    let fmt = image::ImageFormat::from_path(&output).unwrap();
    assert_eq!(fmt, ImageFormat::Png, "Output should remain PNG format");
}

// 8. Resize-only: dimensions unchanged when no resize occurs
#[test]
fn test_resize_only_dimensions() {
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "input.png");
    let output = dir.path().join("resized.png");

    resize_image(&input, &output).unwrap();

    let img = image::open(&output).unwrap();
    assert_eq!(img.width(), 64);
    assert_eq!(img.height(), 64);
}

// 9. Batch collect_image_files: directory with .png/.jpg/.webp
#[test]
fn test_collect_image_files_directory() {
    let dir = TempDir::new().unwrap();
    make_png(&dir, "a.png");
    make_jpeg(&dir, "b.jpg");

    // Create a .webp file manually
    let webp_path = dir.path().join("c.webp");
    let img = RgbaImage::from_pixel(16, 16, Rgba([255, 0, 0, 255]));
    let enc = webp::Encoder::from_rgba(img.as_raw(), 16, 16);
    fs::write(&webp_path, &*enc.encode_lossless()).unwrap();

    let out_dir = TempDir::new().unwrap();
    let files = collect_image_files(
        &[dir.path().to_path_buf()],
        out_dir.path(),
        OperationMode::ConvertToWebP,
    );

    assert_eq!(files.len(), 3, "Should collect all 3 image files");

    let actions: Vec<FileAction> = files.iter().map(|(_, _, a)| *a).collect();
    let convert_count = actions
        .iter()
        .filter(|&&a| a == FileAction::Convert)
        .count();
    let copy_count = actions.iter().filter(|&&a| a == FileAction::Copy).count();
    assert_eq!(convert_count, 2, "PNG and JPG should be converted");
    assert_eq!(copy_count, 1, "WebP should be copied");
}

// 10. Batch recursive structure preservation
#[test]
fn test_collect_recursive_structure() {
    let dir = TempDir::new().unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();

    let img = RgbaImage::from_pixel(16, 16, Rgba([0, 255, 0, 255]));
    image::DynamicImage::ImageRgba8(img)
        .save(sub.join("img.png"))
        .unwrap();

    let out_dir = TempDir::new().unwrap();
    let files = collect_image_files(
        &[dir.path().to_path_buf()],
        out_dir.path(),
        OperationMode::ConvertToWebP,
    );

    assert_eq!(files.len(), 1);
    let (_, out_path, _) = &files[0];
    assert!(
        out_path.to_str().unwrap().contains("sub"),
        "Output should preserve sub/ structure, got: {:?}",
        out_path
    );
}

// 11. Parallel batch: 20 files converted concurrently without corruption
#[test]
fn test_parallel_batch_no_corruption() {
    let dir = TempDir::new().unwrap();
    for i in 0..20 {
        make_png(&dir, &format!("img_{:02}.png", i));
    }

    let out_dir = TempDir::new().unwrap();
    let files = collect_image_files(
        &[dir.path().to_path_buf()],
        out_dir.path(),
        OperationMode::ConvertToWebP,
    );

    assert_eq!(files.len(), 20);
    let result = process_batch(files, &default_opts());
    assert_eq!(result.failed.len(), 0, "No failures expected");
    assert_eq!(result.converted, 20);

    // Verify each output is a valid image
    for entry in fs::read_dir(out_dir.path()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("webp") {
            let img =
                image::open(&path).unwrap_or_else(|_| panic!("{:?} should be valid WebP", path));
            assert!(img.width() > 0);
        }
    }
}

// 12. Overwrite declined: existing file unchanged
#[test]
fn test_overwrite_declined() {
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "input.png");
    let output = dir.path().join("output.webp");

    // Write sentinel content
    fs::write(&output, b"SENTINEL").unwrap();

    let result = convert_to_webp(&input, Some(&output), &default_opts(), Some(&|_| false)).unwrap();
    assert!(!result, "Should return false when overwrite declined");
    assert_eq!(
        fs::read(&output).unwrap(),
        b"SENTINEL",
        "File should be unchanged when overwrite declined"
    );
}

// 13. Overwrite accepted with force flag
#[test]
fn test_overwrite_with_force() {
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "input.png");
    let output = dir.path().join("output.webp");
    fs::write(&output, b"OLD").unwrap();

    let opts = ConversionOptions::new(80, false, true).unwrap();
    let result = convert_to_webp(&input, Some(&output), &opts, None).unwrap();
    assert!(result, "Should return true with force=true");
    assert_ne!(
        fs::read(&output).unwrap(),
        b"OLD",
        "File should be overwritten with force=true"
    );
}

// 14. Missing input returns ConverterError::InputNotFound
#[test]
fn test_missing_input_file() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("nonexistent.png");
    let output = dir.path().join("output.webp");

    let err = convert_to_webp_core(&input, &output, &default_opts()).unwrap_err();
    assert!(
        matches!(err, ConverterError::InputNotFound(_)),
        "Expected InputNotFound, got: {:?}",
        err
    );
}

// 15. Corrupt image bytes returns ConverterError::Decode
#[test]
fn test_corrupt_image() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("corrupt.png");
    fs::write(&input, b"this is not a valid image file").unwrap();
    let output = dir.path().join("output.webp");

    let err = convert_to_webp_core(&input, &output, &default_opts()).unwrap_err();
    assert!(
        matches!(err, ConverterError::Decode(_)),
        "Expected Decode error for corrupt file, got: {:?}",
        err
    );
}

// 16. Read-only output directory returns IO error (Unix only)
#[cfg(unix)]
#[test]
fn test_readonly_output_dir() {
    use std::os::unix::fs::PermissionsExt;
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "input.png");

    let readonly_dir = TempDir::new().unwrap();
    let perms = std::fs::Permissions::from_mode(0o444);
    fs::set_permissions(readonly_dir.path(), perms).unwrap();

    let output = readonly_dir.path().join("output.webp");
    let err = convert_to_webp_core(&input, &output, &default_opts()).unwrap_err();
    assert!(
        matches!(err, ConverterError::Io(_)),
        "Expected Io error for read-only dir, got: {:?}",
        err
    );
}

// 17. BMP → WebP
#[test]
fn test_bmp_to_webp() {
    let dir = TempDir::new().unwrap();
    let input = make_bmp(&dir, "input.bmp");
    let output = dir.path().join("output.webp");

    convert_to_webp_core(&input, &output, &default_opts()).unwrap();
    assert!(output.exists());
    let img = image::open(&output).unwrap();
    assert!(img.width() > 0);
}

// 18. TIFF → WebP
#[test]
fn test_tiff_to_webp() {
    let dir = TempDir::new().unwrap();
    let input = make_tiff(&dir, "input.tiff");
    let output = dir.path().join("output.webp");

    convert_to_webp_core(&input, &output, &default_opts()).unwrap();
    assert!(output.exists());
    let img = image::open(&output).unwrap();
    assert!(img.width() > 0);
}

// 19. Default output path: same dir, .webp extension
#[test]
fn test_default_output_path() {
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "myimage.png");

    let result = convert_to_webp(&input, None, &default_opts(), None).unwrap();
    assert!(result);

    let expected_output = input.with_extension("webp");
    assert!(
        expected_output.exists(),
        "Default output {:?} should exist",
        expected_output
    );
}

// 20. Single file input in collect_image_files
#[test]
fn test_collect_single_file() {
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "single.png");
    let out_dir = TempDir::new().unwrap();

    let files = collect_image_files(
        std::slice::from_ref(&input),
        out_dir.path(),
        OperationMode::ConvertToWebP,
    );

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].2, FileAction::Convert);
    assert!(
        files[0].1.to_str().unwrap().ends_with(".webp"),
        "Output path should have .webp extension"
    );
}

// 21. Non-image file skipped by collect_image_files
#[test]
fn test_collect_skips_non_image() {
    let dir = TempDir::new().unwrap();
    make_png(&dir, "image.png");
    fs::write(dir.path().join("readme.txt"), b"not an image").unwrap();
    fs::write(dir.path().join("data.csv"), b"col1,col2").unwrap();

    let out_dir = TempDir::new().unwrap();
    let files = collect_image_files(
        &[dir.path().to_path_buf()],
        out_dir.path(),
        OperationMode::ConvertToWebP,
    );

    assert_eq!(files.len(), 1, "Only the PNG should be collected");
}

// 22. Resize-only mode sets FileAction::Resize in collect_image_files
#[test]
fn test_collect_resize_mode() {
    let dir = TempDir::new().unwrap();
    make_png(&dir, "img.png");
    let out_dir = TempDir::new().unwrap();

    let files = collect_image_files(
        &[dir.path().to_path_buf()],
        out_dir.path(),
        OperationMode::ResizeOnly,
    );

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].2, FileAction::Resize);
}

fn find_webp_files(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("webp"))
        .collect()
}

#[test]
fn test_batch_result_counts() {
    let dir = TempDir::new().unwrap();
    make_png(&dir, "a.png");
    make_jpeg(&dir, "b.jpg");

    let out_dir = TempDir::new().unwrap();
    let files = collect_image_files(
        &[dir.path().to_path_buf()],
        out_dir.path(),
        OperationMode::ConvertToWebP,
    );

    let result = process_batch(files, &default_opts());
    assert_eq!(result.total, 2);
    assert_eq!(result.converted, 2);
    assert_eq!(result.failed.len(), 0);

    let outputs = find_webp_files(out_dir.path());
    assert_eq!(outputs.len(), 2, "Two WebP files should be produced");
}
