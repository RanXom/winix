use sysinfo::Disks;

pub fn execute() {
    let disks = Disks::new_with_refreshed_list();

    // Print header
    println!(
        "{:<20} {:<15} {:<15} {:<15}",
        "Disk", "Total", "Available", "Used"
    );
    println!("{:-<65}", "");

    // Print disk information in rows
    for disk in disks.list() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total - available;

        println!(
            "{:<20} {:<15} {:<15} {:<15}",
            format!("{:?}", disk.name()),
            format_memory(total),
            format_memory(available),
            format_memory(used)
        );
    }
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
