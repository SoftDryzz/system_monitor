// System monitoring module
// Main facade for accessing all system information

use super::cpu::CpuInfo;
use super::disk::DiskInfo;
use super::memory::MemoryInfo;
use super::network::NetworkInfo;
use super::process::ProcessInfo;
use std::time::Instant;
use sysinfo::System;

/// Main system monitor facade
pub struct SystemMonitor {
    sys: System,
    last_network_check: Option<(Instant, u64, u64)>, // (time, rx, tx)
}

impl SystemMonitor {
    /// Create a new system monitor
    pub fn new() -> Self {
        SystemMonitor {
            sys: System::new_all(),
            last_network_check: None,
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

    /// Get network information with speed calculation
    pub fn network_info(&mut self) -> NetworkInfo {
        let current = super::network::get_network_info();
        let now = Instant::now();

        if let Some((last_time, last_rx, last_tx)) = self.last_network_check {
            let elapsed = now.duration_since(last_time).as_secs_f64();

            if elapsed > 0.0 {
                let rx_diff = current.total_received.saturating_sub(last_rx) as f64;
                let tx_diff = current.total_transmitted.saturating_sub(last_tx) as f64;

                let download_speed = rx_diff / elapsed;
                let upload_speed = tx_diff / elapsed;

                self.last_network_check =
                    Some((now, current.total_received, current.total_transmitted));

                return NetworkInfo::new(
                    download_speed,
                    upload_speed,
                    current.total_received,
                    current.total_transmitted,
                );
            }
        }

        // First call or error - store current values
        self.last_network_check = Some((now, current.total_received, current.total_transmitted));
        current
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
