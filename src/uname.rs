use sysinfo::{Networks, System};

pub fn execute() {
    let mut sys = System::new_all();

    sys.refresh_all();

    // RAM and swap information with formatted memory:
    println!("Used memory : {}", format_memory(sys.used_memory()));
    println!("Total memory: {}", format_memory(sys.total_memory()));
    println!("Total swap  : {}", format_memory(sys.total_swap()));
    println!("Used swap   : {}", format_memory(sys.used_swap()));

    // Display system information (handle Option types):
    println!(
        "System name:             {}",
        System::name().unwrap_or_else(|| "Unknown".to_string())
    );

    println!(
        "System kernel version:   {}",
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    );

    println!(
        "System OS version:       {}",
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    );

    println!(
        "System host name:        {}",
        System::host_name().unwrap_or_else(|| "Unknown".to_string())
    );

    println!("CPUs:         {}", sys.cpus().len());
    println!("CPU usage:    {}", sys.global_cpu_usage());
    // Network interfaces with formatted data:
    let networks = Networks::new_with_refreshed_list();
    println!("Networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} (down) / {} (up)",
            format_memory(data.total_received()),
            format_memory(data.total_transmitted()),
        );
    }
}

// Helper function to format bytes into human-readable format
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
