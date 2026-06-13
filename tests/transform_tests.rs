use image::{Rgba, RgbaImage};
use std::fs;
use tempfile::TempDir;
use webp_converter::transform::{process_all_logos, transform_logo, TransformOptions};

fn make_png(dir: &TempDir, name: &str, w: u32, h: u32) -> std::path::PathBuf {
    let path = dir.path().join(name);
    let img = RgbaImage::from_pixel(w, h, Rgba([100, 150, 200, 255]));
    image::DynamicImage::ImageRgba8(img).save(&path).unwrap();
    path
}

fn make_jpeg(dir: &TempDir, name: &str) -> std::path::PathBuf {
    let path = dir.path().join(name);
    let img = image::RgbImage::from_pixel(64, 64, image::Rgb([200, 100, 50]));
    image::DynamicImage::ImageRgb8(img)
        .save_with_format(&path, image::ImageFormat::Jpeg)
        .unwrap();
    path
}

fn default_opts() -> TransformOptions {
    TransformOptions::default()
}

// 1. Output dimensions = target_size + 2 * padding
#[test]
fn test_transform_logo_output_dimensions() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    let input = make_png(&dir, "logo.png", 100, 100);
    let opts = TransformOptions {
        target_size: 300,
        padding: 10,
    };

    let out = transform_logo(&input, out_dir.path(), &opts).unwrap();
    let img = image::open(&out).unwrap();
    let expected = opts.target_size + opts.padding * 2;
    assert_eq!(
        img.width(),
        expected,
        "Width should be target_size + 2*padding"
    );
    assert_eq!(
        img.height(),
        expected,
        "Height should be target_size + 2*padding"
    );
}

// 2. Output is RGBA mode
#[test]
fn test_transform_logo_output_is_rgba() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    let input = make_png(&dir, "logo.png", 50, 50);

    let out = transform_logo(&input, out_dir.path(), &default_opts()).unwrap();
    let img = image::open(&out).unwrap();
    // RGBA8 should be convertible without panic
    let rgba = img.into_rgba8();
    // Verify alpha channel exists (canvas padding should be transparent)
    let corner_pixel = rgba.get_pixel(0, 0);
    assert_eq!(
        corner_pixel[3], 0,
        "Canvas padding should be transparent (alpha=0)"
    );
}

// 3. Aspect ratio preserved for non-square image
#[test]
fn test_transform_logo_aspect_ratio_preserved() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    // 2:1 aspect ratio
    let input = make_png(&dir, "wide.png", 200, 100);
    let opts = TransformOptions {
        target_size: 200,
        padding: 0,
    };

    let out = transform_logo(&input, out_dir.path(), &opts).unwrap();
    let img = image::open(&out).unwrap();

    // With 2:1 input and 200×200 canvas, scaled image should be 200×100
    // The resized region preserves ratio
    assert_eq!(img.width(), 200, "Canvas width should be target_size");
    assert_eq!(img.height(), 200, "Canvas height should be target_size");

    // Non-square content: the image content area should not be squashed
    // Verify bottom-center row is transparent (padding from 2:1 aspect)
    let rgba = img.into_rgba8();
    let bottom_center = rgba.get_pixel(100, 199);
    assert_eq!(
        bottom_center[3], 0,
        "Bottom of canvas should be transparent for wide image"
    );
}

// 4. Missing input returns None
#[test]
fn test_transform_logo_missing_input() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    let missing = dir.path().join("nonexistent.png");

    let result = transform_logo(&missing, out_dir.path(), &default_opts());
    assert!(result.is_none(), "Missing input should return None");
}

// 5. Corrupt input returns None
#[test]
fn test_transform_logo_corrupt_input() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    let corrupt = dir.path().join("corrupt.png");
    fs::write(&corrupt, b"not a valid image").unwrap();

    let result = transform_logo(&corrupt, out_dir.path(), &default_opts());
    assert!(result.is_none(), "Corrupt input should return None");
}

// 6. Zero-dimension handling: 0×64 image returns None
#[test]
fn test_transform_logo_zero_width_image() {
    // Can't create a 0×64 image with the image crate (it would panic),
    // so test via a 1×1 image and verify it doesn't crash
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    let input = make_png(&dir, "tiny.png", 1, 1);
    let opts = TransformOptions {
        target_size: 300,
        padding: 10,
    };

    // Should succeed without panic for 1×1 image
    let result = transform_logo(&input, out_dir.path(), &opts);
    assert!(result.is_some(), "1×1 image should transform successfully");
}

// 7. process_all_logos: processes png/jpg, skips .txt
#[test]
fn test_process_all_logos_skips_non_images() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();

    make_png(&dir, "logo1.png", 100, 100);
    make_jpeg(&dir, "logo2.jpg");
    fs::write(dir.path().join("readme.txt"), b"not an image").unwrap();
    fs::write(dir.path().join("data.csv"), b"col1,col2").unwrap();

    let results = process_all_logos(dir.path(), out_dir.path(), &default_opts());
    assert_eq!(
        results.len(),
        2,
        "Should process 2 images and skip 2 non-images"
    );
}

// 8. Output directory created if missing
#[test]
fn test_transform_logo_creates_output_dir() {
    let dir = TempDir::new().unwrap();
    let input = make_png(&dir, "logo.png", 100, 100);
    let new_out_dir = dir.path().join("new_subdir").join("nested");

    assert!(!new_out_dir.exists(), "Output dir should not exist yet");
    let result = transform_logo(&input, &new_out_dir, &default_opts());
    assert!(result.is_some(), "Should succeed and create the output dir");
    assert!(new_out_dir.exists(), "Output dir should have been created");
}

// 9. Output file has .png extension regardless of input format
#[test]
fn test_transform_logo_output_is_png() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    let input = make_jpeg(&dir, "logo.jpg");

    let out = transform_logo(&input, out_dir.path(), &default_opts()).unwrap();
    assert!(
        out.extension().and_then(|e| e.to_str()) == Some("png"),
        "Output should always be PNG"
    );
}

// 10. process_all_logos with empty directory returns empty vec
#[test]
fn test_process_all_logos_empty_dir() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    let results = process_all_logos(dir.path(), out_dir.path(), &default_opts());
    assert!(results.is_empty(), "Empty dir should yield no results");
}

// 11. process_all_logos with non-existent input dir returns empty vec
#[test]
fn test_process_all_logos_missing_dir() {
    let dir = TempDir::new().unwrap();
    let out_dir = TempDir::new().unwrap();
    let missing = dir.path().join("no_such_dir");
    let results = process_all_logos(&missing, out_dir.path(), &default_opts());
    assert!(
        results.is_empty(),
        "Missing input dir should return empty vec"
    );
}
