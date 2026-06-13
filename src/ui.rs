use crate::converter::{BatchResult, ConversionMetrics, FileAction, FileConversionResult};
use console::{measure_text_width, style, Term};
use std::path::{Path, PathBuf};

const LABEL_WIDTH: usize = 16;
const MIN_BOX: usize = 44;
const MAX_BOX: usize = 84;

// Sky blue — #5fd7ff, matches docs/banner.png
pub const SKY: u8 = 81;

// ── Terminal helpers ─────────────────────────────────────────

fn term_cols() -> usize {
    Term::stdout().size().1 as usize
}

fn truncate_path(s: &str, max: usize) -> String {
    if measure_text_width(s) <= max {
        return s.to_string();
    }
    let chars: Vec<char> = s.chars().collect();
    let keep = max.saturating_sub(1);
    let start = chars.len().saturating_sub(keep);
    format!("…{}", chars[start..].iter().collect::<String>())
}

// ── Size formatting ──────────────────────────────────────────

pub fn fmt_size(bytes: u64) -> String {
    if bytes < 1_024 {
        format!("{} B", bytes)
    } else if bytes < 1_024 * 1_024 {
        format!("{:.1} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1_024.0 * 1_024.0))
    }
}

// ── Dynamic-width box drawing ────────────────────────────────

fn panel_width(contents: &[String]) -> usize {
    let max_term = term_cols().saturating_sub(2).max(MIN_BOX);
    let natural = contents
        .iter()
        .map(|s| measure_text_width(s) + 4)
        .max()
        .unwrap_or(MIN_BOX);
    natural.clamp(MIN_BOX, max_term.min(MAX_BOX))
}

fn box_top(title: &str, w: usize) -> String {
    let title_str = format!(" {} ", title);
    let inner = w - 2;
    let dashes = inner.saturating_sub(measure_text_width(&title_str));
    let left = dashes / 2;
    let right = dashes - left;
    format!("╭{}{}{}╮", "─".repeat(left), title_str, "─".repeat(right))
}

fn box_bottom(w: usize) -> String {
    format!("╰{}╯", "─".repeat(w - 2))
}

fn box_sep(w: usize) -> String {
    format!("├{}┤", "─".repeat(w - 2))
}

fn box_line(content: &str, w: usize) -> String {
    let inner = w - 2;
    let padded = format!(" {} ", content);
    let vis = measure_text_width(&padded);
    let pad = inner.saturating_sub(vis);
    format!("│{}{}│", padded, " ".repeat(pad))
}

/// Two-tone key/value line: dim sky label + bold white value, sky border.
fn print_kv(label: &str, value: &str, w: usize) {
    let inner = w - 2;
    let label_col = format!("{:<lw$}", label, lw = LABEL_WIDTH);
    let content_vis = 1 + LABEL_WIDTH + measure_text_width(value) + 1;
    let pad = inner.saturating_sub(content_vis);
    print!("{}", style("│").color256(SKY));
    print!(" {}", style(&label_col).color256(SKY));
    print!("{}", style(value).bold().white());
    println!("{}{}", " ".repeat(pad), style("│").color256(SKY));
}

// ── Public panel API ─────────────────────────────────────────

pub fn show_success(input_path: &Path, output_path: &Path, metrics: &ConversionMetrics) {
    let saved = metrics.original_size.saturating_sub(metrics.new_size);
    let pct = if metrics.original_size > 0 {
        100.0 - (metrics.new_size as f64 / metrics.original_size as f64 * 100.0)
    } else {
        0.0
    };

    let input_name = input_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| input_path.display().to_string());
    let output_name = output_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| output_path.display().to_string());
    let dest_raw = output_path
        .parent()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    let size_line = format!(
        "{} → {} (saved {:.1}%)",
        fmt_size(metrics.original_size),
        fmt_size(metrics.new_size),
        pct
    );
    let max_val_vis = [
        measure_text_width(&input_name),
        measure_text_width(&output_name),
        measure_text_width(&dest_raw),
        measure_text_width(&size_line),
    ]
    .into_iter()
    .max()
    .unwrap_or(0);

    let contents: Vec<String> = vec![format!(
        "{:<lw$}{}",
        "x",
        "x".repeat(max_val_vis),
        lw = LABEL_WIDTH
    )];
    let w = panel_width(&contents);

    let val_max = w - 4 - LABEL_WIDTH;
    let dest = truncate_path(&dest_raw, val_max);

    println!(
        "{}",
        style(box_top("✓ Conversion Successful!", w))
            .bold()
            .color256(SKY)
    );
    print_kv("Original", &input_name, w);
    print_kv("WebP", &output_name, w);
    print_kv("Destination", &dest, w);
    println!("{}", style(box_sep(w)).color256(SKY));
    print_kv("Original size", &fmt_size(metrics.original_size), w);
    print_kv("WebP size", &fmt_size(metrics.new_size), w);
    print_kv("Saved", &format!("{} ({:.1}%)", fmt_size(saved), pct), w);
    print_kv("Quality", &metrics.quality.to_string(), w);
    println!("{}", style(box_bottom(w)).color256(SKY));
}

