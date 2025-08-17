use colored::Colorize;
use std::io::{self, Write};
use std::process::Command;

/// Execute git commands by shelling out to the system git
pub fn execute(args: &[&str]) {
    // Check if git is available
    if !is_git_available() {
        println!("{}", "Error: Git is not installed or not in PATH".red());
        println!(
            "{}",
            "Please install Git and ensure it's in your PATH".yellow()
        );
        return;
    }

    // If no arguments provided, show git help
    if args.is_empty() {
        show_git_help();
        return;
    }

    // Execute the git command
    execute_git_command(args);
}

/// Check if git is available on the system
pub fn is_git_available() -> bool {
    match Command::new("git").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Execute a git command with the provided arguments
fn execute_git_command(args: &[&str]) {
    let mut cmd = Command::new("git");
    cmd.args(args);

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
                        format!("Git command failed with exit code: {}", code).red()
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to execute git command: {}", e).red());
        }
    }
}

/// Show interactive git mode for complex operations
pub fn interactive_mode() {
    println!("{}", "Git Interactive Mode".bold().green());
    println!(
        "{}",
        "Type git commands (without 'git' prefix) or 'exit' to quit".dimmed()
    );
    println!(
        "{}",
        "Example: status, log --oneline, add ., commit -m \"message\"".dimmed()
    );
    println!();

    loop {
        // Show git prompt
        print!("{}", "git> ".bold().cyan());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                if input == "exit" || input == "quit" {
                    println!("{}", "Exiting git interactive mode".green());
                    break;
                }

                // Split the input into arguments
                let args: Vec<&str> = input.split_whitespace().collect();
                execute_git_command(&args);
            }
            Err(e) => {
                eprintln!("{}", format!("Error reading input: {}", e).red());
                break;
            }
        }
    }
}

/// Show git help and common commands
fn show_git_help() {
    println!("{}", "Git Commands Available".bold().green());
    println!("{}", "Usage: git <command> [options]".dimmed());
    println!();

    println!("{}", "Most Common Git Commands:".bold().white());
    println!("  {:<15} {}", "status".yellow(), "Show working tree status");
    println!("  {:<15} {}", "log".yellow(), "Show commit logs");
    println!(
        "  {:<15} {}",
        "add <file>".yellow(),
        "Add file contents to index"
    );
    println!(
        "  {:<15} {}",
        "commit".yellow(),
        "Record changes to repository"
    );
    println!("  {:<15} {}", "push".yellow(), "Update remote refs");
    println!(
        "  {:<15} {}",
        "pull".yellow(),
        "Fetch and merge from remote"
    );
    println!("  {:<15} {}", "clone <url>".yellow(), "Clone a repository");
    println!(
        "  {:<15} {}",
        "branch".yellow(),
        "List, create, or delete branches"
    );
    println!(
        "  {:<15} {}",
        "checkout".yellow(),
        "Switch branches or restore files"
    );
    println!(
        "  {:<15} {}",
        "merge".yellow(),
        "Join development histories"
    );
    println!(
        "  {:<15} {}",
        "diff".yellow(),
        "Show changes between commits"
    );
    println!(
        "  {:<15} {}",
        "reset".yellow(),
        "Reset current HEAD to state"
    );
    println!(
        "  {:<15} {}",
        "stash".yellow(),
        "Stash changes in working directory"
    );
    println!(
        "  {:<15} {}",
        "remote".yellow(),
        "Manage remote repositories"
    );
    println!(
        "  {:<15} {}",
        "init".yellow(),
        "Create empty Git repository"
    );
    println!();

    println!("{}", "Examples:".bold().cyan());
    println!("  {}", "git status".dimmed());
    println!("  {}", "git log --oneline".dimmed());
    println!("  {}", "git add .".dimmed());
    println!("  {}", "git commit -m \"Initial commit\"".dimmed());
    println!("  {}", "git push origin main".dimmed());
    println!("  {}", "git pull origin main".dimmed());
    println!("  {}", "git branch -a".dimmed());
    println!("  {}", "git checkout -b new-feature".dimmed());
    println!();

    println!("{}", "Interactive Mode:".bold().magenta());
    println!("  {}", "git --interactive".dimmed());
    println!(
        "  {}",
        "  Enter interactive git mode for easier command execution".dimmed()
    );
}

/// Check if current directory is a git repository
pub fn is_git_repo() -> bool {
    // Use a faster check - just look for .git directory first
    if std::path::Path::new(".git").exists() {
        return true;
    }

    // Check for GIT_DIR environment variable
    if std::env::var("GIT_DIR").is_ok() {
        return true;
    }

    // Fallback to git command only if needed (this is slower)
    match Command::new("git")
        .args(&["rev-parse", "--git-dir"])
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Get current git branch name
pub fn get_current_branch() -> Option<String> {
    match Command::new("git")
        .args(&["branch", "--show-current"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !branch.is_empty() {
                    Some(branch)
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

/// Get git repository status summary
pub fn get_repo_status() -> Option<String> {
    match Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let status = String::from_utf8_lossy(&output.stdout);
                if status.trim().is_empty() {
                    Some("clean".to_string())
                } else {
                    Some("dirty".to_string())
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
