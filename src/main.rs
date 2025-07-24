use colored::*;
use std::env;
use std::fs;
use std::io::{self, Write};

mod cd;
mod chmod;
mod chown;
mod disown;
mod df;
mod free;
mod git;
mod kill;
mod powershell;
mod ps;
mod sensors;
mod sudo;
mod tui;
mod uname;
mod uptime;


fn main() {
    test_sudo();
    test_disown();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--cli" {
        // Run original command-line mode (optional fallback)
        show_splash_screen();
        command_loop();
    } else {
        // Run TUI mode by default
        if let Err(err) = tui::run_tui() {
            eprintln!("Error running TUI: {}", err);
            eprintln!("Falling back to CLI mode...");
            show_splash_screen();
            command_loop();
        }
    }
}

fn show_splash_screen() {
    println!(
        "{}",
        "██     ██ ██ ███    ██ ██ ██   ██ 
██     ██ ██ ████   ██ ██  ██ ██  
██  █  ██ ██ ██ ██  ██ ██   ███   
██ ███ ██ ██ ██  ██ ██ ██  ██ ██  
 ███ ███  ██ ██   ████ ██ ██   ██ 
                                  
                                  "
    );
    println!(
        "{}",
        "-----------Your Most Useful Linux Commands directly on Your Windows without WSL or a Linux Distro-------------"
            .bold()
            .blue()
    );
    println!();
    println!(
        "{}",
        "� RECOMMENDED: Launch the beautiful TUI interface with: winix --tui"
            .bold()
            .green()
    );
    println!(
        "{}",
        "   Experience all commands in a modern, responsive terminal interface!"
            .bold()
            .cyan()
    );
    println!();
    println!("{}", "Available Commands:".bold().white());
    println!(
        "  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
        "cd".bold().yellow(),
        "chmod".bold().yellow(),
        "chown".bold().yellow(),
        "df".bold().yellow(),
        "exit".bold().red(),
        "free".bold().yellow(),
        "git".bold().yellow(),
        "kill".bold().yellow(),
        "ls".bold().yellow(),
        "ps".bold().yellow(),
        "psh/powershell".bold().cyan(),
        "pwd".bold().yellow(),
        "sensors".bold().yellow(),
        "uptime".bold().yellow(),
        "uname".bold().yellow(),
    );
    println!();
}

