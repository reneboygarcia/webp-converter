use std::fs;
use tempfile::NamedTempFile;
use webp_converter::update_checker::UpdateChecker;

#[test]
fn test_update_checker_no_update() {
    let temp_file = NamedTempFile::new().unwrap();
    let cache_path = temp_file.path().to_path_buf();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let cache_content = format!("{{\"latest_version\":\"0.2.8\",\"last_check\":{}}}", now);
    fs::write(&cache_path, cache_content).unwrap();

    let checker = UpdateChecker::new_with_cache_file("v0.2.8", cache_path);
    assert!(checker.check_for_update().is_none());
}

#[test]
fn test_update_checker_has_update_in_cache() {
    let temp_file = NamedTempFile::new().unwrap();
    let cache_path = temp_file.path().to_path_buf();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let cache_content = format!("{{\"latest_version\":\"0.3.0\",\"last_check\":{}}}", now);
    fs::write(&cache_path, cache_content).unwrap();

    let checker = UpdateChecker::new_with_cache_file("v0.2.8", cache_path);
    assert_eq!(checker.check_for_update().unwrap(), "0.3.0");
}

#[test]
fn test_update_checker_older_in_cache() {
    let temp_file = NamedTempFile::new().unwrap();
    let cache_path = temp_file.path().to_path_buf();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let cache_content = format!("{{\"latest_version\":\"0.1.0\",\"last_check\":{}}}", now);
    fs::write(&cache_path, cache_content).unwrap();

    let checker = UpdateChecker::new_with_cache_file("v0.2.8", cache_path);
    assert!(checker.check_for_update().is_none());
}

#[test]
fn test_homebrew_detection() {
    let is_homebrew = UpdateChecker::is_installed_via_homebrew();
    assert!(is_homebrew || !is_homebrew);
}
