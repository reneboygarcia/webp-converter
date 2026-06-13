use anyhow::Result;
use clap::Parser;
use console::style;
use std::path::PathBuf;
use webp_converter::{
    collect_image_files, convert_to_webp, get_downloads_dir, process_batch, show_batch_summary,
    show_error, show_goodbye, show_info, show_success, ConversionOptions, OperationMode,
};

const BANNER: &str = r#"
 ██╗    ██╗███████╗██████╗ ██████╗      ██████╗ ██████╗ ███╗   ██╗██╗   ██╗
 ██║    ██║██╔════╝██╔══██╗██╔══██╗    ██╔════╝██╔═══██╗████╗  ██║██║   ██║
 ██║ █╗ ██║█████╗  ██████╔╝██████╔╝    ██║     ██║   ██║██╔██╗ ██║██║   ██║
 ██║███╗██║██╔══╝  ██╔══██╗██╔═══╝     ██║     ██║   ██║██║╚██╗██║╚██╗ ██╔╝
 ╚███╔███╔╝███████╗██████╔╝██║         ╚██████╗╚██████╔╝██║ ╚████║ ╚████╔╝
  ╚══╝╚══╝ ╚══════╝╚═════╝ ╚═╝          ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝  ╚═══╝
                    Fast WebP conversion — Powered by Rust
"#;

#[derive(Parser)]
#[command(
    name = "webp-convert",
    about = "Convert images to WebP format",
    version
)]
struct Args {
    /// Input file or directory (batch mode)
    #[arg(long, short)]
    input: Option<PathBuf>,

    /// Output directory (batch mode)
    #[arg(long, short)]
    output: Option<PathBuf>,

    /// Quality 0–100 (default 80)
    #[arg(long, short, default_value = "80")]
    quality: u8,

    /// Lossless encoding
    #[arg(long)]
    lossless: bool,

    /// Overwrite existing files without prompting
    #[arg(long, short)]
    force: bool,

    /// Resize only (preserve original format)
    #[arg(long)]
    resize_only: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Non-interactive batch mode when --input is supplied
    if let Some(input) = args.input {
        let output_dir = args.output.unwrap_or_else(get_downloads_dir);
        let opts = ConversionOptions::new(args.quality, args.lossless, args.force)?;
        let mode = if args.resize_only {
            OperationMode::ResizeOnly
        } else {
            OperationMode::ConvertToWebP
        };
        let files = collect_image_files(&[input], &output_dir, mode);
        if files.is_empty() {
            show_info("No supported image files found.", "Info");
            return Ok(());
        }
        let result = process_batch(files, &opts);
        show_batch_summary(&result);
        return Ok(());
    }

    // Interactive TUI mode
    run_interactive()
}

fn run_interactive() -> Result<()> {
    println!("{}", style(BANNER).cyan());

    loop {
        let choice = inquire::Select::new(
            "What would you like to do?",
            vec!["Convert images to WebP", "Show information", "Exit"],
        )
        .prompt();

        match choice {
            Ok("Convert images to WebP") => {
                if let Err(e) = run_conversion_workflow() {
                    show_error(&e.to_string(), "Error");
                }
            }
            Ok("Show information") => {
                show_info(
                    concat!(
                        "webp-converter v0.2.0\n",
                        "Rewritten in Rust for speed and efficiency.\n",
                        "Converts PNG/JPG/BMP/TIFF/GIF to WebP.\n",
                        "GitHub: github.com/reneboygarcia/webp-converter"
                    ),
                    "About webp-converter",
                );
            }
            Ok("Exit") | Err(_) => {
                show_goodbye();
                break;
            }
            _ => break,
        }
    }

    Ok(())
}

fn run_conversion_workflow() -> Result<()> {
    // Input path(s)
    let input_str =
        inquire::Text::new("Enter input path(s) — file or directory (comma-separated):")
            .with_help_message("Press ESC to cancel")
            .prompt()?;

    let inputs: Vec<PathBuf> = input_str
        .split(',')
        .map(|s| PathBuf::from(s.trim()))
        .filter(|p| !p.as_os_str().is_empty())
        .collect();

    if inputs.is_empty() {
        show_error("No input paths provided.", "Error");
        return Ok(());
    }

    let missing: Vec<_> = inputs.iter().filter(|p| !p.exists()).collect();
    if !missing.is_empty() {
        for p in &missing {
            show_error(&format!("Path not found: {}", p.display()), "Error");
        }
        return Ok(());
    }

    // Output directory
    let default_out = get_downloads_dir().to_string_lossy().to_string();
    let output_str = inquire::Text::new("Output directory:")
        .with_default(&default_out)
        .prompt()?;
    let output_dir = PathBuf::from(output_str.trim());

    // Operation mode
    let mode_str = inquire::Select::new(
        "Operation mode:",
        vec!["Convert to WebP", "Resize Only (retain original format)"],
    )
    .prompt()?;
    let mode = if mode_str == "Resize Only (retain original format)" {
        OperationMode::ResizeOnly
    } else {
        OperationMode::ConvertToWebP
    };

    let (lossless, quality) = if mode == OperationMode::ConvertToWebP {
        let encoding = inquire::Select::new("Encoding:", vec!["Lossy", "Lossless"]).prompt()?;
        let lossless = encoding == "Lossless";

        let quality = if !lossless {
            inquire::CustomType::<u8>::new("Quality (0–100):")
                .with_default(80)
                .with_validator(|q: &u8| {
                    if *q <= 100 {
                        Ok(inquire::validator::Validation::Valid)
                    } else {
                        Ok(inquire::validator::Validation::Invalid(
                            "Quality must be 0–100".into(),
                        ))
                    }
                })
                .prompt()?
        } else {
            80
        };
        (lossless, quality)
    } else {
        (false, 80)
    };

    let force = inquire::Confirm::new("Overwrite existing files without prompting?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);

    let opts = ConversionOptions::new(quality, lossless, force)?;

    let files = collect_image_files(&inputs, &output_dir, mode);
    if files.is_empty() {
        show_info("No supported image files found in the given paths.", "Info");
        return Ok(());
    }

    println!(
        "\n{} {} files...\n",
        style("Processing").cyan(),
        files.len()
    );

    // For single-file interactive, show detailed success panel
    if files.len() == 1 {
        let (src, dst, _action) = &files[0];
        match convert_to_webp(
            src,
            Some(dst),
            &opts,
            Some(&|filename| ask_overwrite_prompt(filename)),
        ) {
            Ok(true) => {
                if let Ok(metrics) = webp_converter::convert_to_webp_core(src, dst, &opts) {
                    show_success(src, dst, &metrics);
                }
            }
            Ok(false) => show_info("Conversion skipped.", "Skipped"),
            Err(e) => show_error(&e.to_string(), "Error"),
        }
    } else {
        let result = process_batch(files, &opts);
        show_batch_summary(&result);
    }

    Ok(())
}

fn ask_overwrite_prompt(filename: &str) -> bool {
    inquire::Confirm::new(&format!("'{}' already exists. Overwrite?", filename))
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}
