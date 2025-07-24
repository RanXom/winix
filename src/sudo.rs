#[cfg(target_family = "unix")]
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: sudo <command> [args]");
        std::process::exit(1);
    }

    #[cfg(target_os = "macos")]
    {
        let status = Command::new("sudo")
            .args(&args)
            .status()
            .expect("Failed to execute sudo command");

        std::process::exit(status.code().unwrap_or(1));
    }

    #[cfg(target_family = "windows")]
    {
        use std::os::windows::process::CommandExt;
        use std::process::Command;

        const SEE_MASK_NO_CONSOLE: u32 = 0x00008000;

        let mut full_command = String::new();
        for arg in &args {
            full_command.push_str(&format!("{} ", arg));
        }

        let _ = Command::new("cmd")
            .arg("/C")
            .arg("powershell Start-Process cmd -Verb runAs")
            .creation_flags(SEE_MASK_NO_CONSOLE)
            .status()
            .expect("Failed to launch command as admin");
    }
}
