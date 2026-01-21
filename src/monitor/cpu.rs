// CPU monitoring module
// Provides CPU usage information per core and globally

use sysinfo::{System, Cpu};

/// CPU information structure
#[derive(Debug)]
pub struct CpuInfo {
    pub global_usage: f32,
    pub cores: Vec<CoreInfo>,
}

/// Individual CPU core information
#[derive(Debug)]
pub struct CoreInfo {
    pub index: usize,
    pub usage: f32,
}

impl CpuInfo {
    /// Get CPU information from system
    pub fn from_system(sys: &System) -> Self {
        let global_usage = sys.global_cpu_usage();
        
        let cores = sys.cpus()
            .iter()
            .enumerate()
            .map(|(index, cpu)| CoreInfo {
                index,
                usage: cpu.cpu_usage(),
            })
            .collect();

        CpuInfo {
            global_usage,
            cores,
        }
    }
}
