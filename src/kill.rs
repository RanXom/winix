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
#![cfg(windows)]

use colored::Colorize;
use std::thread;
use std::time::Duration;
use winapi::shared::minwindef::{BOOL, DWORD, LPARAM, TRUE};
use winapi::shared::windef::HWND;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetCurrentProcessId, OpenProcess, TerminateProcess};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First, Process32Next, TH32CS_SNAPPROCESS,
};
use winapi::um::wincon::{CTRL_BREAK_EVENT, CTRL_C_EVENT, GenerateConsoleCtrlEvent};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE};
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, PostMessageW, WM_CLOSE};

// Simple debug macro replacement
macro_rules! debug {
    ($($arg:tt)*) => {};
}

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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum WindowsKillMethod {
    ForceTerminate,    // SIGKILL (9) -> TerminateProcess
    GracefulCtrlC,     // SIGTERM (15), SIGINT (2) -> Ctrl+C
    GracefulCtrlBreak, // SIGQUIT (3) -> Ctrl+Break
    WindowClose,       // For GUI applications
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
        return Err(format!(
            "{}",
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
    let signal = options
        .signal
        .as_ref()
        .or(options.signal_explicit.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("9"); // Default to SIGKILL if no signal specified

    let kill_method = signal_to_windows_method(signal)?;
    debug!("Using kill method: {:?}", kill_method);

    // Process each target
    let mut results = Vec::new();
    for target in &options.targets {
        let result = if target.chars().all(|c| c.is_ascii_digit()) {
            // Target is a PID
            let pid: u32 = target
                .parse()
                .map_err(|_| format!("Invalid PID: {} must be a number or name", target))?;
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
fn kill_process_by_pid(
    pid: u32,
    method: &WindowsKillMethod,
    _options: &KillOptions,
) -> Result<(), String> {
    debug!("Attempting to kill PID {} using method {:?}", pid, method);
    validate_pid_safety(pid)?;
    if !process_exists(pid) {
        return Err(format!("No such process: {}", pid));
    }
    match method {
        WindowsKillMethod::ForceTerminate => force_terminate_process(pid),
        WindowsKillMethod::GracefulCtrlC => graceful_terminate_process(pid, false),
        WindowsKillMethod::GracefulCtrlBreak => graceful_terminate_process(pid, true),
        WindowsKillMethod::WindowClose => window_close_process(pid),
    }
}

// Kill processes by name
fn kill_process_by_name(
    name: &str,
    method: &WindowsKillMethod,
    options: &KillOptions,
) -> Result<(), String> {
    debug!(
        "Attempting to kill processes with name '{}' using method {:?}",
        name, method
    );

    let pids = find_processes_by_name(name)?;
    if pids.is_empty() {
        return Err(format!("No processes found with name: {}", name));
    }
    let targets = if options.all_processes {
        pids
    } else {
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
fn handle_timeout_kill(
    results: &[(String, Result<(), String>)],
    timeout_ms: u64,
    options: &KillOptions,
) -> Result<(), String> {
    debug!("Handling timeout kill: {} ms", timeout_ms);

    // Get the timeout signal (this should be validated already)
    let timeout_signal = options
        .timeout_signal
        .as_ref()
        .ok_or("Timeout signal not specified")?;
    let timeout_method = signal_to_windows_method(timeout_signal)?;
    println!(
        "{}",
        format!(
            "Timeout kill: waiting {} ms before sending {} signal",
            timeout_ms, timeout_signal
        )
        .yellow()
    );
    // Collect PIDs from successful initial kills
    let mut target_pids = Vec::new();
    for (target, result) in results {
        if result.is_ok() {
            // Try to parse as PID, or find processes by name
            if target.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(pid) = target.parse::<u32>() {
                    target_pids.push((pid, target.clone()));
                }
            } else {
                // For process names, we need to find PIDs again
                // (they might have changed since initial kill)
                match find_processes_by_name(target) {
                    Ok(pids) => {
                        if options.all_processes {
                            for pid in pids {
                                target_pids.push((pid, target.clone()));
                            }
                        } else if !pids.is_empty() {
                            target_pids.push((pids[0], target.clone()));
                        }
                    }
                    Err(_) => {
                        debug!(
                            "Could not find processes for name '{}' during timeout check",
                            target
                        );
                    }
                }
            }
        }
    }

    if target_pids.is_empty() {
        println!("{}", "No processes to check for timeout kill".yellow());
        return Ok(());
    }

    // Wait for the specified timeout
    println!(
        "{}",
        format!(
            "Waiting {} ms for processes to terminate gracefully...",
            timeout_ms
        )
        .cyan()
    );
    thread::sleep(Duration::from_millis(timeout_ms));

    // Check which processes are still alive and kill them with the timeout signal
    let mut still_alive = Vec::new();
    for (pid, target_name) in target_pids {
        if process_exists(pid) {
            still_alive.push((pid, target_name));
        } else {
            println!(
                "{}",
                format!("Process {} ({}) terminated gracefully", pid, target_name).green()
            );
        }
    }

    if still_alive.is_empty() {
        println!(
            "{}",
            "All processes terminated gracefully within timeout period".green()
        );
        return Ok(());
    }

    // Kill remaining processes with the timeout signal
    println!(
        "{}",
        format!(
            "Sending {} signal to {} remaining process(es)",
            timeout_signal,
            still_alive.len()
        )
        .yellow()
    );

    let mut timeout_errors = Vec::new();
    let mut timeout_success = 0;

    for (pid, target_name) in still_alive {
        debug!(
            "Sending timeout signal {} to process {} ({})",
            timeout_signal, pid, target_name
        );

        // Validate safety again (process might have changed)
        if let Err(e) = validate_pid_safety(pid) {
            timeout_errors.push(format!("Cannot kill {} ({}): {}", pid, target_name, e));
            continue;
        }
        match kill_process_with_method(pid, &timeout_method) {
            Ok(_) => {
                timeout_success += 1;
                println!(
                    "{}",
                    format!(
                        "Timeout kill: {} sent to process {} ({})",
                        timeout_signal, pid, target_name
                    )
                    .green()
                );
            }
            Err(e) => {
                timeout_errors.push(format!(
                    "Timeout kill failed for {} ({}): {}",
                    pid, target_name, e
                ));
            }
        }
    }
    if !timeout_errors.is_empty() {
        println!(
            "{}",
            format!("Timeout kill errors: {}", timeout_errors.join("; ")).red()
        );
    }
    if timeout_success > 0 {
        println!(
            "{}",
            format!(
                "Timeout kill completed: {} process(es) killed with {}",
                timeout_success, timeout_signal
            )
            .green()
        );
    }

    Ok(())
}

// Helper function to kill a process with a specific method
fn kill_process_with_method(pid: u32, method: &WindowsKillMethod) -> Result<(), String> {
    match method {
        WindowsKillMethod::ForceTerminate => force_terminate_process(pid),
        WindowsKillMethod::GracefulCtrlC => graceful_terminate_process(pid, false),
        WindowsKillMethod::GracefulCtrlBreak => graceful_terminate_process(pid, true),
        WindowsKillMethod::WindowClose => window_close_process(pid),
    }
}

// Report the results of kill operations
fn report_kill_results(results: &[(String, Result<(), String>)]) -> Result<(), String> {
    let mut has_errors = false;
    for (target, result) in results {
        match result {
            Ok(_) => {
                println!(
                    "{}",
                    format!("Successfully processed target: {}", target).green()
                );
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

// ============================================================================
// Helper Functions
// ============================================================================

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

fn is_valid_signal_name(signal: &str) -> bool {
    matches!(
        signal.to_uppercase().as_str(),
        // Termination signals
        "TERM" | "KILL" | "INT" | "QUIT" |
        // Numeric equivalents
        "2" | "3" | "9" | "15"
    )
}

fn signal_to_windows_method(signal: &str) -> Result<WindowsKillMethod, String> {
    match signal.to_uppercase().as_str() {
        "KILL" | "9" => Ok(WindowsKillMethod::ForceTerminate),
        "TERM" | "15" => Ok(WindowsKillMethod::GracefulCtrlC),
        "INT" | "2" => Ok(WindowsKillMethod::GracefulCtrlC),
        "QUIT" | "3" => Ok(WindowsKillMethod::GracefulCtrlBreak),
        _ => Err(format!(
            "Signal '{}' is not supported on Windows. Supported signals: TERM(15), INT(2), QUIT(3), KILL(9)",
            signal
        )),
    }
}

fn process_exists(pid: u32) -> bool {
    debug!("Checking if process {} exists", pid);
    unsafe {
        // Try to open the process with minimal rights just to check existence
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION,
            0, // bInheritHandle = FALSE
            pid,
        );

        if process_handle.is_null() {
            debug!("Process {} does not exist or cannot be accessed", pid);
            return false;
        }

        // Process exists and we can access it
        CloseHandle(process_handle);
        debug!("Process {} exists and is accessible", pid);
        true
    }
}

// Safety validation for PIDs
fn validate_pid_safety(pid: u32) -> Result<(), String> {
    // Protect against killing critical system processes
    const PROTECTED_PIDS: &[u32] = &[0, 4, 8]; // System, System Idle, etc.
    if PROTECTED_PIDS.contains(&pid) {
        return Err(format!("Cannot kill system process with PID {}", pid));
    }
    // Protect against killing current process
    unsafe {
        let current_pid = GetCurrentProcessId();
        if pid == current_pid {
            return Err("Cannot kill current process".to_string());
        }
    }
    Ok(())
}

// Find processes by name
fn find_processes_by_name(name: &str) -> Result<Vec<u32>, String> {
    debug!("Finding processes with name: {}", name);
    let mut matching_pids = Vec::new();

    unsafe {
        // Take a snapshot of all processes in the system
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            let error_code = GetLastError();
            return Err(format!(
                "Failed to create process snapshot: Windows error code {}",
                error_code
            ));
        }
        // Initialize the PROCESSENTRY32 structure
        let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;
        // Get the first process in the snapshot
        let mut result = Process32First(snapshot, &mut process_entry);
        if result == 0 {
            CloseHandle(snapshot);
            let error_code = GetLastError();
            return Err(format!(
                "Failed to get first process: Windows error code {}",
                error_code
            ));
        }

        // Iterate through all processes in the snapshot
        loop {
            // Convert the szExeFile (which is a null-terminated C string) to a Rust string
            let exe_name = std::ffi::CStr::from_ptr(process_entry.szExeFile.as_ptr())
                .to_string_lossy()
                .to_lowercase();
            let target_name = name.to_lowercase();

            // Check if the process name matches (with or without .exe extension)
            let matches = exe_name == target_name
                || exe_name == format!("{}.exe", target_name)
                || (exe_name.ends_with(".exe") && exe_name[..exe_name.len() - 4] == target_name);
            if matches {
                debug!(
                    "Found matching process: {} (PID: {})",
                    exe_name, process_entry.th32ProcessID
                );
                matching_pids.push(process_entry.th32ProcessID);
            }

            // Move to the next process
            result = Process32Next(snapshot, &mut process_entry);
            if result == 0 {
                break; // No more processes
            }
        }

        // Clean up the snapshot handle
        CloseHandle(snapshot);
    }

    debug!(
        "Found {} processes matching name '{}'",
        matching_pids.len(),
        name
    );
    Ok(matching_pids)
}

// Force terminate a process (SIGKILL equivalent)
fn force_terminate_process(pid: u32) -> Result<(), String> {
    debug!("Force terminating process {} using TerminateProcess", pid);
    unsafe {
        // Open the process with termination rights
        let process_handle = OpenProcess(
            PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION,
            0, // bInheritHandle = FALSE
            pid,
        );

        if process_handle.is_null() {
            let error_code = GetLastError();
            return match error_code {
                5 => Err(format!(
                    "Access denied: Cannot terminate process {} (insufficient privileges)",
                    pid
                )),
                87 => Err(format!("Invalid PID: Process {} does not exist", pid)),
                _ => Err(format!(
                    "Failed to open process {}: Windows error code {}",
                    pid, error_code
                )),
            };
        }

        // Attempt to terminate the process
        let result = TerminateProcess(
            process_handle,
            1, // Exit code - using 1 to indicate forced termination
        );

        // Clean up the handle
        CloseHandle(process_handle);

        if result == 0 {
            let error_code = GetLastError();
            return match error_code {
                5 => Err(format!(
                    "Access denied: Cannot terminate process {} (protected process)",
                    pid
                )),
                _ => Err(format!(
                    "Failed to terminate process {}: Windows error code {}",
                    pid, error_code
                )),
            };
        }

        println!(
            "{}",
            format!("Force terminated process {} (SIGKILL)", pid).green()
        );
        Ok(())
    }
}

// Graceful terminate using console control events (SIGINT/SIGQUIT)
fn graceful_terminate_process(pid: u32, use_ctrl_break: bool) -> Result<(), String> {
    let (signal_type, ctrl_event) = if use_ctrl_break {
        ("Ctrl+Break (SIGQUIT)", CTRL_BREAK_EVENT)
    } else {
        ("Ctrl+C (SIGINT)", CTRL_C_EVENT)
    };

    debug!(
        "Gracefully terminating process {} with {}",
        pid, signal_type
    );

    unsafe {
        let result = GenerateConsoleCtrlEvent(ctrl_event, pid);

        if result == 0 {
            let error_code = GetLastError();
            return match error_code {
                6 => Err(format!(
                    "Invalid handle: Process {} is not a console process or not accessible",
                    pid
                )),
                87 => Err(format!(
                    "Invalid parameter: Process {} may not exist or not be a console application",
                    pid
                )),
                _ => {
                    // Fallback: If console control event fails, try alternative approaches
                    debug!(
                        "Console control event failed (error {}), attempting alternative graceful termination",
                        error_code
                    );
                    return graceful_terminate_fallback(pid, use_ctrl_break);
                }
            };
        }

        println!(
            "{}",
            format!("Sent {} to process {}", signal_type, pid).green()
        );
        Ok(())
    }
}

// Fallback graceful termination for non-console applications
fn graceful_terminate_fallback(pid: u32, use_ctrl_break: bool) -> Result<(), String> {
    let signal_type = if use_ctrl_break {
        "Ctrl+Break"
    } else {
        "Ctrl+C"
    };

    debug!("Using fallback graceful termination for process {}", pid);
    match window_close_process(pid) {
        Ok(_) => {
            println!(
                "{}",
                format!(
                    "Sent window close message to process {} (fallback for {})",
                    pid, signal_type
                )
                .yellow()
            );
            Ok(())
        }
        Err(_) => {
            println!(
                "{}",
                format!(
                    "Warning: Graceful termination failed for process {}, using force termination",
                    pid
                )
                .yellow()
            );
            force_terminate_process(pid)
        }
    }
}

// Close process by sending WM_CLOSE to its windows
fn window_close_process(pid: u32) -> Result<(), String> {
    debug!("Attempting to close windows for process {}", pid);
    unsafe {
        let mut data = EnumWindowsData {
            target_pid: pid,
            windows_found: 0,
            windows_closed: 0,
        };
        // Enumerate all top-level windows and send WM_CLOSE to those owned by our target process
        let result = EnumWindows(
            Some(enum_windows_proc),
            &mut data as *mut EnumWindowsData as LPARAM,
        );
        if result == 0 {
            let error_code = GetLastError();
            return Err(format!(
                "Failed to enumerate windows: Windows error code {}",
                error_code
            ));
        }
        if data.windows_found == 0 {
            return Err(format!(
                "No windows found for process {} (may be a console-only or background process)",
                pid
            ));
        }
        if data.windows_closed == 0 {
            return Err(format!(
                "Found {} windows for process {} but failed to send close messages",
                data.windows_found, pid
            ));
        }
        println!(
            "{}",
            format!(
                "Sent close message to {} window(s) of process {}",
                data.windows_closed, pid
            )
            .green()
        );
        Ok(())
    }
}

// Structure to pass data to the window enumeration callback
#[repr(C)]
struct EnumWindowsData {
    target_pid: DWORD,
    windows_found: u32,
    windows_closed: u32,
}

// Callback function for EnumWindows
unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let data = &mut *(lparam as *mut EnumWindowsData);
        let mut window_pid: DWORD = 0;
        // Get the process ID that owns this window
        GetWindowThreadProcessId(hwnd, &mut window_pid);
        // If this window belongs to our target process
        if window_pid == data.target_pid {
            data.windows_found += 1;
            debug!(
                "Found window for process {}, sending WM_CLOSE",
                data.target_pid
            );

            // Send WM_CLOSE message to the window
            let result = PostMessageW(hwnd, WM_CLOSE, 0, 0);
            if result != 0 {
                data.windows_closed += 1;
            }
        }

        TRUE // Continue enumeration
    }
}
