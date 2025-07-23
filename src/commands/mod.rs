#[cfg(any(target_os = "windows", target_os = "macos"))]
pub mod sudo;
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub mod disown;
