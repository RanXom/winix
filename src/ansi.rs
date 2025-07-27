use regex::Regex;
use std::str;

#[derive(Debug, PartialEq)]
pub enum AnsiEvent {
    SetColor(String),
    ResetColor,
    MoveCursor(u16, u16),
    ClearLine,
    PrintText(String),
}

pub struct AnsiParser;

impl AnsiParser {
    pub fn parse(input: &[u8]) -> Vec<AnsiEvent> {
        let mut events = Vec::new();
        // Convert &[u8] to &str
        let input_str = match str::from_utf8(input) {
            Ok(s) => s,
            Err(_) => return vec![], // or handle invalid UTF-8 as needed
        };

        let re = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
        let mut last = 0;

        for m in re.find_iter(input_str) {
            if m.start() > last {
                events.push(AnsiEvent::PrintText(input_str[last..m.start()].to_string()));
            }

            let seq = m.as_str();
            match seq {
                "\x1b[0m" => events.push(AnsiEvent::ResetColor),
                "\x1b[31m" => events.push(AnsiEvent::SetColor("Red".to_string())),
                "\x1b[32m" => events.push(AnsiEvent::SetColor("Green".to_string())),
                "\x1b[K" => events.push(AnsiEvent::ClearLine),
                _ => { /* handle other sequences */ }
            }

            last = m.end();
        }

        if last < input_str.len() {
            events.push(AnsiEvent::PrintText(input_str[last..].to_string()));
        }

        events
    }
}
