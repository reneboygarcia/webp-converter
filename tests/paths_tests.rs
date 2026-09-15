use std::path::PathBuf;
use webp_converter::{clean_path, get_downloads_dir, parse_input_paths};

#[test]
fn test_downloads_dir_non_empty() {
    let path = get_downloads_dir();
    assert!(
        !path.as_os_str().is_empty(),
        "Downloads dir path should not be empty"
    );
}

#[test]
fn test_downloads_dir_contains_downloads() {
    let path = get_downloads_dir();
    let path_str = path.to_string_lossy().to_lowercase();
    assert!(
        path_str.contains("download"),
        "Path '{}' should contain 'download'",
        path_str
    );
}

#[test]
fn test_downloads_dir_is_absolute() {
    let path = get_downloads_dir();
    assert!(
        path.is_absolute(),
        "Downloads dir should be an absolute path, got: {:?}",
        path
    );
}

#[test]
fn test_clean_path_spaces_and_quotes() {
    assert_eq!(
        clean_path("  '/path/to/My Image.png'  "),
        PathBuf::from("/path/to/My Image.png")
    );
    assert_eq!(
        clean_path("\"/path/to/My Image.png\""),
        PathBuf::from("/path/to/My Image.png")
    );
    assert_eq!(
        clean_path("'/path/to/My Image.png'"),
        PathBuf::from("/path/to/My Image.png")
    );
}

#[test]
fn test_clean_path_escaped_characters() {
    assert_eq!(
        clean_path("/Users/rene/Desktop/My\\ Image\\ (1).png"),
        PathBuf::from("/Users/rene/Desktop/My Image (1).png")
    );
    assert_eq!(
        clean_path("/path/to/file\\ with\\ spaces.jpg"),
        PathBuf::from("/path/to/file with spaces.jpg")
    );
}

#[test]
fn test_clean_path_tilde_expansion() {
    let cleaned = clean_path("~/Desktop/My Image.png");
    let home = dirs::home_dir().unwrap_or_default();
    assert_eq!(cleaned, home.join("Desktop/My Image.png"));
}

#[test]
fn test_parse_input_paths_single_path_with_spaces() {
    let input = "/Users/rene/Pictures/Vacation 2026/beach.jpg";
    let paths = parse_input_paths(input);
    assert_eq!(paths.len(), 1);
    assert_eq!(
        paths[0],
        PathBuf::from("/Users/rene/Pictures/Vacation 2026/beach.jpg")
    );
}

#[test]
fn test_parse_input_paths_quoted_space_separated() {
    let input = "'/path/to/file 1.png' '/path/to/file 2.png'";
    let paths = parse_input_paths(input);
    assert_eq!(paths.len(), 2);
    assert_eq!(paths[0], PathBuf::from("/path/to/file 1.png"));
    assert_eq!(paths[1], PathBuf::from("/path/to/file 2.png"));
}

#[test]
fn test_parse_input_paths_comma_separated() {
    let input = "\"/path/to/file 1.png\", \"/path/to/file 2.png\"";
    let paths = parse_input_paths(input);
    assert_eq!(paths.len(), 2);
    assert_eq!(paths[0], PathBuf::from("/path/to/file 1.png"));
    assert_eq!(paths[1], PathBuf::from("/path/to/file 2.png"));
}

#[test]
fn test_parse_input_paths_escaped_spaces() {
    let input = "/path/to/file\\ 1.png /path/to/file\\ 2.png";
    let paths = parse_input_paths(input);
    assert_eq!(paths.len(), 2);
    assert_eq!(paths[0], PathBuf::from("/path/to/file 1.png"));
    assert_eq!(paths[1], PathBuf::from("/path/to/file 2.png"));
}
