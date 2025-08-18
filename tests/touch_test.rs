use std::fs;
use std::path::Path;
use winix::touch;

#[test]
fn test_touch_creates_file() {
    let filename = "test_temp_file.txt";

    // Cleanup before
    if Path::new(filename).exists() {
        fs::remove_file(filename).unwrap();
    }

    // Call touch::run directly
    touch::run(&vec![filename.to_string()]);

    // Check file exists
    assert!(Path::new(filename).exists(), "File was not created");

    // Cleanup after
    fs::remove_file(filename).unwrap();
}
