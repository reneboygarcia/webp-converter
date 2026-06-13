use webp_converter::get_downloads_dir;

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
