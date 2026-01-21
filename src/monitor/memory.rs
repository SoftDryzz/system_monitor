// Memory monitoring module
// Provides RAM usage information

use sysinfo::System;

/// Memory information structure
#[derive(Debug)]
pub struct MemoryInfo {
    pub total_gb: f64,
    pub used_gb: f64,
    pub percentage: f64,
}

impl MemoryInfo {
    /// Get memory information from system
    pub fn from_system(sys: &System) -> Self {
        const GB: f64 = 1_073_741_824.0; // 1024^3
        
        let total_gb = sys.total_memory() as f64 / GB;
        let used_gb = sys.used_memory() as f64 / GB;
        let percentage = (used_gb / total_gb) * 100.0;

        MemoryInfo {
            total_gb,
            used_gb,
            percentage,
        }
    }
}
