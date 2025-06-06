use sysinfo::{Networks, System};

pub fn execute() {
    let mut sys = System::new_all();

    sys.refresh_all();
    // Display system information (handle Option types):
    println!(
        "System name:             {}",
        System::name().unwrap_or_else(|| "Unknown".to_string())
    );

    println!("System kernel version:   {}", System::kernel_long_version());

    println!(
        "System OS version:       {}",
        System::long_os_version().unwrap_or_else(|| "Unknown".to_string())
    );

    println!(
        "System host name:        {}",
        System::host_name().unwrap_or_else(|| "Unknown".to_string())
    );

    println!("CPUs:         {}", sys.cpus().len());
    println!("CPU usage:    {}", sys.global_cpu_usage());
    println!("CPU Architecture: {:?}", System::cpu_arch());
    println!(
        "Physical cores: {}",
        System::physical_core_count().map_or("Unknown".to_string(), |count| count.to_string())
    );

    // Network interfaces with formatted data:
    let networks = Networks::new_with_refreshed_list();
    println!("\nNetworks:");
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
