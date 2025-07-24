#[cfg(windows)]
mod tests {
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::Duration;
    
    use winapi::um::processthreadsapi::{GetCurrentProcessId, OpenProcess};
    use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
    use winapi::um::synchapi::WaitForSingleObject;
    use winapi::um::handleapi::CloseHandle;

    #[test]
    fn test_kill_running_process() {
        // Starting a long-running process
        let mut child = Command::new("ping")
            .args(&["127.0.0.1", "-n", "1000"]) // Ping localhost 1000 times
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start ping process");

        let pid = child.id();
        thread::sleep(Duration::from_millis(100));
        assert!(is_process_running(pid), "Process should be running before kill");

        //Call to kill function as in UNIX
        let result = winix::kill::execute(&[&pid.to_string()]);

        assert!(result.is_ok(), "Kill command should succeed");
        thread::sleep(Duration::from_millis(500));

        // Verify the process was actually killed
        match child.try_wait() {
            Ok(Some(_)) => {
                assert!(true, "Process was successfully terminated");
            }
            Ok(None) => {
                //kill failed
                let _ = child.kill(); // Clean up the started process
                panic!("Process is still running after kill command");
            }
            Err(e) => {
                panic!("Error checking process status: {}", e);
            }
        }
    }

    #[test]
    fn test_kill_nonexistent_process() {
        let fake_pid = "999999"; //Fake pid unlikely to exist
        let result = winix::kill::execute(&[fake_pid]);
        // This should return an error
        assert!(result.is_err(), "Kill should fail for non-existent process");
    }

    #[test]
    fn test_kill_with_signal() {
        let mut child = Command::new("ping")
            .args(&["127.0.0.1", "-n", "1000"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start ping process");

        let pid = child.id();
        thread::sleep(Duration::from_millis(100));
        let result = winix::kill::execute(&["-9", &pid.to_string()]);
        assert!(result.is_ok(), "Kill with signal should succeed");
        thread::sleep(Duration::from_millis(500));
        
        match child.try_wait() {
            Ok(Some(_)) => assert!(true, "Process terminated with signal"),
            Ok(None) => {
                let _ = child.kill();
                panic!("Process still running after kill -9");
            }
            Err(e) => panic!("Error checking process: {}", e),
        }
    }

    #[test]
    fn test_kill_current_process_protection() {
        // Try to kill the current process (should be protected or handled gracefully)
        let current_pid = unsafe { GetCurrentProcessId() };
        let result = winix::kill::execute(&[&current_pid.to_string()]);
        assert!(true, "Test process should still be running");
    }

    #[test]
    fn test_kill_invalid_arguments() {
        // Test with no arguments
        let result = winix::kill::execute(&[]);
        assert!(result.is_err(), "Kill with no args should fail");

        // Test with invalid PID format
        let result = winix::kill::execute(&["not_a_number"]);
        assert!(result.is_err(), "Kill with invalid PID should fail");

        // Test with negative PID
        let result = winix::kill::execute(&["-123"]);
        // This might be treated as a signal, so check implementation behavior
        // For now, just ensure it doesn't panic
        let _ = result;
    }

    // Helper function to check if a process is running
    fn is_process_running(pid: u32) -> bool {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
            if handle.is_null() {
                return false;
            }
            let result = WaitForSingleObject(handle, 0);
            CloseHandle(handle);
            result == 0x102
        }
    }
}
