use winix::input::{LineEditor, MyHelper};

#[test]
fn test_basic_input_simulation() {
    let mut editor = LineEditor::new();
    let input = editor.feed_input(vec!['h', 'e', 'l', 'l', 'o', '\n']);
    assert_eq!(input.trim(), "hello");
}
