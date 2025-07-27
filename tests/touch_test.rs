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

    // Run the binary with filename as argument
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", filename])
        .output()
        .expect("Failed to run touch");

    // Debug output in case of error
    if !output.status.success() {
        eprintln!(
            "STDOUT: {}\nSTDERR: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Check file exists
    assert!(Path::new(filename).exists(), "File was not created");

    // Cleanup after
    fs::remove_file(filename).unwrap();
}
