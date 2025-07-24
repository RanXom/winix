use colored::*;
use std::io::{self, Write};
use std::process::Command;

/// Execute PowerShell commands by shelling out to the system PowerShell
pub fn execute(args: &[&str]) {
    // Check if PowerShell is available
    if !is_powershell_available() {
        println!(
            "{}",
            "Error: PowerShell is not available on this system".red()
        );
        return;
    }

    // If no arguments provided, show PowerShell help
    if args.is_empty() {
        show_powershell_help();
        return;
    }

    // Execute the PowerShell command
    execute_powershell_command(args);
}

/// Check if PowerShell is available on the system
pub fn is_powershell_available() -> bool {
    // Try both pwsh (PowerShell Core) and powershell (Windows PowerShell)
    is_command_available("pwsh") || is_command_available("powershell")
}

/// Check if a specific PowerShell executable is available
fn is_command_available(cmd: &str) -> bool {
    match Command::new(cmd)
        .arg("-Command")
        .arg("$PSVersionTable.PSVersion")
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Get the preferred PowerShell executable
fn get_powershell_executable() -> &'static str {
    if is_command_available("pwsh") {
        "pwsh" // PowerShell Core (cross-platform)
    } else {
        "powershell" // Windows PowerShell
    }
}

/// Execute a PowerShell command with the provided arguments
fn execute_powershell_command(args: &[&str]) {
    let ps_exe = get_powershell_executable();
    let command_string = args.join(" ");

    let mut cmd = Command::new(ps_exe);
    cmd.args(&["-Command", &command_string]);

    // Execute the command and handle output
    match cmd.output() {
        Ok(output) => {
            // Print stdout
            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }

            // Print stderr
            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }

            // If the command failed, show the exit code
            if !output.status.success() {
                if let Some(code) = output.status.code() {
                    eprintln!(
                        "{}",
                        format!("PowerShell command failed with exit code: {}", code).red()
                    );
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}",
                format!("Failed to execute PowerShell command: {}", e).red()
            );
        }
    }
}

/// Show interactive PowerShell mode for complex operations
pub fn interactive_mode() {
    println!("{}", "PowerShell Interactive Mode".bold().blue());
    println!("{}", "Type PowerShell commands or 'exit' to quit".dimmed());
    println!(
        "{}",
        "Example: Get-Process, Get-ChildItem, Set-Location C:\\".dimmed()
    );
    println!();

    loop {
        // Show PowerShell prompt
        print!("{}", "PS> ".bold().blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                if input == "exit" || input == "quit" {
                    println!("{}", "Exiting PowerShell interactive mode".green());
                    break;
                }

                // Execute the PowerShell command
                execute_powershell_command(&[input]);
            }
            Err(e) => {
                eprintln!("{}", format!("Error reading input: {}", e).red());
                break;
            }
        }
    }
}

/// Show PowerShell help and common commands
fn show_powershell_help() {
    println!("{}", "PowerShell Commands Available".bold().blue());
    println!("{}", "Usage: ps <command> [options]".dimmed());
    println!();

    println!("{}", "Most Common PowerShell Commands:".bold().white());
    println!(
        "  {:<20} {}",
        "Get-Process".yellow(),
        "List running processes"
    );
    println!(
        "  {:<20} {}",
        "Get-ChildItem".yellow(),
        "List files and directories (like ls)"
    );
    println!(
        "  {:<20} {}",
        "Set-Location".yellow(),
        "Change directory (like cd)"
    );
    println!(
        "  {:<20} {}",
        "Get-Location".yellow(),
        "Get current directory (like pwd)"
    );
    println!(
        "  {:<20} {}",
        "Get-Content".yellow(),
        "Read file contents (like cat)"
    );
    println!(
        "  {:<20} {}",
        "Set-Content".yellow(),
        "Write content to file"
    );
    println!(
        "  {:<20} {}",
        "Copy-Item".yellow(),
        "Copy files or directories (like cp)"
    );
    println!(
        "  {:<20} {}",
        "Move-Item".yellow(),
        "Move files or directories (like mv)"
    );
    println!(
        "  {:<20} {}",
        "Remove-Item".yellow(),
        "Delete files or directories (like rm)"
    );
    println!(
        "  {:<20} {}",
        "New-Item".yellow(),
        "Create new files or directories"
    );
    println!(
        "  {:<20} {}",
        "Get-Service".yellow(),
        "List system services"
    );
    println!("  {:<20} {}", "Get-EventLog".yellow(), "Read event logs");
    println!("  {:<20} {}", "Get-WmiObject".yellow(), "Query WMI objects");
    println!(
        "  {:<20} {}",
        "Invoke-WebRequest".yellow(),
        "Make HTTP requests (like curl)"
    );
    println!(
        "  {:<20} {}",
        "Test-Connection".yellow(),
        "Ping hosts (like ping)"
    );
    println!();

    println!("{}", "Examples:".bold().cyan());
    println!(
        "  {}",
        "ps Get-Process | Where-Object {$_.CPU -gt 100}".dimmed()
    );
    println!(
        "  {}",
        "ps Get-ChildItem C:\\ -Recurse -Include *.txt".dimmed()
    );
    println!(
        "  {}",
        "ps Get-Service | Where-Object {$_.Status -eq 'Running'}".dimmed()
    );
    println!("  {}", "ps Test-Connection google.com -Count 4".dimmed());
    println!(
        "  {}",
        "ps Get-EventLog -LogName System -Newest 10".dimmed()
    );
    println!(
        "  {}",
        "ps Get-WmiObject -Class Win32_ComputerSystem".dimmed()
    );
    println!();

    println!("{}", "Interactive Mode:".bold().magenta());
    println!("  {}", "ps --interactive".dimmed());
    println!(
        "  {}",
        "  Enter interactive PowerShell mode for easier command execution".dimmed()
    );
    println!();

    println!("{}", "Aliases:".bold().green());
    println!(
        "  {}",
        "psh = ps (shorter alias for PowerShell commands)".dimmed()
    );
}

/// Get PowerShell version information
#[allow(dead_code)]
pub fn get_version_info() -> Option<String> {
    let ps_exe = get_powershell_executable();
    match Command::new(ps_exe)
        .args(&["-Command", "$PSVersionTable.PSVersion.ToString()"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !version.is_empty() {
                    Some(format!("{} {}", ps_exe, version))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Check if current directory is accessible via PowerShell
#[allow(dead_code)]
pub fn test_current_directory() -> bool {
    let ps_exe = get_powershell_executable();
    match Command::new(ps_exe)
        .args(&["-Command", "Get-Location"])
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
