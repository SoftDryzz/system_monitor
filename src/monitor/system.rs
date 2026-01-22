// System monitoring module
// Main facade for accessing all system information

use super::cpu::CpuInfo;
use super::disk::DiskInfo;
use super::memory::MemoryInfo;
use super::process::ProcessInfo;
use sysinfo::System;

/// Main system monitor facade
pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    /// Create a new system monitor
    pub fn new() -> Self {
        SystemMonitor {
            sys: System::new_all(),
        }
    }

    /// Refresh all system information
    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    /// Get CPU information
    pub fn cpu_info(&self) -> CpuInfo {
        CpuInfo::from_system(&self.sys)
    }

    /// Get memory information
    pub fn memory_info(&self) -> MemoryInfo {
        MemoryInfo::from_system(&self.sys)
    }

    /// Get disk information
    pub fn disks_info(&self) -> Vec<DiskInfo> {
        super::disk::get_disks_info()
    }

    /// Get top N processes by CPU usage
    pub fn top_processes_by_cpu(&self, n: usize) -> Vec<ProcessInfo> {
        super::process::get_top_processes_by_cpu(&self.sys, n)
    }

    /// Get top N processes by memory usage
    pub fn top_processes_by_memory(&self, n: usize) -> Vec<ProcessInfo> {
        super::process::get_top_processes_by_memory(&self.sys, n)
    }

    /// Get top N busiest CPU cores
    pub fn top_cpu_cores(&self, n: usize) -> Vec<(usize, f32)> {
        super::process::get_top_cores(self.sys.cpus(), n)
    }

    /// Get total number of CPU cores
    pub fn cpu_count(&self) -> usize {
        self.sys.cpus().len()
    }

    /// Get system uptime in seconds
    pub fn uptime(&self) -> u64 {
        System::uptime()
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}
