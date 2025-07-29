#[cfg(windows)]
mod windows_process {
    use std::ffi::OsStr;
    use std::io;
    use std::mem::zeroed;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
    use winapi::um::winbase::CREATE_UNICODE_ENVIRONMENT;
    use winapi::um::winnt::HANDLE;

    #[derive(Debug)]
    pub struct ProcessHandle {
        pub process_handle: HANDLE,
        pub thread_handle: HANDLE,
    }

    impl Drop for ProcessHandle {
        fn drop(&mut self) {
            unsafe {
                if !self.process_handle.is_null() {
                    CloseHandle(self.process_handle);
                }
                if !self.thread_handle.is_null() {
                    CloseHandle(self.thread_handle);
                }
            }
        }
    }

    #[derive(Debug)]
    pub enum ProcessError {
        Io(io::Error),
        NullTermination,
        Other(String),
    }

    impl From<io::Error> for ProcessError {
        fn from(e: io::Error) -> Self {
            ProcessError::Io(e)
        }
    }

    fn to_wide_null(s: &str) -> Result<Vec<u16>, ProcessError> {
        let mut wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
        if wide.contains(&0) {
            return Err(ProcessError::NullTermination);
        }
        wide.push(0);
        Ok(wide)
    }

    pub fn spawn(
        exe_path: &str,
        args: &[&str],
        current_dir: Option<&str>,
    ) -> Result<ProcessHandle, ProcessError> {
        let mut cmdline = String::from(exe_path);
        for arg in args {
            cmdline.push(' ');
            cmdline.push_str(arg);
        }

        let mut cmdline_wide = to_wide_null(&cmdline)?;
        let exe_path_wide = to_wide_null(exe_path)?;
        let current_dir_wide = if let Some(dir) = current_dir {
            Some(to_wide_null(dir)?)
        } else {
            None
        };

        unsafe {
            let mut si: STARTUPINFOW = zeroed();
            si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
            let mut pi: PROCESS_INFORMATION = zeroed();

            let success = CreateProcessW(
                exe_path_wide.as_ptr(),
                cmdline_wide.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                CREATE_UNICODE_ENVIRONMENT,
                ptr::null_mut(),
                current_dir_wide
                    .as_ref()
                    .map(|v| v.as_ptr())
                    .unwrap_or(ptr::null()),
                &mut si,
                &mut pi,
            );

            if success == 0 {
                let err = io::Error::from_raw_os_error(GetLastError() as i32);
                return Err(ProcessError::Io(err));
            }

            Ok(ProcessHandle {
                process_handle: pi.hProcess,
                thread_handle: pi.hThread,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_to_wide_null_basic() {
            let wide = to_wide_null("hello").unwrap();
            assert_eq!(wide[wide.len() - 1], 0);
            assert_eq!(&wide[..5], &[104, 101, 108, 108, 111]);
        }

        #[test]
        fn test_to_wide_null_error_on_null() {
            let result = to_wide_null("hel\0lo");
            assert!(matches!(result, Err(ProcessError::NullTermination)));
        }

        #[test]
        fn test_spawn_invalid_exe_path() {
            let result = spawn("C:/not_a_real_exe.exe", &[], None);
            assert!(result.is_err());
        }
    }
}

#[cfg(windows)]
pub use windows_process::{ProcessError, ProcessHandle, spawn};

#[cfg(not(windows))]
mod fallback {
    #[derive(Debug)]
    pub struct ProcessHandle;

    #[derive(Debug)]
    pub enum ProcessError {
        UnsupportedPlatform,
    }

    pub fn spawn(
        _exe_path: &str,
        _args: &[&str],
        _current_dir: Option<&str>,
    ) -> Result<ProcessHandle, ProcessError> {
        Err(ProcessError::UnsupportedPlatform)
    }
}

#[cfg(not(windows))]
pub use fallback::{ProcessError, ProcessHandle, spawn};
