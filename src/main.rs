use colored::*;
use std::io::{self, Write};
mod chmod;
mod chown;
mod df;
mod free;
mod ps;
mod sensors;
mod uname;
mod uptime;
fn main() {
    show_splash_screen();
    command_loop();
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
    println!("{}", "Available Commands:".bold().white());
    println!(
        "  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
        "chmod".bold().yellow(),
        "chown".bold().yellow(),
        "uname".bold().yellow(),
        "ps".bold().yellow(),
        "sensors".bold().yellow(),
        "free".bold().yellow(),
        "uptime".bold().yellow(),
        "df".bold().yellow(),
        "exit".bold().red()
    );
    println!();
}

fn command_loop() {
    loop {
        print!("{}", "winix> ".bold().green());
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
            "help" => {
                println!("{}", "Available Commands:".bold().white());
                println!(
                    "  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
                    "chmod <permissions> <file>".bold().yellow(),
                    "chown <owner_name> <file>".bold().yellow(),
                    "uname".bold().yellow(),
                    "ps".bold().yellow(),
                    "sensors".bold().yellow(),
                    "free".bold().yellow(),
                    "uptime".bold().yellow(),
                    "df".bold().yellow(),
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
