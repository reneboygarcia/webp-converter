use anyhow::Result;
use clap::Parser;
use console::style;
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};
use std::path::PathBuf;
use webp_converter::{
    collect_image_files, convert_to_webp, get_downloads_dir, process_batch, show_batch_summary,
    show_error, show_goodbye, show_info, show_success, ConversionOptions, OperationMode, SKY,
};

const BANNER_LINES: &[&str] = &[
    " ██╗    ██╗███████╗██████╗ ██████╗      ██████╗ ██████╗ ███╗   ██╗██╗   ██╗",
    " ██║    ██║██╔════╝██╔══██╗██╔══██╗    ██╔════╝██╔═══██╗████╗  ██║██║   ██║",
    " ██║ █╗ ██║█████╗  ██████╔╝██████╔╝    ██║     ██║   ██║██╔██╗ ██║██║   ██║",
    " ██║███╗██║██╔══╝  ██╔══██╗██╔═══╝     ██║     ██║   ██║██║╚██╗██║╚██╗ ██╔╝",
    " ╚███╔███╔╝███████╗██████╔╝██║         ╚██████╗╚██████╔╝██║ ╚████║ ╚████╔╝ ",
    "  ╚══╝╚══╝ ╚══════╝╚═════╝ ╚═╝          ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝  ╚═══╝ ",
];

// Dim gray — for banner second half and help text
const DIM_GRAY: u8 = 243;
// Amber — for answers / highlights
const AMBER: u8 = 214;

fn print_banner() {
    println!();
    for line in BANNER_LINES {
        // Split at char 30: first half gets mint, second half gets dim gray
        let split = std::cmp::min(30, line.len());
        let (a, b) = line.split_at(split);
        print!("{}", style(a).bold().color256(SKY));
        println!("{}", style(b).color256(DIM_GRAY));
    }
    let version = env!("CARGO_PKG_VERSION");
    println!(
        "  {} {}",
        style("Fast WebP conversion — Powered by Rust").color256(SKY).dim(),
        style(format!("v{version}")).color256(DIM_GRAY).dim(),
    );
    println!();
}

fn make_render_config() -> RenderConfig<'static> {
    RenderConfig {
        prompt_prefix: Styled::new("?").with_fg(Color::AnsiValue(SKY)),
        answered_prompt_prefix: Styled::new("✔").with_fg(Color::AnsiValue(SKY)),
        highlighted_option_prefix: Styled::new(">").with_fg(Color::AnsiValue(SKY)),
        selected_option: Some(StyleSheet::new().with_fg(Color::AnsiValue(SKY))),
        answer: StyleSheet::new().with_fg(Color::AnsiValue(AMBER)),
        help_message: StyleSheet::new().with_fg(Color::AnsiValue(DIM_GRAY)),
        default_value: StyleSheet::new().with_fg(Color::AnsiValue(DIM_GRAY)),
        placeholder: StyleSheet::new().with_fg(Color::AnsiValue(DIM_GRAY)),
        ..RenderConfig::default()
    }
}

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
    inquire::set_global_render_config(make_render_config());

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
    print_banner();

    loop {
        let choice = inquire::Select::new(
            "What would you like to do?",
            vec!["Convert images to WebP", "Show information", "Exit"],
        )
        .with_help_message("↑↓ to move, enter to select")
        .prompt();

        match choice {
            Ok("Convert images to WebP") => {
                if let Err(e) = run_conversion_workflow() {
                    show_error(&e.to_string(), "Error");
                }
            }
            Ok("Show information") => {
                let version = env!("CARGO_PKG_VERSION");
                show_info(
                    &format!(
                        "webp-converter v{version}\nRewritten in Rust for speed and efficiency.\nConverts PNG/JPG/BMP/TIFF/GIF to WebP.\nGitHub: github.com/reneboygarcia/webp-converter"
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

    let default_out = get_downloads_dir().to_string_lossy().to_string();
    let output_str = inquire::Text::new("Output directory:")
        .with_default(&default_out)
        .prompt()?;
    let output_dir = PathBuf::from(output_str.trim());

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
        "\n{}  {} files...\n",
        style("Processing").color256(SKY).bold(),
        style(files.len()).color256(AMBER).bold(),
    );

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
