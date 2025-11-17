// use nvml_wrapper::Nvml;
use std::{collections::HashMap, usize};
use sysinfo::{Component, Disks, Networks, Process, System};

pub struct Monitor {
    sys: System,
    // nvml: Nvml,
    // gpu_metrics: Vec<GpuMetrics>,
}

// pub struct GpuMetrics {
//     #[derive(Clone, Debug)]
//     pub name: Option<String>,
//     pub temp_c: Option<u32>,
//     pub usage_percent: Option<u32>,
//     pub vram_used_mb: Option<u64>,
//     pub vram_total_mb: Option<u64>,
//     pub fan_percent: Option<u32>,
// }

impl Monitor {
    pub fn new() -> Self {
        // let nvml = Nvml::init().unwrap();

        Self {
            sys: System::new_all(),
            // nvml,
            // gpu_metrics: Vec::new(),
        }
    }

    pub fn processes(&self) -> Vec<(String, f32, u64)> {
        self.sys
            .processes()
            .values()
            .map(|process| {
                (
                    process.name().to_string_lossy().to_string(),
                    process.cpu_usage(),
                    process.memory(),
                )
            })
            .collect()
    }

    pub fn processes_grouped(&self) -> Vec<(String, f32, u64, usize)> {
        let mut grouped: HashMap<String, (f32, u64, usize)> = HashMap::new();

        for process in self.sys.processes().values() {
            let name = process.name().to_string_lossy().to_string();
            let entry = grouped.entry(name).or_insert((0.0, 0, 0));
            entry.0 = process.cpu_usage();
            entry.1 = process.memory();
            entry.2 += 1;
        }

        grouped
            .into_iter()
            .map(|(name, (cpu, mem, count))| (name, cpu, mem, count))
            .collect()
    }

    pub fn total_memory(&self) -> u64 {
        self.sys.total_memory()
    }

    pub fn used_memory(&self) -> u64 {
        self.sys.used_memory()
    }

    pub fn available_memory(&self) -> u64 {
        self.sys.available_memory()
    }

    pub fn cpu_usage(&self) -> Vec<f32> {
        self.sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    }

    pub fn cpu_model(&self) -> &str {
        self.sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand())
            .unwrap_or("<unknown>")
    }

    pub fn cpu_frequencies(&self) -> Vec<u64> {
        self.sys.cpus().iter().map(|cpu| cpu.frequency()).collect()
    }

    // pub fn gpu_metrics(&self) -> &Vec<GpuMetrics> {
    //     &self.gpu_metrics
    // }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        // self.gpu_metrics.clear();
        //
        // if let Ok(nvml) = Nvml::init() {
        //     let count = nvml.device_count().unwrap_or(0);
        //
        //     for i in 0..count {
        //         let device = nvml.device_by_index(i).ok();
        //
        //         let gpu_metric = GpuMetrics {
        //             name: device.as_ref().and_then(|d| d.name().ok()),
        //             temp_c: device.as_ref().and_then(|d| {
        //                 d.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
        //                     .ok()
        //             }),
        //             usage_percent: device
        //                 .as_ref()
        //                 .and_then(|d| d.utilization_rates().ok().map(|u| u.gpu)),
        //             vram_used_mb: device
        //                 .as_ref()
        //                 .and_then(|d| d.memory_info().ok().map(|m| m.used / 1024 / 1024)),
        //             vram_total_mb: device
        //                 .as_ref()
        //                 .and_then(|d| d.memory_info().ok().map(|m| m.total / 1024 / 1024)),
        //             fan_percent: device.as_ref().and_then(|d| d.fan_speed(0).ok()),
        //         };
        //
        //         self.gpu_metrics.push(gpu_metric);
        //     }
        // } else {
        //     self.gpu_metrics.push(GpuMetrics {
        //         name: None,
        //         temp_c: None,
        //         usage_percent: None,
        //         vram_used_mb: None,
        //         vram_total_mb: None,
        //         fan_percent: None,
        //     });
        // }
    }

    pub fn uptime_days(&self) -> f32 {
        System::uptime() as f32 / 3600.0 / 24.0
    }

    pub fn system_info(&self) -> String {
        let kernel = System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned());
        let os_version = System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned());
        let distro = System::distribution_id();
        let arch = std::env::consts::ARCH;

        format!("{} kernel {} • {} • {}", distro, kernel, arch, os_version)
    }
}
