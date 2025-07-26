use std::process::Command;

#[test]
fn test_echo_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "echo", "Hello,", "Rust!"])
        .output()
        .expect("Failed to run echo");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, Rust!"));
}
