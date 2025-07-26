use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_touch_creates_file() {
    let filename = "test_temp_file.txt";

    // Cleanup before
    if Path::new(filename).exists() {
        fs::remove_file(filename).unwrap();
    }

    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "touch", filename])
        .output()
        .expect("Failed to run touch");

    assert!(Path::new(filename).exists(), "File was not created");

    // Cleanup after
    fs::remove_file(filename).unwrap();
}