pub fn show_batch_summary(result: &BatchResult) {
    let has_errors = !result.failed.is_empty();

    let fail_lines: Vec<String> = result
        .failed
        .iter()
        .map(|(path, err)| {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            format!("{}: {}", name, err)
        })
        .collect();

    let mut contents = vec![
        format!("Processed : {}", result.total),
        format!("Converted : {}", result.converted),
        format!("Failed    : {}", result.failed.len()),
    ];
    if result.copied > 0 {
        contents.push(format!("Copied    : {}", result.copied));
    }
    contents.extend(fail_lines.iter().cloned());

    let w = panel_width(&contents);

    let print_line = |s: String| {
        if has_errors {
            println!("{}", style(s).red());
        } else {
            println!("{}", style(s).color256(SKY));
        }
    };

    print_line(box_top("Batch Complete", w));
    print_line(box_line(&format!("Processed : {}", result.total), w));
    print_line(box_line(&format!("Converted : {}", result.converted), w));
    if result.copied > 0 {
        print_line(box_line(&format!("Copied    : {}", result.copied), w));
    }
    print_line(box_line(&format!("Failed    : {}", result.failed.len()), w));

    if has_errors {
        print_line(box_sep(w));
        for line in &fail_lines {
            print_line(box_line(line, w));
        }
    }

    print_line(box_bottom(w));
}

pub fn show_detailed_log(details: &[FileConversionResult]) {
    let title = "Detailed Conversion Log";
    let w = panel_width(
        &details
            .iter()
            .map(|item| {
                let rel_src = item
                    .src
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                format!(
                    "{}: {}",
                    rel_src,
                    item.error.as_deref().unwrap_or("success")
                )
            })
            .collect::<Vec<_>>(),
    );

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::new());
    let inner_w = w.saturating_sub(6); // leaves room for borders and spaces

    let has_errors = details.iter().any(|r| r.error.is_some());
    let print_border = |s: String| {
        if has_errors {
            println!("{}", style(s).red());
        } else {
            println!("{}", style(s).color256(SKY));
        }
    };

    print_border(box_top(title, w));

    for item in details {
        let rel_src = item.src.strip_prefix(&cwd).unwrap_or(&item.src);
        let src_name = rel_src.display().to_string();

        let (icon, prefix_len) = match &item.error {
            None => match item.action {
                FileAction::Convert => (style("✓").green().bold(), 2),
                FileAction::Copy => (style("→").color256(SKY).bold(), 2),
                FileAction::Resize => (style("⚙").yellow().bold(), 2),
            },
            Some(_) => (style("✗").red().bold(), 2),
        };

        let info_str = match &item.error {
            None => {
                let pct = if item.original_size > 0 {
                    100.0 - (item.new_size as f64 / item.original_size as f64 * 100.0)
                } else {
                    0.0
                };

                match item.action {
                    FileAction::Convert => {
                        format!(
                            " {} → {} (saved {:.1}%)",
                            fmt_size(item.original_size),
                            fmt_size(item.new_size),
                            pct
                        )
                    }
                    FileAction::Copy => {
                        format!(" (copied, {})", fmt_size(item.original_size))
                    }
                    FileAction::Resize => {
                        format!(
                            " {} → {} (resized)",
                            fmt_size(item.original_size),
                            fmt_size(item.new_size)
                        )
                    }
                }
            }
            Some(err) => {
                format!(" (failed: {})", err)
            }
        };

        // Calculate max filename length to fit in the box line
        let info_len = measure_text_width(&info_str);
        let max_filename_len = inner_w.saturating_sub(prefix_len + 1 + info_len);

        let display_name = if measure_text_width(&src_name) > max_filename_len {
            truncate_path(&src_name, max_filename_len)
        } else {
            src_name
        };

        let display_name_styled = if item.error.is_some() {
            style(display_name).dim()
        } else {
            style(display_name).white()
        };

        let line_content = format!("{} {} {}", icon, display_name_styled, info_str);
        let inner = w - 2;
        let padded = format!(" {} ", line_content);
        let vis = measure_text_width(&padded);
        let pad = inner.saturating_sub(vis);

        let border_char = if has_errors {
            style("│").red()
        } else {
            style("│").color256(SKY)
        };

        println!(
            "{}{}{}{}",
            border_char,
            padded,
            " ".repeat(pad),
            border_char
        );
    }

    print_border(box_bottom(w));
}

pub fn show_error(message: &str, title: &str) {
    let t = format!("❌  {}", title);
    let w = panel_width(&[message.to_string()]);
    println!("{}", style(box_top(&t, w)).bold().red());
    for line in message.lines() {
        println!("{}", style(box_line(line, w)).red());
    }
    println!("{}", style(box_bottom(w)).red());
}

pub fn show_warning(message: &str, title: &str) {
    let t = format!("⚠   {}", title);
    let w = panel_width(&[message.to_string()]);
    println!("{}", style(box_top(&t, w)).bold().yellow());
    for line in message.lines() {
        println!("{}", style(box_line(line, w)).yellow());
    }
    println!("{}", style(box_bottom(w)).yellow());
}

pub fn show_info(message: &str, title: &str) {
    let t = format!("ℹ   {}", title);
    let lines: Vec<String> = message.lines().map(|l| l.to_string()).collect();
    let w = panel_width(&lines);
    println!("{}", style(box_top(&t, w)).bold().color256(SKY));
    for line in &lines {
        println!("{}", style(box_line(line, w)).color256(SKY));
    }
    println!("{}", style(box_bottom(w)).color256(SKY));
}

pub fn show_goodbye() {
    println!("\n{}", style("✌️  Later!").bold().color256(SKY));
}

pub fn ask_overwrite(filename: &str) -> bool {
    show_warning(
        &format!("Output file '{}' already exists.", filename),
        "File Exists",
    );
    inquire::Confirm::new("Overwrite this file?")
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}
