use sysinfo::System;

pub fn execute() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Print header
    println!("{}", "=".repeat(90));
    println!("{:^90}", "PROCESS LIST");
    println!("{}", "=".repeat(90));

    // Column headers
    println!(
        "{:<8} {:<25} {:<8} {:<10} {:<12} {:<15}",
        "PID", "NAME", "CPU%", "MEMORY", "DISK R/W", "STATUS"
    );
    println!("{}", "-".repeat(90));

    // Get processes and sort by CPU usage
    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| {
        b.1.cpu_usage()
            .partial_cmp(&a.1.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Display top 25 processes
    for (pid, process) in processes.iter().take(25) {
        let name = truncate_string(&process.name().to_string_lossy(), 24);
        let cpu = format!("{:.1}", process.cpu_usage());
        let memory = format_bytes(process.memory());

        // Get disk usage
        let disk_usage = process.disk_usage();
        let disk_info = format!(
            "{}/{}",
            format_bytes(disk_usage.read_bytes),
            format_bytes(disk_usage.written_bytes)
        );

        let status = format!("{:?}", process.status());

        println!(
            "{:<8} {:<25} {:<8} {:<10} {:<12} {:<15}",
            pid, name, cpu, memory, disk_info, status
        );
    }

    println!("{}", "-".repeat(90));

    // System summary
    println!("\n{:^40}", "SYSTEM SUMMARY");
    println!("{}", "-".repeat(40));
    println!("Total processes: {}", sys.processes().len());
    println!("CPU cores: {}", sys.cpus().len());
    println!("Global CPU usage: {:.1}%", sys.global_cpu_usage());
    println!("Total memory: {}", format_bytes(sys.total_memory()));
    println!("Used memory: {}", format_bytes(sys.used_memory()));
    println!("Total swap: {}", format_bytes(sys.total_swap()));
    println!("Used swap: {}", format_bytes(sys.used_swap()));
}

// Helper function to format bytes
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

// Helper function to truncate long strings
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
