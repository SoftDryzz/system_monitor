// System monitoring module
// Main facade for accessing all system information

use sysinfo::System;
use super::cpu::CpuInfo;
use super::memory::MemoryInfo;

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
