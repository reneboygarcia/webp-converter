pub mod converter;
pub mod paths;
pub mod transform;
pub mod ui;

pub use converter::{
    collect_image_files, convert_to_webp, convert_to_webp_core, process_batch, resize_image,
    BatchResult, ConversionMetrics, ConversionOptions, ConverterError, FileAction, OperationMode,
};
pub use paths::get_downloads_dir;
pub use transform::{process_all_logos, transform_logo, TransformOptions};
pub use ui::{ask_overwrite, show_error, show_info, show_success, show_warning};
