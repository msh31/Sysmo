mod monitor;

use std::thread;
use std::time::Duration;

use crate::monitor::Monitor;

fn main() {
    let mut monitor = Monitor::new();

    loop {
        monitor.refresh();

        print!("\x1B[2J\x1B[1;1H");

        println!("{}", monitor.system_info());

        println!("\nUptime: {:.2} Days", monitor.uptime_days());

        println!("\n=== CPU Usages ===");
        for (i, usage) in monitor.cpu_usage().iter().enumerate() {
            println!("Core {}: {:.2}%", i + 1, usage);
        }

        println!("\n=== Memory ===");
        let total = monitor.total_memory() / 1024 / 1024;
        let available = monitor.available_memory() / 1024 / 1024;
        let used = total - available;

        println!("Total: {} MB", total);
        println!("Used: {} MB", used);
        println!("Available: {} MB", available);

        println!("\n=== Top Processes (by Memory) ===");
        let mut processes = monitor.processes_grouped();

        processes.sort_by(|a, b| b.2.cmp(&a.2));

        for (i, (name, cpu, memory, count)) in processes.iter().take(15).enumerate() {
            println!(
                "{}. {} ({} processes) - CPU: {:.1}% | Mem: {} MB",
                i + 1,
                name,
                count,
                cpu,
                memory / 1024 / 1024 // Convert bytes to MB
            );
        }

        thread::sleep(Duration::from_secs(1));
    }
}
