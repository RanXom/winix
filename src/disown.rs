use std::env;
use std::process::Command;
use std::process::Stdio;

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: disown <command> [args...]");
        std::process::exit(1);
    }

    #[cfg(target_os = "macos")]
    {
        let mut command = Command::new("nohup");
        command
            .arg(&args[0])
            .args(&args[1..])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        match command.spawn() {
            Ok(_) => {
                println!("Process disowned (macOS)");
            }
            Err(e) => {
                eprintln!("Failed to disown process: {}", e);
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let mut command = Command::new("nohup");
        command
            .arg(&args[0])
            .args(&args[1..])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        match command.spawn() {
            Ok(_) => {
                println!("Process disowned (non-macOS)");
            }
            Err(e) => {
                eprintln!("Failed to disown process: {}", e);
            }
        }
    }
}

