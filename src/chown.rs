use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

use colored::*;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::accctrl::SE_FILE_OBJECT;
use winapi::um::aclapi::SetNamedSecurityInfoW;
use winapi::um::winnt::*;
use windows_acl::helper::*;

pub fn execute(args: &[&str]) {
    if args.len() < 2 {
        println!(
            "{}",
            "Usage: chown [OPTION]... [OWNER][:[GROUP]] FILE...".red()
        );
        println!(
            "{}",
            "   or: chown [OPTION]... --reference=RFILE FILE...".red()
        );
        println!();
        println!("{}", "Examples:".yellow());
        println!("  {}", "chown alice file.txt".dimmed());
        println!("  {}", "chown alice:developers file.txt".dimmed());
        println!("  {}", "chown :developers file.txt".dimmed());
        println!("  {}", "chown --recursive alice:developers /mydir".dimmed());
        println!("  {}", "chown --reference=ref.txt file.txt".dimmed());
        return;
    }

    let mode = args[0];
    let files = &args[1..];

    for filename in files {
        if !std::path::Path::new(filename).exists() {
            println!(
                "{}",
                format!(
                    "chown: cannot access  '{}': No such file or directory",
                    filename
                )
                .red()
            );
            continue;
        }

        match parse_and_mode(filename, mode) {
            Ok(_) => {
                println!("{}", format!("Owner changed for '{}'", filename).green());
            }
            Err(e) => {
                println!("{}", format!("chown: {}", e).red());
            }
        }
    }
}

fn parse_and_mode(file: &str, mode: &str) -> Result<bool, String> {
    let (user, group) = if let Some(colon_pos) = mode.find(':') {
        let user_part = &mode[..colon_pos];
        let group_part = &mode[colon_pos + 1..];

        let user = if user_part.is_empty() {
            None
        } else {
            Some(user_part)
        };

        let group = if group_part.is_empty() {
            None
        } else {
            Some(group_part)
        };

        (user, group)
    } else {
        // No colon found, just user
        (Some(mode), None)
    };

    // Change owner if user is specified
    if let Some(username) = user {
        change_owner_with_only_sid(file, username)?;
    }

    // Handle group if specified (Windows doesn't have traditional Unix groups)
    if let Some(_groupname) = group {
        println!("Note: Group ownership changes are not fully supported on Windows");
    }

    Ok(true)
}

fn change_owner_with_only_sid(file: &str, mode: &str) -> Result<(), String> {
    let user_sid = match name_to_sid(mode, None) {
        Ok(user_sid) => user_sid,
        Err(0) => return Err(format!("invalid user: {}", mode)),
        Err(code) => {
            return Err(format!(
                "error looking user '{}': Error code '{}' ",
                mode, code
            ));
        }
    };

    let file_wide: Vec<u16> = OsStr::new(file)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let res = SetNamedSecurityInfoW(
            file_wide.as_ptr() as *mut u16,
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION,
            user_sid.as_ptr() as *mut _,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );

        if res != ERROR_SUCCESS {
            return Err(format!(
                "failed to change owner of '{}': error code '{}'",
                file, res
            ));
        }
        println!("Successfully changed owner of '{}'", file);
        Ok(())
    }
}
