use image::{DynamicImage, GenericImageView, RgbaImage};
use std::fs;
use std::path::{Path, PathBuf};

pub struct TransformOptions {
    pub target_size: u32,
    pub padding: u32,
}

impl Default for TransformOptions {
    fn default() -> Self {
        Self {
            target_size: 300,
            padding: 10,
        }
    }
}

static SUPPORTED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "bmp", "tiff", "tif"];

fn is_supported(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| SUPPORTED_EXTENSIONS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn transform_logo(
    image_path: &Path,
    output_dir: &Path,
    opts: &TransformOptions,
) -> Option<PathBuf> {
    if !image_path.exists() {
        return None;
    }

    let img = image::open(image_path).ok()?;
    let (orig_w, orig_h) = img.dimensions();

    if orig_w == 0 || orig_h == 0 {
        return None;
    }

    let canvas_size = opts.target_size + opts.padding * 2;
    let available = opts.target_size;

    let scale = (available as f32 / orig_w as f32).min(available as f32 / orig_h as f32);
    let new_w = (orig_w as f32 * scale).round() as u32;
    let new_h = (orig_h as f32 * scale).round() as u32;

    let resized = img
        .resize(new_w, new_h, image::imageops::FilterType::Lanczos3)
        .into_rgba8();

    let mut canvas = RgbaImage::from_pixel(canvas_size, canvas_size, image::Rgba([0, 0, 0, 0]));

    let x_offset = opts.padding + (available.saturating_sub(new_w)) / 2;
    let y_offset = opts.padding + (available.saturating_sub(new_h)) / 2;

    image::imageops::overlay(&mut canvas, &resized, x_offset as i64, y_offset as i64);

    if fs::create_dir_all(output_dir).is_err() {
        return None;
    }

    let stem = image_path.file_stem()?.to_str()?;
    let out_path = output_dir.join(format!("{}.png", stem));

    DynamicImage::ImageRgba8(canvas).save(&out_path).ok()?;

    Some(out_path)
}

pub fn process_all_logos(
    input_dir: &Path,
    output_dir: &Path,
    opts: &TransformOptions,
) -> Vec<PathBuf> {
    let entries = match fs::read_dir(input_dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && is_supported(p))
        .filter_map(|p| transform_logo(&p, output_dir, opts))
        .collect()
}
