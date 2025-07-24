#[cfg(windows)]
mod tests {
    use winapi::um::synchapi::WaitForSingleObject;
    use winapi::um::winbase::INFINITE;
    use winix::process::{spawn, ProcessError};

    #[test]
    fn test_spawn_success_cmd() {
        // Try to launch cmd.exe with /C exit (should exit immediately)
        let result = spawn("C:\\Windows\\System32\\cmd.exe", &["/C", "exit"], None);
        assert!(result.is_ok(), "Expected success, got: {:?}", result);
        let handle = result.unwrap();
        unsafe {
            // Wait for process to exit
            WaitForSingleObject(handle.process_handle, INFINITE);
        }
    }

    #[test]
    fn test_spawn_invalid_path() {
        let result = spawn("C:\\not_a_real_exe.exe", &[], None);
        assert!(result.is_err(), "Expected error for invalid path");
        match result {
            Err(ProcessError::Io(e)) => {
                assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
            }
            _ => panic!("Expected Io error for invalid path"),
        }
    }

    #[test]
    fn test_spawn_malformed_args() {
        // Malformed args: include a null byte (should error in to_wide_null)
        let result = spawn("C:\\Windows\\System32\\cmd.exe", &["/C\0"], None);
        assert!(result.is_err(), "Expected error for malformed args");
        match result {
            Err(ProcessError::NullTermination) => {}
            _ => panic!("Expected NullTermination error"),
        }
    }

    #[test]
    fn test_spawn_insufficient_permissions() {
        // Try to launch a system process that requires admin (simulate by using a protected path)
        let result = spawn("C:\\Windows\\System32\\config\\SAM", &[], None);
        assert!(result.is_err(), "Expected error for insufficient permissions");
        match result {
            Err(ProcessError::Io(e)) => {
                assert!(matches!(e.kind(), std::io::ErrorKind::PermissionDenied | std::io::ErrorKind::Other));
            }
            _ => panic!("Expected Io error for permission denied"),
        }
    }
} 