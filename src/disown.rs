use std::env;
use std::process::{Command, Stdio};

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: disown <command> [args...]");
        std::process::exit(1);
    }

    // For Unix-based systems (macOS, Linux)
    #[cfg(unix)]
    {
        let mut command = Command::new("nohup");
        command
            .arg(&args[0])
            .args(&args[1..])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        match command.spawn() {
            Ok(_) => println!("Process disowned (Unix-based OS)"),
            Err(e) => eprintln!("Failed to disown process: {}", e),
        }
    }

    // For Windows
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x00000008;

        let mut command = Command::new(&args[0]);
        command
            .args(&args[1..])
            .creation_flags(DETACHED_PROCESS)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        match command.spawn() {
            Ok(_) => println!("Process disowned (Windows)"),
            Err(e) => eprintln!("Failed to disown process: {}", e),
        }
    }
}
