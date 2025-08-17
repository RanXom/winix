use colored::Colorize;
use sysinfo::Components;

pub fn execute() {
    println!("{}", "System Component Temperatures:".bold().blue());
    println!("{}", "=".repeat(50));

    let mut components = Components::new_with_refreshed_list();

    components.refresh(false);

    if components.is_empty() {
        println!("{}", "No temperature sensors found or accessible.".yellow());
        println!(
            "{}",
            "Note: On Windows, temperature sensors may require:".dimmed()
        );
        println!("{}", "  - Administrator privileges".dimmed());
        println!(
            "{}",
            "  - Hardware that supports temperature monitoring".dimmed()
        );
        println!("{}", "  - Proper drivers installed".dimmed());
        return;
    }

    let mut sensor_count = 0;
    for component in &components {
        let label = component.label();
        let temperature = component.temperature();
        let max_temp = component.max();
        let critical_temp = component.critical();

        if let Some(temp) = temperature {
            if temp > 0.0 {
                sensor_count += 1;
                print!("{}: ", label.bold());

                let temp_str = format!("{:.1}°C", temp);
                if let Some(crit) = critical_temp {
                    if temp >= crit {
                        print!("{}", temp_str.red().bold());
                    } else if temp >= crit * 0.8 {
                        print!("{}", temp_str.yellow());
                    } else {
                        print!("{}", temp_str.green());
                    }
                } else {
                    print!("{}", temp_str.cyan());
                }

                if let Some(max) = max_temp {
                    if max > 0.0 {
                        print!(" {}", format!("(Max: {:.1}°C)", max).dimmed());
                    }
                }

                if let Some(crit) = critical_temp {
                    if crit > 0.0 {
                        print!(" {}", format!("[Critical: {:.1}°C]", crit).red().dimmed());
                    }
                }

                println!();
            }
        }
    }

    if sensor_count == 0 {
        println!("{}", "No valid temperature data available.".yellow());
        println!(
            "{}",
            "This may be normal on Windows systems without accessible sensors.".dimmed()
        );
    } else {
        println!("{}", "=".repeat(50));
        println!(
            "{}",
            format!("Found {} temperature sensor(s)", sensor_count).green()
        );
    }
}
