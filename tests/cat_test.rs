use std::io::Write;
use tempfile::NamedTempFile;
use winix::cat::{cat, cat_async_to_string}; // <- Adjust path if not in `lib.rs`

/// Create a temporary file with given content
fn create_temp_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    write!(file, "{}", content).expect("Failed to write content to temp file");
    file
}

#[test]
fn test_cat_sync() {
    let file = create_temp_file("hello world");
    let path = file.path();

    let result = cat(vec![path]);
    assert!(result.is_ok());
    assert!(result.unwrap().contains("hello world"));
}

#[tokio::test]
async fn test_cat_async_to_string() {
    let file = create_temp_file("async hello");
    let path = file.path();

    let result = cat_async_to_string(path).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().trim(), "async hello");
}
