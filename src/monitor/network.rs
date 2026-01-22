// Network monitoring module
// Provides network traffic information

use sysinfo::Networks;

/// Network traffic information
#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub download_speed: f64,    // Bytes per second
    pub upload_speed: f64,      // Bytes per second
    pub total_received: u64,    // Total bytes received
    pub total_transmitted: u64, // Total bytes transmitted
}

impl NetworkInfo {
    /// Create network info from current state
    pub fn new(
        download_speed: f64,
        upload_speed: f64,
        total_received: u64,
        total_transmitted: u64,
    ) -> Self {
        NetworkInfo {
            download_speed,
            upload_speed,
            total_received,
            total_transmitted,
        }
    }

    /// Format bytes to human-readable speed (KB/s, MB/s, GB/s)
    pub fn format_speed(bytes_per_sec: f64) -> (f64, &'static str) {
        const KB: f64 = 1024.0;
        const MB: f64 = 1024.0 * 1024.0;
        const GB: f64 = 1024.0 * 1024.0 * 1024.0;

        if bytes_per_sec >= GB {
            (bytes_per_sec / GB, "GB/s")
        } else if bytes_per_sec >= MB {
            (bytes_per_sec / MB, "MB/s")
        } else if bytes_per_sec >= KB {
            (bytes_per_sec / KB, "KB/s")
        } else {
            (bytes_per_sec, "B/s")
        }
    }

    /// Format bytes to human-readable size (KB, MB, GB, TB)
    pub fn format_bytes(bytes: u64) -> (f64, &'static str) {
        const KB: f64 = 1024.0;
        const MB: f64 = 1024.0 * 1024.0;
        const GB: f64 = 1024.0 * 1024.0 * 1024.0;
        const TB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

        let bytes_f = bytes as f64;

        if bytes_f >= TB {
            (bytes_f / TB, "TB")
        } else if bytes_f >= GB {
            (bytes_f / GB, "GB")
        } else if bytes_f >= MB {
            (bytes_f / MB, "MB")
        } else if bytes_f >= KB {
            (bytes_f / KB, "KB")
        } else {
            (bytes_f, "B")
        }
    }
}

/// Get network statistics
///
/// Note: Speed calculation requires two measurements with a time interval.
/// This function returns current totals, speed should be calculated by the caller.
pub fn get_network_info() -> NetworkInfo {
    let networks = Networks::new_with_refreshed_list();

    let mut total_received = 0u64;
    let mut total_transmitted = 0u64;

    for (_name, network) in networks.iter() {
        total_received += network.total_received();
        total_transmitted += network.total_transmitted();
    }

    // Speed will be calculated by SystemMonitor using deltas
    NetworkInfo::new(0.0, 0.0, total_received, total_transmitted)
}
