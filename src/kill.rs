/*
This file tries to mimic the behavior of the Unix `kill` command
for Windows, allowing users to terminate processes by PID or name.

The Unix definition for 'kill' is:
kill [-signal|-s signal|-p] [-q value] [-a] [--timeout milliseconds signal] [--] pid|name...
We will follow this structure closely.

Arguments breakdown:
- -signal: Specify signal number or name 
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
    - Support for reasonable signals like -2, -3, -9, -15 (INT, QUIT, KILL, TERM)
    - Ensures -a tag is used with only names processes
*/

use colored::*;
use log::debug;

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
        return Err(format!("{}",
            "Usage: kill [-signal|-s signal|-p] [-q value] [-a] [--timeout milliseconds signal] [--] pid|name...\n\
            \n\
            Supported signals on Windows:\n\
            -2, -INT    Interrupt (Ctrl+C)\n\
            -3, -QUIT   Quit (Ctrl+Break)\n\
            -9, -KILL   Force terminate (default)\n\
            -15, -TERM  Graceful terminate (Ctrl+C)\n\
            \n\
            Examples:\n\
            kill 1234           # Force terminate process 1234\n\
            kill -TERM 1234     # Graceful terminate\n\
            kill -9 1234        # Force terminate\n\
            kill -a notepad     # Kill all notepad processes"
        ));
    }

    let options = parse_arguments(args)?;
    validate_options(&options)?;
    debug!("Parsed options: {:?}", options);
    handle_kill(&options)
}