fn command_loop() {
    // Cache git info to avoid repeated expensive operations
    let mut cached_git_info = String::new();
    let mut last_dir = String::new();
    let mut git_check_counter = 0;

    loop {
        // Show current directory in the prompt
        let cwd = env::current_dir().unwrap_or_else(|_| "?".into());
        let current_dir = cwd.display().to_string();

        // Only check git status every 10 iterations or when directory changes
        // This significantly improves performance
        if current_dir != last_dir || git_check_counter % 10 == 0 {
            cached_git_info = if git::is_git_repo() {
                if let Some(branch) = git::get_current_branch() {
                    // Skip the expensive status check for better performance
                    format!(" ({})", branch.magenta())
                } else {
                    " (git)".dimmed().to_string()
                }
            } else {
                String::new()
            };
            last_dir = current_dir.clone();
        }
        git_check_counter += 1;

        print!(
            "{} {}{}",
            "WX".bold().white(),
            current_dir.white(),
            cached_git_info
        );
        print!("{}", "> ".bold().green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Split the command into parts
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "exit" | "quit" => {
                println!("{}", "Goodbye!".bold().blue());
                print!(
                    "{}",
                    "Want to contribute to this !? \tCheck out at : "
                        .bold()
                        .white()
                );
                println!("{}", "https://github.com/0xsambit/winix".bold().green());
                break;
            }
            "chmod" => {
                if parts.len() < 2 {
                    println!(
                        "{}",
                        "Usage: chmod [OPTION]... MODE[,MODE]... FILE...".red()
                    );
                    println!("{}", "   or: chmod [OPTION]... OCTAL-MODE FILE...".red());
                    println!(
                        "{}",
                        "   or: chmod [OPTION]... --reference=RFILE FILE...".red()
                    );
                    println!();
                    println!("{}", "Examples:".yellow());
                    println!("  {}", "chmod 755 myfile.txt".dimmed());
                    println!("  {}", "chmod u+x script.sh".dimmed());
                    println!("  {}", "chmod -R 644 directory/".dimmed());
                    println!("  {}", "chmod a-w file.txt".dimmed());
                    println!("  {}", "chmod u=rwx,g=rx,o=r file.txt".dimmed());
                } else {
                    // Pass all arguments except the command itself
                    let args: Vec<&str> = parts[1..].to_vec();
                    chmod::execute(&args);
                }
            }

            "chown" => {
                if parts.len() < 3 {
                    println!("{}", "Usage: chown [OWNER][:[GROUP]] FILE...".red());
                    println!();
                    println!("{}", "Examples:".yellow());
                    println!("  {}", "chown user file.txt".dimmed());
                    println!("  {}", "chown user:group file.txt".dimmed());
                    println!("  {}", "chown :group file.txt".dimmed());
                } else {
                    // Pass all arguments except the command itself
                    let args: Vec<&str> = parts[1..].to_vec();
                    chown::execute(&args);
                }
            }
            "uname" => {
                uname::execute();
            }
            "ps" => {
                ps::execute();
            }
            "sensors" => {
                sensors::execute();
            }
            "free" => {
                free::execute();
            }
            "uptime" => {
                uptime::execute();
            }
            "df" => {
                df::execute();
            }
            "git" => {
                if parts.len() == 1 {
                    // Show git help if no arguments
                    git::execute(&[]);
                } else if parts.len() == 2 && parts[1] == "--interactive" {
                    // Enter interactive git mode
                    git::interactive_mode();
                } else {
                    // Pass all arguments except the command itself
                    let args: Vec<&str> = parts[1..].to_vec();
                    git::execute(&args);
                }
            }
            "kill" => {
                if parts.len() < 2 {
                    println!("{}", "Usage: kill [-signal|-s signal|-p] [-q value] [-a] [--timeout milliseconds signal] [--] pid|name...".red());
                    println!();
                    println!("{}", "Supported Windows signals:".yellow());
                    println!("  {}", "-2, -INT    Interrupt (Ctrl+C)".dimmed());
                    println!("  {}", "-3, -QUIT   Quit (Ctrl+Break)".dimmed());
                    println!("  {}", "-9, -KILL   Force terminate (default)".dimmed());
                    println!("  {}", "-15, -TERM  Graceful terminate".dimmed());
                    println!();
                    println!("{}", "Examples:".yellow());
                    println!("  {}", "kill 1234".dimmed());
                    println!("  {}", "kill -TERM 1234".dimmed());
                    println!("  {}", "kill -9 1234".dimmed());
                    println!("  {}", "kill -a notepad".dimmed());
                } else {
                    // Pass all arguments except the command itself
                    let args: Vec<&str> = parts[1..].to_vec();
                    match kill::execute(&args) {
                        Ok(_) => {}
                        Err(e) => println!("{}", format!("kill: {}", e).red()),
                    }
                }
            }
            "psh" | "powershell" => {
                if parts.len() == 1 {
                    // Show PowerShell help if no arguments
                    powershell::execute(&[]);
                } else if parts.len() == 2 && parts[1] == "--interactive" {
                    // Enter interactive PowerShell mode
                    powershell::interactive_mode();
                } else {
                    // Pass all arguments except the command itself
                    let args: Vec<&str> = parts[1..].to_vec();
                    powershell::execute(&args);
                }
            }
            "cd" => {
                if parts.len() < 2 {
                    println!("{}", "Usage: cd <directory>".red());
                } else {
                    match cd_command(parts[1]) {
                        Ok(_) => {}
                        Err(e) => println!("{}", format!("cd: {}", e).red()),
                    }
                }
            }
            "pwd" => match pwd_command() {
                Ok(_) => {}
                Err(e) => println!("{}", format!("pwd: {}", e).red()),
            },
            "ls" => {
                let dir = if parts.len() > 1 { parts[1] } else { "." };
                match ls_command(dir) {
                    Ok(_) => {}
                    Err(e) => println!("{}", format!("ls: {}", e).red()),
                }
            }
            "help" => {
                println!("{}", "Available Commands:".bold().white());
                println!(
                    "  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
                    "cd".bold().yellow(),
                    "chmod".bold().yellow(),
                    "chown".bold().yellow(),
                    "df".bold().yellow(),
                    "exit".bold().red(),
                    "free".bold().yellow(),
                    "git".bold().yellow(),
                    "kill".bold().yellow(),
                    "ls".bold().yellow(),
                    "ps".bold().yellow(),
                    "psh/powershell".bold().cyan(),
                    "pwd".bold().yellow(),
                    "sensors".bold().yellow(),
                    "uptime".bold().yellow(),
                    "uname".bold().yellow(),
                );
            }
            _ => {
                println!("{}", format!("Unknown command: '{}'", command).red());
                println!("{}", "Type 'help' for available commands".dimmed());
            }
        }
    }
}

// --- CD, PWD, and LS commands ---

fn cd_command(path: &str) -> std::io::Result<()> {
    std::env::set_current_dir(path)
}

fn pwd_command() -> std::io::Result<()> {
    let cwd = std::env::current_dir()?;
    println!("{}", cwd.display().to_string().bold().cyan());
    Ok(())
}

fn ls_command(path: &str) -> std::io::Result<()> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if entry.file_type()?.is_dir() {
            println!("{}", file_name.blue().bold());
        } else {
            println!("{}", file_name.white());
        }
    }
    Ok(())
}



fn test_sudo() {
    println!("Running sudo...");
}

fn test_disown() {
    println!("Running disown...");
}