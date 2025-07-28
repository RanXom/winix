use colored::*;
use std::env;
use std::fs;
use std::io::{self, Write};
#[cfg(windows)]
use winix::pipeline::execute_pipeline;
#[cfg(windows)]
use winix::{chmod, chown};
use winix::{echo, touch};
use crate::cat::cat;
use std::process;
use input::LineEditor;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Config, DefaultEditor};

#[cfg(windows)]
mod pipeline;
mod cd;
#[cfg(windows)]
mod chmod;
#[cfg(windows)]
mod chown;
mod disown;
mod df;
mod free;
mod git;
#[cfg(windows)]
mod kill;

mod powershell;
mod ps;
mod sensors;
mod sudo;
mod tui;
mod uname;
mod uptime;
mod cat;
mod rm;
mod input;


fn main() {

    let args: Vec<String> = env::args().collect();
    let mut editor = input::LineEditor::new();
        // Start the REPL loop
        loop {
            // Show the prompt and read input
            let readline = editor.read_line();
    
            match readline {
                Ok(line) => {
                    // Save input to history
                    editor.add_history_entry(line.as_str());
    
                    // Exit condition
                    if line.trim() == "exit" {
                        println!("Exiting shell.");
                        break;
                    }
    
                    // Process the input (for now, just print it)
                    println!("You entered: {}", line);
                }
                Err(ReadlineError::Interrupted) => {
                    // Handle Ctrl+C
                    println!("^C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    // Handle Ctrl+D
                    println!("^D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

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
    let mut cached_git_info = String::new();
    let mut last_dir = String::new();
    let mut git_check_counter = 0;

    loop {
        let cwd = env::current_dir().unwrap_or_else(|_| "?".into());
        let current_dir = cwd.display().to_string();

        if current_dir != last_dir || git_check_counter % 10 == 0 {
            cached_git_info = if git::is_git_repo() {
                if let Some(branch) = git::get_current_branch() {
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

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();
        let command_args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        match command.as_str() {
            "exit" | "quit" => {
                println!("{}", "Goodbye!".bold().blue());
                println!(
                    "{}{}",
                    "Want to contribute? Check out: ".bold().white(),
                    "https://github.com/0xsambit/winix".bold().green()
                );
                break;
            }
         
            #[cfg(windows)]
            "chmod" => {
                if parts.len() < 2 {
                    println!("{}", "Usage: chmod <mode> <file>".red());
                } else {
                    let args: Vec<&str> = parts[1..].iter().copied().collect();
                    chmod::execute(&args);
                }
            }

            #[cfg(windows)]
            "chown" => {
                if parts.len() < 3 {
                    println!("{}", "Usage: chown <owner>:<group> <file>".red());
                } else {
                    let args: Vec<&str> = parts[1..].iter().copied().collect();
                    chown::execute(&args);
                }
            }

            "echo" => echo::run(&command_args),
            "touch" => touch::run(&command_args),


            // "echo" => {
            //     if !command_args.is_empty() {
            //         println!("{}", command_args.join(" "));
            //     } else {
            //         println!(); // print a blank line if no arguments
            //     }
            // }
        
            // "touch" => {
            //     for file in &command_args {
            //         match std::fs::OpenOptions::new()
            //             .create(true)
            //             .write(true)
            //             .append(true)
            //             .open(file)
            //         {
            //             Ok(_) => {}
            //             Err(e) => eprintln!("touch: cannot create file '{}': {}", file, e),
            //         }
            //     }
            // }
            "uname" => uname::execute(),
            "ps" => ps::execute(),
            "sensors" => sensors::execute(),
            "free" => free::execute(),
            "uptime" => uptime::execute(),
            "df" => df::execute(),

            "cd" => {
                if parts.len() < 2 {
                    println!("{}", "Usage: cd <directory>".red());
                } else if let Err(e) = cd_command(parts[1]) {
                    println!("{}", format!("cd: {}", e).red());
                }
            }

            "pwd" => {
                if let Err(e) = pwd_command() {
                    println!("{}", format!("pwd: {}", e).red());
                }
            }

            "ls" => {
                let dir = if parts.len() > 1 { parts[1] } else { "." };
                if let Err(e) = ls_command(dir) {
                    println!("{}", format!("ls: {}", e).red());
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
                    #[cfg(windows)]
                    match kill::execute(&args) {
                        Ok(_) => {}
                        Err(e) => println!("{}", format!("kill: {}", e).red()),
                    }
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
                    #[cfg(windows)]
                    match kill::execute(&args) {
                        Ok(_) => {}
                        Err(e) => println!("{}", format!("kill: {}", e).red()),
                    }
                }
            }
            "psh" | "powershell" => {
                if parts.len() == 1 {
                    powershell::execute(&[]);
                } else if parts.len() == 2 && parts[1] == "--interactive" {
                    powershell::interactive_mode();
                } else {
                    let args: Vec<&str> = parts[1..].iter().copied().collect();
                    powershell::execute(&args);
                }
            }
           
            "rm" => {
                let args: Vec<String> = env::args().collect();
                if args.len() < 3 {
                     eprintln!("Please provide a file to remove");
                 return;
             }
 
             for file in &args[2..] {
                 match fs::remove_file(file) {
                     Ok(_) => println!("Deleted {}", file),
                     Err(e) => eprintln!("Failed to delete {}: {}", file, e),
                 }
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

fn rm(files: Vec<&str>) -> Result<(), std::io::Error> {
    for file in files {
        std::fs::remove_file(file)?;
        println!("Deleted: {}", file);
    }
    Ok(())
}