fn handle_kill(options: &KillOptions) -> Result<(), String> {
    debug!("Starting kill operation");
    
    // Handle special modes first
    if options.print_only {
        return handle_print_only_mode(options);
    }
    
    // Determine the kill method to use
    let signal = options.signal.as_ref()
        .or(options.signal_explicit.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("9"); // Default to SIGKILL if no signal specified
    
    let kill_method = signal_to_windows_method(signal)
    debug!("Using kill method: {:?}", kill_method);
    
    // Process each target
    let mut results = Vec::new();
    for target in &options.targets {
        let result = if target.chars().all(|c| c.is_ascii_digit()) {
            // Target is a PID
            let pid: u32 = target.parse().map_err(|_| format!("Invalid PID: {} must be a number or name", target))?;
            kill_process_by_pid(pid, &kill_method, options)
        } else {
            // Target is a process name
            kill_process_by_name(target, &kill_method, options)
        };
        results.push((target.clone(), result));
    }
    
    // Handle timeout logic if specified
    if let Some(timeout_ms) = options.timeout_ms {
        handle_timeout_kill(&results, timeout_ms, options)?;
    }
    
    // Report results
    report_kill_results(&results)?;
    
    Ok(())
}

// Handle -p flag: just print PIDs without killing
fn handle_print_only_mode(options: &KillOptions) -> Result<(), String> {
    debug!("Print-only mode activated");  
    for target in &options.targets {
        if target.chars().all(|c| c.is_ascii_digit()) {
            println!("{}", target);
        } else {
            // Process name, find and print all matching PIDs
            let pids = find_processes_by_name(target)?;
            if pids.is_empty() {
                return Err(format!("No processes found with name: {}", target));
            }
            if options.all_processes {
                for pid in pids {
                    println!("{}", pid);
                }
            } else {
                // Just print the first one
                println!("{}", pids[0]);
            }
        }
    }
    
    Ok(())
}


// Kill a specific process by PID
fn kill_process_by_pid(pid: u32, method: &WindowsKillMethod, options: &KillOptions) -> Result<(), String> {
    debug!("Attempting to kill PID {} using method {:?}", pid, method);
    
    // Safety checks
    validate_pid_safety(pid)?;
    
    // Check if process exists
    if !process_exists(pid) {
        return Err(format!("No such process: {}", pid));
    }
    
    // Perform the actual kill operation
    match method {
        WindowsKillMethod::ForceTerminate => force_terminate_process(pid),
        WindowsKillMethod::GracefulCtrlC => graceful_terminate_process(pid, false),
        WindowsKillMethod::GracefulCtrlBreak => graceful_terminate_process(pid, true),
        WindowsKillMethod::WindowClose => window_close_process(pid),
    }
}

// Kill processes by name
fn kill_process_by_name(name: &str, method: &WindowsKillMethod, options: &KillOptions) -> Result<(), String> {
    debug!("Attempting to kill processes with name '{}' using method {:?}", name, method);
    
    let pids = find_processes_by_name(name)?;
    if pids.is_empty() {
        return Err(format!("No processes found with name: {}", name));
    }
    
    let targets = if options.all_processes {
        pids
    } else {
        // Just kill the first one found
        vec![pids[0]]
    };
    
    let mut errors = Vec::new();
    let mut success_count = 0;
    
    for pid in targets {
        match kill_process_by_pid(pid, method, options) {
            Ok(_) => {
                success_count += 1;
                println!("{}", format!("Killed process {} ({})", pid, name).green());
            }
            Err(e) => {
                errors.push(format!("Failed to kill {} ({}): {}", pid, name, e));
            }
        }
    }
    
    if !errors.is_empty() {
        return Err(errors.join("; "));
    }
    
    if success_count == 0 {
        return Err(format!("No processes were killed for name: {}", name));
    }
    
    Ok(())
}

// Handle timeout logic: send initial signal, wait, then send final signal
fn handle_timeout_kill(results: &[(String, Result<(), String>)], timeout_ms: u64, options: &KillOptions) -> Result<(), String> {
    debug!("Handling timeout kill: {} ms", timeout_ms);
    
    // For now, this is a placeholder
    // In a full implementation, you'd:
    // 1. Send the initial signal (usually TERM)
    // 2. Wait for the specified timeout
    // 3. Check if processes are still alive
    // 4. Send the final signal (from timeout_signal)
    
    println!("{}", format!("Timeout kill not yet implemented (would wait {} ms)", timeout_ms).yellow());
    Ok(())
}

// Report the results of kill operations
fn report_kill_results(results: &[(String, Result<(), String>)]) -> Result<(), String> {
    let mut has_errors = false;
    
    for (target, result) in results {
        match result {
            Ok(_) => {
                println!("{}", format!("Successfully processed target: {}", target).green());
            }
            Err(e) => {
                println!("{}", format!("Failed to process {}: {}", target, e).red());
                has_errors = true;
            }
        }
    }
    
    if has_errors {
        Err("Some kill operations failed".to_string())
    } else {
        Ok(())
    }
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
    
    // Validate signal if specified
    let signal_to_check = options.signal.as_ref().or(options.signal_explicit.as_ref());
    if let Some(signal) = signal_to_check {
        signal_to_windows_method(signal)?; // This will return an error for unsupported signals
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
    
    // Validate timeout signal if specified
    if let Some(timeout_signal) = &options.timeout_signal {
        signal_to_windows_method(timeout_signal)?;
    }
    
    Ok(())
}

// Helper function to check if a signal name is valid
// Only supports signals that can be reasonably simulated on Windows
fn is_valid_signal_name(signal: &str) -> bool {
    matches!(signal.to_uppercase().as_str(),
        // Termination signals
        "TERM" | "KILL" | "INT" | "QUIT" |
        // Numeric equivalents
        "2" | "3" | "9" | "15"
    )
}

// Map Unix signal to Windows termination method
#[derive(Debug, Clone)]
pub enum WindowsKillMethod {
    ForceTerminate,    // SIGKILL (9) -> TerminateProcess
    GracefulCtrlC,     // SIGTERM (15), SIGINT (2) -> Ctrl+C
    GracefulCtrlBreak, // SIGQUIT (3) -> Ctrl+Break
    WindowClose,       // For GUI applications
}

fn signal_to_windows_method(signal: &str) -> Result<WindowsKillMethod, String> {
    match signal.to_uppercase().as_str() {
        "KILL" | "9" => Ok(WindowsKillMethod::ForceTerminate),
        "TERM" | "15" => Ok(WindowsKillMethod::GracefulCtrlC),
        "INT" | "2" => Ok(WindowsKillMethod::GracefulCtrlC),
        "QUIT" | "3" => Ok(WindowsKillMethod::GracefulCtrlBreak),
        _ => Err(format!("Signal '{}' is not supported on Windows. Supported signals: TERM(15), INT(2), QUIT(3), KILL(9)", signal)),
    }
}

// ============================================================================
// HELPER FUNCTIONS - These will contain the actual Windows API calls
// ============================================================================

// Check if a process exists
fn process_exists(pid: u32) -> bool {
    // TODO: Implement using Windows API
    // Use OpenProcess() to check if process exists
    debug!("Checking if process {} exists", pid);
    true // Placeholder
}

// Safety validation for PIDs
fn validate_pid_safety(pid: u32) -> Result<(), String> {
    // Protect against killing critical system processes
    const PROTECTED_PIDS: &[u32] = &[0, 4, 8]; // System, System Idle, etc.
    
    if PROTECTED_PIDS.contains(&pid) {
        return Err(format!("Cannot kill system process with PID {}", pid));
    }
    
    // Protect against killing current process
    // TODO: Get current process ID and compare
    // if pid == current_process_id() {
    //     return Err("Cannot kill current process".to_string());
    // }
    
    Ok(())
}

// Find processes by name
fn find_processes_by_name(name: &str) -> Result<Vec<u32>, String> {
    // TODO: Implement using Windows API
    // Use CreateToolhelp32Snapshot() and Process32First/Process32Next
    debug!("Finding processes with name: {}", name);
    
    // Placeholder - return empty vec for now
    Ok(vec![])
}

// Force terminate a process (SIGKILL equivalent)
fn force_terminate_process(pid: u32) -> Result<(), String> {
    // TODO: Implement using TerminateProcess()
    debug!("Force terminating process {}", pid);
    println!("{}", format!("Force terminated process {}", pid).green());
    Ok(())
}

// Graceful terminate using console control events (SIGTERM/SIGINT)
fn graceful_terminate_process(pid: u32, use_ctrl_break: bool) -> Result<(), String> {
    // TODO: Implement using GenerateConsoleCtrlEvent()
    let signal_type = if use_ctrl_break { "Ctrl+Break" } else { "Ctrl+C" };
    debug!("Gracefully terminating process {} with {}", pid, signal_type);
    println!("{}", format!("Sent {} to process {}", signal_type, pid).green());
    Ok(())
}

// Close process by sending WM_CLOSE to its windows
fn window_close_process(pid: u32) -> Result<(), String> {
    // TODO: Implement using EnumWindows() and PostMessage(WM_CLOSE)
    debug!("Closing windows for process {}", pid);
    println!("{}", format!("Sent close message to process {}", pid).green());
    Ok(())
}

