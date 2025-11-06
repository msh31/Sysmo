use sysinfo::{Component, Disks, Networks, System};

fn main() {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();
        print!("\x1B[2J\x1B[1;1H");
        println!("=SYSMON=");

        println!("===CPU USAGE===");
        for cpu in sys.cpus() {
            println!("{}: {:.2}%", cpu.name(), cpu.cpu_usage());
        }
    }
}
