//! Safe abstraction over Windows CreateProcessW
//!
//! Provides process::spawn() for launching processes with robust error handling and resource management.

use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::mem::zeroed;
use winapi::um::processthreadsapi::CreateProcessW;
use winapi::um::processthreadsapi::PROCESS_INFORMATION;
use winapi::um::processthreadsapi::STARTUPINFOW;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winbase::CREATE_UNICODE_ENVIRONMENT;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnt::HANDLE;

/// Represents a handle to a spawned process.
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

/// Errors that can occur when spawning a process.
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

/// Converts a Rust &str to a null-terminated UTF-16 Vec<u16> for Win32 APIs.
fn to_wide_null(s: &str) -> Result<Vec<u16>, ProcessError> {
    let mut wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
    if wide.contains(&0) {
        return Err(ProcessError::NullTermination);
    }
    wide.push(0);
    Ok(wide)
}

/// Spawns a new process using CreateProcessW.
///
/// - `exe_path`: Path to the executable.
/// - `args`: Command-line arguments (excluding the executable name).
/// - `current_dir`: Optional working directory.
///
/// Returns a ProcessHandle on success, or ProcessError on failure.
pub fn spawn(
    exe_path: &str,
    args: &[&str],
    current_dir: Option<&str>,
) -> Result<ProcessHandle, ProcessError> {
    // Build command line: first arg is exe_path, then args, all joined by spaces
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
            ptr::null_mut(), // Process security attributes
            ptr::null_mut(), // Thread security attributes
            0,               // Inherit handles
            CREATE_UNICODE_ENVIRONMENT,
            ptr::null_mut(), // Environment
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

        // On success, wrap handles in ProcessHandle
        Ok(ProcessHandle {
            process_handle: pi.hProcess,
            thread_handle: pi.hThread,
        })
    }
} 