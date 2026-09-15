use std::path::PathBuf;

/// Get default Downloads directory for current user.
pub fn get_downloads_dir() -> PathBuf {
    dirs::download_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Downloads")
    })
}

/// Clean and normalize a single path string.
/// Handles surrounding quotes (single or double), shell-escaped spaces/characters (\ ),
/// leading/trailing whitespace, and tilde (~) expansion.
pub fn clean_path(raw: &str) -> PathBuf {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return PathBuf::new();
    }

    // 1. If raw input already exists on disk as-is, return it directly.
    let direct_path = PathBuf::from(trimmed);
    if direct_path.exists() {
        return direct_path;
    }

    // 2. Strip surrounding matching or unmatching quotes (' and ")
    let unquoted = strip_quotes(trimmed);

    // 3. Unescape backslashes before spaces and special characters
    let unescaped = unescape_path(&unquoted);

    // 4. Expand tilde (~) to home directory if applicable
    let expanded = expand_tilde(&unescaped);

    PathBuf::from(expanded)
}

fn strip_quotes(s: &str) -> String {
    let mut current = s.trim();
    loop {
        let prev = current;
        current = current.trim();
        if (current.starts_with('"') && current.ends_with('"') && current.len() >= 2)
            || (current.starts_with('\'') && current.ends_with('\'') && current.len() >= 2)
        {
            current = &current[1..current.len() - 1];
        } else {
            if current.len() > 1 && (current.starts_with('"') || current.starts_with('\'')) {
                let first = current.chars().next().unwrap();
                if !current[1..].contains(first) {
                    current = &current[1..];
                }
            }
            if current.len() > 1 && (current.ends_with('"') || current.ends_with('\'')) {
                let last = current.chars().last().unwrap();
                if !current[..current.len() - 1].contains(last) {
                    current = &current[..current.len() - 1];
                }
            }
        }
        current = current.trim();
        if current == prev {
            break;
        }
    }
    current.to_string()
}

fn unescape_path(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                if is_escapable_char(next_ch) {
                    result.push(next_ch);
                    chars.next();
                    continue;
                }
            }
        }
        result.push(ch);
    }
    result
}

fn is_escapable_char(ch: char) -> bool {
    #[cfg(windows)]
    {
        matches!(ch, ' ' | '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | '\\')
    }
    #[cfg(not(windows))]
    {
        !ch.is_alphanumeric()
            || matches!(
                ch,
                ' ' | '\''
                    | '"'
                    | '('
                    | ')'
                    | '['
                    | ']'
                    | '{'
                    | '}'
                    | '\\'
                    | '&'
                    | '$'
                    | '!'
                    | '#'
                    | '?'
                    | '*'
                    | '<'
                    | '>'
                    | '|'
                    | ';'
                    | '='
            )
    }
}

fn expand_tilde(s: &str) -> String {
    if s == "~" {
        if let Some(home) = dirs::home_dir() {
            return home.to_string_lossy().to_string();
        }
    } else if s.starts_with("~/") || s.starts_with("~\\") {
        if let Some(home) = dirs::home_dir() {
            let mut home_str = home.to_string_lossy().to_string();
            if !home_str.ends_with('/') && !home_str.ends_with('\\') {
                home_str.push('/');
            }
            return format!("{}{}", home_str, &s[2..]);
        }
    }
    s.to_string()
}

/// Parse a user input string into a list of cleaned PathBufs.
/// Supports comma-separated paths, newline-separated paths, space-separated quoted/escaped paths,
/// and single paths with spaces or quotes.
pub fn parse_input_paths(input: &str) -> Vec<PathBuf> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    // 1. If the whole input string (when cleaned) exists on disk, treat it as a single input path.
    let single_cleaned = clean_path(trimmed);
    if single_cleaned.exists() {
        return vec![single_cleaned];
    }

    // 2. Tokenize input based on delimiters
    let raw_tokens = tokenize_input(trimmed);

    // 3. Clean each token and filter out empty paths
    let mut paths = Vec::new();
    for token in raw_tokens {
        let cleaned = clean_path(&token);
        if !cleaned.as_os_str().is_empty() && !paths.contains(&cleaned) {
            paths.push(cleaned);
        }
    }

    // 4. Fallback: If tokenizing yielded non-existent paths, but single_cleaned could be a single path with spaces:
    if paths.is_empty() && !single_cleaned.as_os_str().is_empty() {
        paths.push(single_cleaned);
    } else if !paths.is_empty() && !has_explicit_delimiters(trimmed) {
        if !paths.iter().any(|p| p.exists()) && !single_cleaned.as_os_str().is_empty() {
            return vec![single_cleaned];
        }
    }

    paths
}

fn has_explicit_delimiters(s: &str) -> bool {
    s.contains(',') || s.contains('\n') || s.contains('"') || s.contains('\'') || s.contains('\\')
}

fn tokenize_input(input: &str) -> Vec<String> {
    if input.contains(',') {
        return input.split(',').map(|s| s.to_string()).collect();
    }
    if input.contains('\n') {
        return input.split('\n').map(|s| s.to_string()).collect();
    }

    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;

    for ch in input.chars() {
        if escaped {
            current.push('\\');
            current.push(ch);
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        match ch {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                current.push(ch);
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                current.push(ch);
            }
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                let t = current.trim();
                if !t.is_empty() {
                    tokens.push(t.to_string());
                }
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if escaped {
        current.push('\\');
    }

    let t = current.trim();
    if !t.is_empty() {
        tokens.push(t.to_string());
    }

    if tokens.is_empty() && !input.trim().is_empty() {
        tokens.push(input.trim().to_string());
    }

    tokens
}
