// System Monitor - A cross-platform CLI system monitoring tool
// Author: SoftDryzz
// License: MIT

mod cli;
mod display;
mod monitor;

use display::formatter;
use monitor::system::SystemMonitor;

fn main() {
    // Parse CLI arguments (for future use)
    let _args = cli::parse_args();

    // Create system monitor instance
    let mut monitor = SystemMonitor::new();

    // Refresh system information
    monitor.refresh();

    // Display system information
    formatter::print_header();
    formatter::print_cpu_info(&monitor);
    formatter::print_memory_info(&monitor);
    formatter::print_uptime(&monitor);
    formatter::print_footer();
}
