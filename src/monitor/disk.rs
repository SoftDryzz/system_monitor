// Disk monitoring module
// Provides disk usage information for all mounted drives

use sysinfo::Disks;

/// Individual disk information
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_gb: f64,
    pub available_gb: f64,
    pub used_gb: f64,
    pub percentage: f64,
}

impl DiskInfo {
    /// Create disk info from sysinfo Disk
    pub fn from_disk(disk: &sysinfo::Disk) -> Self {
        const GB: f64 = 1_073_741_824.0; // 1024^3

        let total_gb = disk.total_space() as f64 / GB;
        let available_gb = disk.available_space() as f64 / GB;
        let used_gb = total_gb - available_gb;
        let percentage = if total_gb > 0.0 {
            (used_gb / total_gb) * 100.0
        } else {
            0.0
        };

        DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_gb,
            available_gb,
            used_gb,
            percentage,
        }
    }
}

/// Get information about all disks
pub fn get_disks_info() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();

    disks
        .iter()
        .filter(|disk| {
            // Filter out very small disks (likely virtual/system drives)
            disk.total_space() > 1_073_741_824 // > 1 GB
        })
        .map(DiskInfo::from_disk)
        .collect()
}
