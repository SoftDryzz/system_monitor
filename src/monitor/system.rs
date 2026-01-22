// System monitoring module
// Main facade for accessing all system information

use super::cpu::CpuInfo;
use super::disk::DiskInfo;
use super::memory::MemoryInfo;
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
