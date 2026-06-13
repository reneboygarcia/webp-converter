use image::{DynamicImage, GenericImageView, ImageFormat};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum ConverterError {
    #[error("Input file not found: {0}")]
    InputNotFound(PathBuf),
    #[error("Unsupported image format: {0}")]
    UnsupportedFormat(String),
    #[error("Image error: {0}")]
    Decode(#[from] image::ImageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Read-only output directory: {0}")]
    ReadOnlyDir(PathBuf),
    #[error("Quality must be 0–100, got {0}")]
    InvalidQuality(u8),
}

#[derive(Clone, Debug)]
pub struct ConversionOptions {
    pub quality: u8,
    pub lossless: bool,
    pub force: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            quality: 80,
            lossless: false,
            force: false,
        }
    }
}

impl ConversionOptions {
    pub fn new(quality: u8, lossless: bool, force: bool) -> Result<Self, ConverterError> {
        if quality > 100 {
            return Err(ConverterError::InvalidQuality(quality));
        }
        Ok(Self {
            quality,
            lossless,
            force,
        })
    }
}

#[derive(Debug)]
pub struct ConversionMetrics {
    pub original_size: u64,
    pub new_size: u64,
    pub quality: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperationMode {
    ConvertToWebP,
    ResizeOnly,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileAction {
    Convert,
    Copy,
    Resize,
}

#[derive(Debug, Clone)]
pub struct FileConversionResult {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub action: FileAction,
    pub original_size: u64,
    pub new_size: u64,
    pub error: Option<String>,
}

pub struct BatchResult {
    pub total: usize,
    pub converted: usize,
    pub copied: usize,
    pub failed: Vec<(PathBuf, String)>,
    pub details: Vec<FileConversionResult>,
}

static SUPPORTED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "bmp", "tiff", "tif", "gif", "webp"];

fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| SUPPORTED_EXTENSIONS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn save_webp(
    img: DynamicImage,
    output_path: &Path,
    opts: &ConversionOptions,
) -> Result<(), ConverterError> {
    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let rgba = img.into_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), width, height);
    let encoded = if opts.lossless {
        encoder.encode_lossless()
    } else {
        encoder.encode(opts.quality as f32)
    };
    fs::write(output_path, &*encoded)?;
    Ok(())
}

pub fn convert_to_webp_core(
    input_path: &Path,
    output_path: &Path,
    opts: &ConversionOptions,
) -> Result<ConversionMetrics, ConverterError> {
    if !input_path.exists() {
        return Err(ConverterError::InputNotFound(input_path.to_path_buf()));
    }
    if opts.quality > 100 {
        return Err(ConverterError::InvalidQuality(opts.quality));
    }

    let original_size = fs::metadata(input_path)?.len();

    let img = image::open(input_path)?;
    save_webp(img, output_path, opts)?;

    let new_size = fs::metadata(output_path)?.len();
    Ok(ConversionMetrics {
        original_size,
        new_size,
        quality: opts.quality,
    })
}

pub fn convert_to_webp(
    input_path: &Path,
    output_path: Option<&Path>,
    opts: &ConversionOptions,
    ask_overwrite_fn: Option<&dyn Fn(&str) -> bool>,
) -> Result<bool, ConverterError> {
    if !input_path.exists() {
        return Err(ConverterError::InputNotFound(input_path.to_path_buf()));
    }

    let default_output;
    let out = match output_path {
        Some(p) => p,
        None => {
            default_output = input_path.with_extension("webp");
            default_output.as_path()
        }
    };

    if out.exists() && !opts.force {
        let filename = out.file_name().and_then(|n| n.to_str()).unwrap_or("file");
        let allowed = ask_overwrite_fn.map(|f| f(filename)).unwrap_or(false);
        if !allowed {
            return Ok(false);
        }
    }

    convert_to_webp_core(input_path, out, opts)?;
    Ok(true)
}

