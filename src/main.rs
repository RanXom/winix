use colored::*;
use std::env;
use std::fs;
use std::io::{self, Write};

mod cd;
mod chmod;
mod chown;
mod df;
mod free;
mod ps;
mod sensors;
mod tui;
mod uname;
mod uptime;

fn main() {
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
        "  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
        "chmod".bold().yellow(),
        "chown".bold().yellow(),
        "uname".bold().yellow(),
        "ps".bold().yellow(),
        "sensors".bold().yellow(),
        "free".bold().yellow(),
        "uptime".bold().yellow(),
        "df".bold().yellow(),
        "cd".bold().yellow(),
        "pwd".bold().yellow(),
        "ls".bold().yellow(),
        "exit".bold().red()
    );
    println!();
}

fn command_loop() {
    loop {
        // Show current directory in the prompt
        let cwd = env::current_dir().unwrap_or_else(|_| "?".into());
        print!(
            "{} {} ",
            "WX".bold().white(),
            format!("{}", cwd.display()).white()
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
                    "  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
                    "chmod <permissions> <file>".bold().yellow(),
                    "chown <owner_name> <file>".bold().yellow(),
                    "uname".bold().yellow(),
                    "ps".bold().yellow(),
                    "sensors".bold().yellow(),
                    "free".bold().yellow(),
                    "uptime".bold().yellow(),
                    "df".bold().yellow(),
                    "cd <directory>".bold().yellow(),
                    "pwd".bold().yellow(),
                    "ls [directory]".bold().yellow(),
                    "exit or quit".bold().red()
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
