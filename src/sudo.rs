#[cfg(target_family = "unix")]
use std::process::Command;

#[allow(dead_code)]
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

    #[cfg(target_os = "linux")]
    {
        let status = Command::new("sudo")
            .args(&args)
            .status()
            .expect("Failed to execute sudo command");

        std::process::exit(status.code().unwrap_or(1));
    }

    #[cfg(target_family = "windows")]
    {
        use std::process::Command;

        let command = &args[0];
        let cmd_args = &args[1..];

        let joined_args = cmd_args
            .iter()
            .map(|s| format!("'{}'", s))
            .collect::<Vec<String>>()
            .join(",");

        let status = Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                "Start-Process '{}' -ArgumentList {} -Verb runAs",
                command, joined_args
            ))
            .status()
            .expect("Failed to launch command as admin");

        std::process::exit(status.code().unwrap_or(1));
    }
}