pub fn resize_image(input_path: &Path, output_path: &Path) -> Result<(), ConverterError> {
    if !input_path.exists() {
        return Err(ConverterError::InputNotFound(input_path.to_path_buf()));
    }

    let fmt = ImageFormat::from_path(output_path).map_err(|_| {
        ConverterError::UnsupportedFormat(
            output_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_string(),
        )
    })?;

    let img = image::open(input_path)?;
    let (w, h) = img.dimensions();
    let resized = img.resize(w, h, image::imageops::FilterType::Lanczos3);

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    resized.save_with_format(output_path, fmt)?;
    Ok(())
}

pub fn collect_image_files(
    inputs: &[PathBuf],
    output_dir: &Path,
    mode: OperationMode,
) -> Vec<(PathBuf, PathBuf, FileAction)> {
    let mut results = Vec::new();

    for input in inputs {
        if input.is_file() {
            if is_supported_image(input) {
                let (out, action) = derive_output(input, output_dir, mode, None);
                results.push((input.clone(), out, action));
            }
        } else if input.is_dir() {
            for entry in WalkDir::new(input)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path().to_path_buf();
                if path.is_file() && is_supported_image(&path) {
                    let rel = path.strip_prefix(input).unwrap_or(&path);
                    let (out, action) = derive_output(&path, output_dir, mode, Some((input, rel)));
                    results.push((path, out, action));
                }
            }
        }
    }

    results
}

fn derive_output(
    input: &Path,
    output_dir: &Path,
    mode: OperationMode,
    rel: Option<(&Path, &Path)>,
) -> (PathBuf, FileAction) {
    let is_webp = input
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase() == "webp")
        .unwrap_or(false);

    let filename = input.file_name().unwrap_or_default();

    let out_relative = match rel {
        Some((_, r)) => r.to_path_buf(),
        None => PathBuf::from(filename),
    };

    match mode {
        OperationMode::ConvertToWebP => {
            if is_webp {
                let out = output_dir.join(&out_relative);
                (out, FileAction::Copy)
            } else {
                let out = output_dir.join(out_relative.with_extension("webp"));
                (out, FileAction::Convert)
            }
        }
        OperationMode::ResizeOnly => {
            let out = output_dir.join(&out_relative);
            (out, FileAction::Resize)
        }
    }
}

pub fn process_batch(
    files: Vec<(PathBuf, PathBuf, FileAction)>,
    opts: &ConversionOptions,
) -> BatchResult {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let total = files.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    let converted = Arc::new(AtomicUsize::new(0));
    let copied = Arc::new(AtomicUsize::new(0));

    let results: Vec<FileConversionResult> = files
        .par_iter()
        .map(|(src, dst, action)| {
            let orig_size = fs::metadata(src).map(|m| m.len()).unwrap_or(0);
            let result = match action {
                FileAction::Convert => convert_to_webp_core(src, dst, opts)
                    .map(|m| m.new_size)
                    .map_err(|e| e.to_string()),
                FileAction::Copy => fs::copy(src, dst)
                    .map(|_| orig_size)
                    .map_err(|e| e.to_string()),
                FileAction::Resize => resize_image(src, dst)
                    .and_then(|_| {
                        fs::metadata(dst)
                            .map(|m| m.len())
                            .map_err(ConverterError::from)
                    })
                    .map_err(|e| e.to_string()),
            };
            pb.inc(1);
            let (new_size, error) = match result {
                Ok(size) => {
                    match action {
                        FileAction::Convert => {
                            converted.fetch_add(1, Ordering::Relaxed);
                        }
                        FileAction::Copy => {
                            copied.fetch_add(1, Ordering::Relaxed);
                        }
                        FileAction::Resize => {
                            converted.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    (size, None)
                }
                Err(e) => (0, Some(e)),
            };

            FileConversionResult {
                src: src.clone(),
                dst: dst.clone(),
                action: *action,
                original_size: orig_size,
                new_size,
                error,
            }
        })
        .collect();

    pb.finish_and_clear();

    let failed: Vec<(PathBuf, String)> = results
        .iter()
        .filter(|r| r.error.is_some())
        .map(|r| (r.src.clone(), r.error.as_ref().unwrap().clone()))
        .collect();

    BatchResult {
        total,
        converted: converted.load(Ordering::Relaxed),
        copied: copied.load(Ordering::Relaxed),
        failed,
        details: results,
    }
}
