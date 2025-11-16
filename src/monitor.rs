use nvml_wrapper::Nvml;
use std::{collections::HashMap, usize};
use sysinfo::{Component, Disks, Networks, Process, System};

pub struct Monitor {
    sys: System,
    nvml: Nvml,
    gpu_metrics: Vec<GpuMetrics>,
}

#[derive(Clone)]
pub struct GpuMetrics {
    pub name: String,
    pub temp_c: u32,
    pub usage_percent: u32,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub fan_percent: u32,
}

impl Monitor {
    pub fn new() -> Self {
        let nvml = Nvml::init().unwrap();

        Self {
            sys: System::new_all(),
            nvml,
            gpu_metrics: Vec::new(),
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

    pub fn gpu_metrics(&self) -> &Vec<GpuMetrics> {
        &self.gpu_metrics
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();

        let count = self.nvml.device_count().unwrap();
        self.gpu_metrics.clear();

        for i in 0..count {
            let device = self.nvml.device_by_index(i).unwrap();

            let memory = device.memory_info().unwrap();
            let util = device.utilization_rates().unwrap();
            let temp = device
                .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                .unwrap();
            let fan = device.fan_speed(0).unwrap();

            self.gpu_metrics.push(GpuMetrics {
                name: device.name().unwrap(),
                temp_c: temp,
                usage_percent: util.gpu,
                vram_used_mb: memory.used / 1024 / 1024,
                vram_total_mb: memory.total / 1024 / 1024,
                fan_percent: fan,
            });
        }
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
