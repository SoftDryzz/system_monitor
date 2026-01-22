// System Monitor - A cross-platform CLI system monitoring tool
// Author: SoftDryzz
// License: MIT

mod cli;
mod display;
mod monitor;

use display::formatter;
use monitor::system::SystemMonitor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // Parse CLI arguments
    let args = cli::parse_args();

    // Setup Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    // Create system monitor instance
    let mut monitor = SystemMonitor::new();

    if args.watch {
        // Watch mode: continuous updates
        watch_mode(&mut monitor, args.interval, running);
    } else {
        // Single snapshot mode
        single_snapshot(&mut monitor);
    }
}

/// Display a single snapshot of system information
fn single_snapshot(monitor: &mut SystemMonitor) {
    monitor.refresh();

    formatter::print_header(false, 0);
    formatter::print_cpu_info(monitor);
    formatter::print_memory_info(monitor);
    formatter::print_disk_info(monitor);
    formatter::print_uptime(monitor);
    formatter::print_footer(false);
}

/// Continuously monitor and display system information
fn watch_mode(monitor: &mut SystemMonitor, interval: u64, running: Arc<AtomicBool>) {
    // Initial display
    formatter::clear_screen();

    while running.load(Ordering::SeqCst) {
        // Refresh system information
        monitor.refresh();

        // Clear screen and move cursor to top
        formatter::clear_screen();

        // Display information
        formatter::print_header(true, interval);
        formatter::print_cpu_info(monitor);
        formatter::print_memory_info(monitor);
        formatter::print_disk_info(monitor);
        formatter::print_uptime(monitor);
        formatter::print_footer(true);

        // Wait for interval (but check running flag more frequently)
        let sleep_iterations = interval * 10; // Check every 100ms
        for _ in 0..sleep_iterations {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    // Clean exit
    println!("\nExiting System Monitor...");
}
