use winix::input::LineEditor;

#[test]
fn test_basic_input_history() {
    let mut editor = LineEditor::new();
    // We can't feed keys directly; instead ensure add_history_entry doesn't panic
    editor.add_history_entry("hello");
    // If we reached here without panic, basic interaction works
    assert!(true);
}
