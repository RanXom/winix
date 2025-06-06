use sysinfo::System;

pub fn execute() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Used memory : {}", format_memory(sys.used_memory()));
    println!("Total memory: {}", format_memory(sys.total_memory()));
    println!("Total swap  : {}", format_memory(sys.total_swap()));
    println!("Used swap   : {}", format_memory(sys.used_swap()));
}

fn format_memory(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let mb = bytes as f64 / (1024.0 * 1024.0);
    let kb = bytes as f64 / 1024.0;

    if gb >= 1.0 {
        format!("{:.2} GB", gb)
    } else if mb >= 1.0 {
        format!("{:.2} MB", mb)
    } else if kb >= 1.0 {
        format!("{:.2} KB", kb)
    } else {
        format!("{} bytes", bytes)
    }
}
