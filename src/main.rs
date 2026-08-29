use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use console::style;
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};
use std::io;
use std::path::PathBuf;
use webp_converter::{
    collect_image_files, convert_to_webp, get_downloads_dir, process_batch, show_batch_summary,
    show_detailed_log, show_error, show_goodbye, show_info, show_success, ConversionOptions,
    OperationMode, UpdateChecker, SKY,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const BANNER_LINES: &[&str] = &[
    "░██╗░░░░░░░██╗███████╗██████╗░██████╗░  ░█████╗░███╗░░██╗██╗░░░██╗████████╗██████╗░",
    "░██║░░██╗░░██║██╔════╝██╔══██╗██╔══██╗  ██╔══██╗████╗░██║██║░░░██║╚══██╔══╝██╔══██╗",
    "░╚██╗████╗██╔╝█████╗░░██████╦╝██████╔╝  ██║░░╚═╝██╔██╗██║╚██╗░██╔╝░░░██║░░░██████╔╝",
    "░░████╔═████║░██╔══╝░░██╔══██╗██╔═══╝░  ██║░░██╗██║╚████║░╚████╔╝░░░░██║░░░██╔══██╗",
    "░░╚██╔╝░╚██╔╝░███████╗██████╦╝██║░░░░░  ╚█████╔╝██║░╚███║░░╚██╔╝░░░░░██║░░░██║░░██║",
    "░░░╚═╝░░░╚═╝░░╚══════╝╚═════╝░╚═╝░░░░░  ░╚════╝░╚═╝░░╚══╝░░░╚═╝░░░░░░╚═╝░░░╚═╝░░╚═╝",
];
const BANNER_SPLIT: usize = 38;
const DIM_GRAY: u8 = 243;
const AMBER: u8 = 214;

fn print_banner() {
    println!();
    for line in BANNER_LINES {
        let split_byte = line
            .char_indices()
            .nth(BANNER_SPLIT)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        let (a, b) = line.split_at(split_byte);
        print!("{}", style(a).bold().color256(SKY));
        println!("{}", style(b).color256(SKY).dim());
    }
    println!(
        "  {}  {}",
        style("Fast WebP conversion").color256(SKY).dim(),
        style(format!("v{VERSION}")).color256(DIM_GRAY).dim(),
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

#[derive(Parser, Debug)]
#[command(
    name = "webp-convert",
    about = "Convert images to WebP format",
    version = VERSION,
    long_about = None
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

    /// Verbose output (show details of each converted file)
    #[arg(long, short)]
    verbose: bool,

    /// Check for updates and upgrade webp-converter
    #[arg(short = 'u', long)]
    update: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install webp-converter CLI shortcuts and Homebrew formula
    Install,
    /// Check for updates and upgrade webp-converter
    Update {
        /// Check for updates without performing upgrade
        #[arg(long)]
        check_only: bool,
    },
    /// Uninstall or remove webp-converter
    Uninstall,
    /// Uninstall or remove webp-converter (alias for uninstall)
    Delete,
    /// Generate shell completion scripts (zsh, bash, fish, powershell)
    Completions {
        /// Shell to generate completions for (zsh, bash, fish, powershell)
        shell: String,
    },
}

fn handle_update(check_only: bool) -> i32 {
    println!();
    println!(
        " {}",
        style("🔄 Checking for updates from GitHub...")
            .bold()
            .white()
    );

    let checker = UpdateChecker::new(VERSION);
    match checker.check_for_update_live() {
        Ok(Some(latest_version)) => {
            println!(
                "\n{} A new version is available! Current: {} -> Latest: {}",
                style("🔔 Notification:").bold().color256(AMBER),
                style(format!("v{}", VERSION)).dim(),
                style(format!("v{}", latest_version)).bold().color256(SKY)
            );

            if check_only {
                println!(
                    "   Run {} to upgrade!",
                    style("webp-conv update").bold().color256(SKY)
                );
                return 2;
            }

            if UpdateChecker::is_installed_via_homebrew() {
                println!(
                    "{}",
                    style("🚀 Upgrading webp-converter via Homebrew...")
                        .color256(SKY)
                        .bold()
                );
                match UpdateChecker::perform_brew_upgrade() {
                    Ok(_) => {
                        println!(
                            "{}",
                            style("✔ Successfully upgraded webp-converter!")
                                .color256(SKY)
                                .bold()
                        );
                        0
                    }
                    Err(e) => {
                        println!(
                            "{} Upgrading via Homebrew failed: {}",
                            style("❌").bold().red(),
                            e
                        );
                        println!(
                            "   Try running manually: {}",
                            style("brew update && brew upgrade reneboygarcia/homebrew-tap/webp-converter")
                                .bold()
                                .color256(SKY)
                        );
                        1
                    }
                }
            } else {
                println!(
                    "   To upgrade, run:\n   {}",
                    style("brew update && brew upgrade reneboygarcia/homebrew-tap/webp-converter")
                        .bold()
                        .color256(SKY)
                );
                2
            }
        }
        Ok(None) => {
            println!(
                "\n{} You are up to date! (Current version: {})",
                style("✔").bold().color256(SKY),
                style(format!("v{}", VERSION)).bold()
            );
            0
        }
        Err(e) => {
            println!(
                "\n{} Could not reach GitHub to check for updates: {}",
                style("⚠️").bold().red(),
                e
            );
            1
        }
    }
}

fn handle_install() -> i32 {
    println!();
    println!(
        " {}",
        style("📦 Installing / configuring webp-converter...")
            .bold()
            .white()
    );

    if UpdateChecker::is_installed_via_homebrew() {
        println!(
            "{}",
            style("✔ webp-converter is already installed via Homebrew!").color256(SKY)
        );
        println!(
            "   Use {} or {} in terminal.",
            style("webp-convert").bold().color256(SKY),
            style("webp-conv").bold().color256(SKY)
        );
        return 0;
    }

    println!(
        "{}",
        style("🚀 Triggering Homebrew installation...")
            .color256(SKY)
            .bold()
    );
    match UpdateChecker::perform_brew_install() {
        Ok(_) => {
            println!(
                "{}",
                style("✔ Successfully installed webp-converter!")
                    .color256(SKY)
                    .bold()
            );
            println!(
                "   Commands available: {} and {}",
                style("webp-convert").bold().color256(SKY),
                style("webp-conv").bold().color256(SKY)
            );
            0
        }
        Err(_) => {
            println!(
                "{}",
                style("ℹ To install via Homebrew manually, run:")
                    .color256(SKY)
            );
            println!(
                "   {}",
                style("brew install reneboygarcia/homebrew-tap/webp-converter")
                    .bold()
                    .color256(SKY)
            );
            0
        }
    }
}

fn handle_uninstall() -> i32 {
    println!();
    println!(
        " {}",
        style("🗑 Uninstalling webp-converter...")
            .bold()
            .white()
    );

    if UpdateChecker::is_installed_via_homebrew() {
        match UpdateChecker::perform_brew_uninstall() {
            Ok(_) => {
                println!(
                    "{}",
                    style("✔ Successfully uninstalled webp-converter via Homebrew!")
                        .color256(SKY)
                        .bold()
                );
                0
            }
            Err(e) => {
                println!(
                    "{} Uninstalling via Homebrew failed: {}",
                    style("❌").bold().red(),
                    e
                );
                1
            }
        }
    } else {
        println!(
            "{}",
            style("ℹ webp-converter was not installed via Homebrew.").color256(SKY)
        );
        println!(
            "   If installed via Cargo, run: {}",
            style("cargo uninstall webp-converter").bold().color256(SKY)
        );
        0
    }
}

fn generate_completions(shell_str: &str) -> i32 {
    let shell = match shell_str.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" | "pwsh" => Shell::PowerShell,
        _ => {
            eprintln!(
                "{} Unsupported shell '{}'. Supported: bash, zsh, fish, powershell",
                style("❌").bold().red(),
                shell_str
            );
            return 1;
        }
    };

    let mut cmd = Args::command();
    generate(shell, &mut cmd, "webp-conv", &mut io::stdout());
    0
}

fn main() -> Result<()> {
    inquire::set_global_render_config(make_render_config());

    let args = Args::parse();

    if args.update {
        let code = handle_update(false);
        std::process::exit(code);
    }

    if let Some(cmd) = args.command {
        let code = match cmd {
            Commands::Install => handle_install(),
            Commands::Update { check_only } => handle_update(check_only),
            Commands::Uninstall | Commands::Delete => handle_uninstall(),
            Commands::Completions { shell } => generate_completions(&shell),
        };
        std::process::exit(code);
    }

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
        if args.verbose {
            show_detailed_log(&result.details);
        }
        return Ok(());
    }

    // Interactive TUI mode
    run_interactive(args.verbose)
}

