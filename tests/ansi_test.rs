use std::process::Command;
use winix::ansi::{AnsiParser, AnsiEvent}; // Adjust based on your module path

#[test]
fn test_ls_color_output_parses_color_events() {
    // Run a command that emits ANSI sequences
    let output = Command::new("echo")
        .arg("\x1b[31mHello\x1b[0m") // Red "Hello"
        .output()
        .expect("Failed to run echo");

    // Decode output
    let stdout = String::from_utf8_lossy(&output.stdout);

    // FIX: convert &Cow<str> to &[u8] using `.as_bytes()`
    let events = AnsiParser::parse(stdout.as_bytes());

    // Check if SetColor was detected
    let has_color_event = events.iter().any(|e| matches!(e, AnsiEvent::SetColor(_)));

    assert!(
        has_color_event,
        "Expected at least one SetColor event in ls output"
    );
}
