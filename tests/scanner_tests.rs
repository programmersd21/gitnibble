use gitnibble::scanner::{DetectedRule, WorkspaceScanner};
use std::fs::{self, File};

#[test]
fn scan_detects_rust_project() {
    let dir = std::env::temp_dir().join("gitnibble_test_rust");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let cargo_path = dir.join("Cargo.toml");
    File::create(cargo_path).unwrap();

    let rules = WorkspaceScanner::scan(&dir);
    assert!(rules.contains(&DetectedRule::Language("Rust")));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn scan_detects_multiple_rules_and_ignores_skip_dirs() {
    let dir = std::env::temp_dir().join("gitnibble_test_multi");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    File::create(dir.join("Cargo.toml")).unwrap();
    File::create(dir.join("package.json")).unwrap();

    // Create a skipped dir containing a dummy file that shouldn't trigger anything else if skip works
    let target_dir = dir.join("target");
    fs::create_dir_all(&target_dir).unwrap();
    File::create(target_dir.join("go.mod")).unwrap(); // should NOT trigger Go

    let rules = WorkspaceScanner::scan(&dir);
    assert!(rules.contains(&DetectedRule::Language("Rust")));
    assert!(rules.contains(&DetectedRule::Language("Node")));
    assert!(!rules.contains(&DetectedRule::Language("Go")));

    let _ = fs::remove_dir_all(&dir);
}
