use crate::converter::ConversionMetrics;
use console::style;
use std::path::Path;

const BOX_WIDTH: usize = 60;

fn box_line(content: &str) -> String {
    let padded = format!(" {} ", content);
    let padding = BOX_WIDTH.saturating_sub(padded.len() + 2);
    format!("│{}{}│", padded, " ".repeat(padding))
}

fn box_top(title: &str) -> String {
    let title_str = format!(" {} ", title);
    let left = (BOX_WIDTH.saturating_sub(title_str.len())) / 2;
    let right = BOX_WIDTH.saturating_sub(title_str.len() + left);
    format!("╭{}{}{}╮", "─".repeat(left), title_str, "─".repeat(right))
}

fn box_bottom() -> String {
    format!("╰{}╯", "─".repeat(BOX_WIDTH))
}

fn box_separator() -> String {
    format!("├{}┤", "─".repeat(BOX_WIDTH))
}

pub fn show_success(input_path: &Path, output_path: &Path, metrics: &ConversionMetrics) {
    let reduction = if metrics.original_size > 0 {
        let pct = 100.0 - (metrics.new_size as f64 / metrics.original_size as f64 * 100.0);
        format!("{:.1}%", pct)
    } else {
        "N/A".to_string()
    };

    println!("{}", style(box_top("Conversion Complete")).green());
    println!(
        "{}",
        style(box_line(&format!("Input:    {}", input_path.display()))).green()
    );
    println!(
        "{}",
        style(box_line(&format!("Output:   {}", output_path.display()))).green()
    );
    println!("{}", style(box_separator()).green());
    println!(
        "{}",
        style(box_line(&format!(
            "Original: {:.1} KB",
            metrics.original_size as f64 / 1024.0
        )))
        .green()
    );
    println!(
        "{}",
        style(box_line(&format!(
            "New size: {:.1} KB  (saved {})",
            metrics.new_size as f64 / 1024.0,
            reduction
        )))
        .green()
    );
    println!(
        "{}",
        style(box_line(&format!("Quality:  {}", metrics.quality))).green()
    );
    println!("{}", style(box_bottom()).green());
}

pub fn show_error(message: &str, title: &str) {
    println!("{}", style(box_top(title)).red());
    println!("{}", style(box_line(message)).red());
    println!("{}", style(box_bottom()).red());
}

pub fn show_warning(message: &str, title: &str) {
    println!("{}", style(box_top(title)).yellow());
    println!("{}", style(box_line(message)).yellow());
    println!("{}", style(box_bottom()).yellow());
}

pub fn show_info(message: &str, title: &str) {
    println!("{}", style(box_top(title)).cyan());
    println!("{}", style(box_line(message)).cyan());
    println!("{}", style(box_bottom()).cyan());
}

pub fn ask_overwrite(filename: &str) -> bool {
    inquire::Confirm::new(&format!("'{}' already exists. Overwrite?", filename))
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}