fn run_interactive(verbose: bool) -> Result<()> {
    print_banner();

    loop {
        let choice = inquire::Select::new(
            "What would you like to do?",
            vec![
                "Convert images to WebP",
                "Check for updates / Upgrade",
                "Install CLI / Homebrew setup",
                "Uninstall / Delete",
                "Show information",
                "Exit",
            ],
        )
        .with_help_message("↑↓ to move, enter to select")
        .prompt();

        match choice {
            Ok("Convert images to WebP") => {
                if let Err(e) = run_conversion_workflow(verbose) {
                    show_error(&e.to_string(), "Error");
                }
            }
            Ok("Check for updates / Upgrade") => {
                handle_update(false);
            }
            Ok("Install CLI / Homebrew setup") => {
                handle_install();
            }
            Ok("Uninstall / Delete") => {
                let confirm = inquire::Confirm::new("Are you sure you want to uninstall webp-converter?")
                    .with_default(false)
                    .prompt()
                    .unwrap_or(false);
                if confirm {
                    handle_uninstall();
                }
            }
            Ok("Show information") => {
                show_info(
                    &format!(
                        "webp-converter v{VERSION}\nConverts PNG/JPG/BMP/TIFF/GIF to WebP.\nGitHub: github.com/reneboygarcia/webp-converter\n\nCommands:\n  webp-conv install   Install / configure CLI\n  webp-conv update    Check & upgrade to latest version\n  webp-conv delete    Uninstall webp-converter"
                    ),
                    "About",
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

fn run_conversion_workflow(verbose: bool) -> Result<()> {
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
        let show_log = if verbose {
            true
        } else {
            inquire::Confirm::new("Show detailed conversion log?")
                .with_default(false)
                .prompt()
                .unwrap_or(false)
        };
        if show_log {
            show_detailed_log(&result.details);
        }
    }

    Ok(())
}

fn ask_overwrite_prompt(filename: &str) -> bool {
    inquire::Confirm::new(&format!("'{}' already exists. Overwrite?", filename))
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}
