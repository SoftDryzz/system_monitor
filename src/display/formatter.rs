// Formatter module - Visual output formatting utilities
// Provides functions to display system information in a beautiful format

use crate::monitor::system::SystemMonitor;
use std::io::{self, Write};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Clear the terminal screen
/// Platform-specific implementation for best results
pub fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        // Windows: Use system command for most reliable clearing
        if std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .is_err()
        {
            // Fallback to ANSI codes
            print!("\x1B[2J\x1B[1;1H");
            let _ = io::stdout().flush();
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Unix/Linux/macOS: Use ANSI escape codes
        print!("\x1B[2J\x1B[1;1H");
        let _ = io::stdout().flush();
    }
}

/// Print the header banner
///
/// # Arguments
/// * `watch_mode` - Whether watch mode is enabled
/// * `interval` - Update interval in seconds (only shown in watch mode)
pub fn print_header(watch_mode: bool, interval: u64) {
    println!("╭─────────────────────────────────────╮");
    if watch_mode {
        println!("│   System Monitor v{} (Watch)    │", VERSION);
        println!("│   Updating every {} second(s)...    │", interval);
        println!("│   Press Ctrl+C to exit              │");
    } else {
        println!("│   System Monitor v{}              │", VERSION);
    }
    println!("╰─────────────────────────────────────╯");
    println!();
}

/// Print CPU information with visual bars
pub fn print_cpu_info(monitor: &SystemMonitor) {
    let cpu_info = monitor.cpu_info();
    let bar = create_bar(cpu_info.global_usage, 20);

    println!("CPU Usage:  {:.1}%  {}", cpu_info.global_usage, bar);

    // Display each core
    for core in &cpu_info.cores {
        let core_bar = create_bar(core.usage, 15);
        println!(
            "  Core {:2}:  {:5.1}%  {}",
            core.index, core.usage, core_bar
        );
    }
    println!();
}

/// Print memory information with visual bar
pub fn print_memory_info(monitor: &SystemMonitor) {
    let mem_info = monitor.memory_info();
    let bar = create_bar(mem_info.percentage as f32, 20);

    println!(
        "Memory:     {:.2}/{:.2} GB ({:.1}%)",
        mem_info.used_gb, mem_info.total_gb, mem_info.percentage
    );
    println!("            {}", bar);
    println!();
}

/// Print system uptime
pub fn print_uptime(monitor: &SystemMonitor) {
    let uptime = monitor.uptime();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;

    println!(
        "Uptime:     {} days, {} hours, {} minutes",
        days, hours, minutes
    );
}

/// Print the footer
///
/// # Arguments
/// * `watch_mode` - Whether watch mode is enabled
pub fn print_footer(watch_mode: bool) {
    if !watch_mode {
        println!();
        println!("Run with --watch for continuous monitoring");
    }
}

/// Create a visual progress bar
///
/// # Arguments
/// * `percentage` - Value between 0 and 100
/// * `width` - Total width of the bar in characters
///
/// # Returns
/// A string containing the visual bar like [████░░░░]
fn create_bar(percentage: f32, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);

    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bar_full() {
        let bar = create_bar(100.0, 10);
        assert_eq!(bar, "[██████████]");
    }

    #[test]
    fn test_create_bar_empty() {
        let bar = create_bar(0.0, 10);
        assert_eq!(bar, "[░░░░░░░░░░]");
    }

    #[test]
    fn test_create_bar_half() {
        let bar = create_bar(50.0, 10);
        assert_eq!(bar, "[█████░░░░░]");
    }
}
