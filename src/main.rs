use sysinfo::{Component, Disks, Networks, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("=SYSMON=");

    println!("=> system:");
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes\n", sys.used_swap());

    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}\n", System::host_name());

    println!("NB CPUs: {}", sys.cpus().len());
}
