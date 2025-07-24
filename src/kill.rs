/*
This file tries to mimic the behavior of the Unix `kill` command
for Windows, allowing users to terminate processes by PID.

The Unix definition for 'kill' is:
kill [-signal|-s signal|-p] [-q value] [-a] [--timeout milliseconds signal] [--] pid|name...
We will follow this structure closely.

Arguments breakdown:
- -signal (e.g., -9, -TERM): Specify signal number or name //Windows does not use signals like Unix, but we can simulate this.
- -s signal: Alternative signal specification  
- -p: Print PID only, don't send signal
- -q value: Send signal with additional data
- -a: Apply to all processes with given name
- --timeout ms signal: Send signal, wait, then send second signal
- --: End of options marker
- pid|name...: Process IDs or names to target

Key things addressed:
1. Parsing command line arguments.
2. Checking if the PID is valid, the checks includes:
    - Protection against killing the current process.
    - Protecting against killing system processes.
    - Handling invalid PIDs. (Like negative or non-numeric values).
    - Handling non-existent PIDs. (Like out of bounds values)
3. Checking if process with specified name exists and is valid to be killed.
4. Implementing all arguments supported by unix
*/

use colored::*;

#[derive(Debug, Clone)]
pub struct KillOptions {
    pub signal: Option<String>,          // Signal
    pub signal_explicit: Option<String>, // Signal from -s flag
    pub print_only: bool,                // -p flag
    pub queue_value: Option<i32>,        // -q value
    pub all_processes: bool,             // -a flag
    pub timeout_ms: Option<u64>,         // --timeout milliseconds
    pub timeout_signal: Option<String>,  // Signal to send after timeout
    pub end_of_options: bool,            // -- encountered
    pub targets: Vec<String>,            // PIDs or process names
}

impl Default for KillOptions {
    fn default() -> Self {
        KillOptions {
            signal: None,
            signal_explicit: None,
            print_only: false,
            queue_value: None,
            all_processes: false,
            timeout_ms: None,
            timeout_signal: None,
            end_of_options: false,
            targets: Vec::new(),
        }
    }
}

pub fn execute(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: kill [-signal|-s signal|-p] [-q value] [-a] [--timeout milliseconds signal] [--] pid|name...".to_string());
    }

    let options = parse_arguments(args)?;
    validate_options(&options)?;
    
    // TODO: Implement the actual kill logic here
    // For now, just print what we parsed
    println!("Parsed options: {:?}", options);
    
    Ok(())
}

fn parse_arguments(args: &[&str]) -> Result<KillOptions, String> {
    let mut options = KillOptions::default();
    let mut i = 0;
    
    while i < args.len() {
        let arg = args[i];
        
        // If we've seen --, everything else is a target
        if options.end_of_options {
            options.targets.push(arg.to_string());
            i += 1;
            continue;
        }
        
        match arg {
            // End of options marker
            "--" => {
                options.end_of_options = true;
            }
            
            // Print PID only
            "-p" => {
                options.print_only = true;
            }
            
            // All processes flag
            "-a" => {
                options.all_processes = true;
            }
            
            // Explicit signal flag
            "-s" => {
                i += 1;
                if i >= args.len() {
                    return Err("Option -s requires a signal argument".to_string());
                }
                options.signal_explicit = Some(args[i].to_string());
            }
            
            // Queue value flag
            "-q" => {
                i += 1;
                if i >= args.len() {
                    return Err("Option -q requires a value argument".to_string());
                }
                match args[i].parse::<i32>() {
                    Ok(val) => options.queue_value = Some(val),
                    Err(_) => return Err(format!("Invalid queue value: {}", args[i])),
                }
            }
            
            // Timeout option
            arg if arg.starts_with("--timeout") => {
                if arg.contains('=') {
                    // Format: --timeout=5000
                    let parts: Vec<&str> = arg.splitn(2, '=').collect();
                    if parts.len() != 2 {
                        return Err("Invalid --timeout format".to_string());
                    }
                    match parts[1].parse::<u64>() {
                        Ok(ms) => options.timeout_ms = Some(ms),
                        Err(_) => return Err(format!("Invalid timeout value: {}", parts[1])),
                    }
                } else {
                    // Format: --timeout 5000 SIGNAL
                    i += 1;
                    if i >= args.len() {
                        return Err("Option --timeout requires milliseconds argument".to_string());
                    }
                    match args[i].parse::<u64>() {
                        Ok(ms) => options.timeout_ms = Some(ms),
                        Err(_) => return Err(format!("Invalid timeout value: {}", args[i])),
                    }
                    
                    // Next argument should be the signal
                    i += 1;
                    if i >= args.len() {
                        return Err("Option --timeout requires a signal argument".to_string());
                    }
                    options.timeout_signal = Some(args[i].to_string());
                }
            }
            
            // Signal arguments (start with -)
            arg if arg.starts_with('-') && arg.len() > 1 => {
                let signal = &arg[1..]; // Remove the leading -
                
                // Check if it's a valid signal (number or name)
                if signal.chars().all(|c| c.is_ascii_digit()) {
                    // Numeric signal
                    options.signal = Some(signal.to_string());
                } else if is_valid_signal_name(signal) {
                    // Named signal
                    options.signal = Some(signal.to_string());
                } else {
                    return Err(format!("Invalid signal: -{}", signal));
                }
            }
            
            // Everything else is a target (PID or process name)
            _ => {
                options.targets.push(arg.to_string());
            }
        }
        
        i += 1;
    }
    
    Ok(options)
}

fn validate_options(options: &KillOptions) -> Result<(), String> {
    // Must have at least one target unless using -p with no targets
    if options.targets.is_empty() && !options.print_only {
        return Err("No process ID or name specified".to_string());
    }
    
    // Cannot use both -s signal and -signal
    if options.signal.is_some() && options.signal_explicit.is_some() {
        return Err("Cannot specify signal with both -signal and -s options".to_string());
    }
    
    // -a flag only makes sense with process names, not PIDs
    if options.all_processes {
        for target in &options.targets {
            if target.chars().all(|c| c.is_ascii_digit()) {
                return Err("Cannot use -a flag with numeric PIDs".to_string());
            }
        }
    }
    
    // Timeout requires a signal
    if options.timeout_ms.is_some() && options.timeout_signal.is_none() {
        return Err("--timeout option requires a signal".to_string());
    }
    
    Ok(())
}

fn is_valid_signal_name(signal: &str) -> bool {
    // Common Unix signal names
    matches!(signal.to_uppercase().as_str(),
        "HUP" | "INT" | "QUIT" | "ILL" | "TRAP" | "ABRT" | "BUS" | "FPE" |
        "KILL" | "USR1" | "SEGV" | "USR2" | "PIPE" | "ALRM" | "TERM" |
        "STKFLT" | "CHLD" | "CONT" | "STOP" | "TSTP" | "TTIN" | "TTOU" |
        "URG" | "XCPU" | "XFSZ" | "VTALRM" | "PROF" | "WINCH" | "IO" |
        "PWR" | "SYS"
    )
}

// TODO: Implement these functions
fn handle_kill(options: &KillOptions) -> Result<(), String> {
    // Main kill logic will go here
    Ok(())
}