use colored::*;
use std::io::{self, Write};
mod chmod;

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
        "  {}\n  {}",
        "chmod".bold().yellow(),
        "exit or quit".bold().red()
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
            "help" => {
                println!("{}", "Available Commands:".bold().white());
                println!(
                    "  {}\n  {}",
                    "chmod <permissions> <file>".bold().yellow(),
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
