use sysinfo::System;

pub fn execute() {
    let mut sys = System::new_all();
    let load_avg = System::load_average();
    sys.refresh_all();

    println!("System booted at {} seconds", System::boot_time());
    println!("System running since {} seconds", System::uptime());
    println!(
        "one minute: {}%, five minutes: {}%, fifteen minutes: {}%",
        load_avg.one, load_avg.five, load_avg.fifteen,
    );
}
