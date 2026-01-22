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
///
/// # Arguments
/// * `monitor` - System monitor instance
/// * `detailed` - If true, shows all cores; if false, shows top 3
pub fn print_cpu_info(monitor: &SystemMonitor, detailed: bool) {
    let cpu_info = monitor.cpu_info();
    let cpu_count = monitor.cpu_count();
    let bar = create_bar(cpu_info.global_usage, 20);

    println!(
        "CPU:  {:.1}% ({} cores)  {}",
        cpu_info.global_usage, cpu_count, bar
    );

    if detailed {
        // Show all cores
        for core in &cpu_info.cores {
            let core_bar = create_bar(core.usage, 15);
            println!(
                "  Core {:2}:  {:5.1}%  {}",
                core.index, core.usage, core_bar
            );
        }
    } else {
        // Show only top 3 busiest cores
        let top_cores = monitor.top_cpu_cores(3);
        if !top_cores.is_empty() {
            print!("  Top 3:");
            for (index, usage) in top_cores {
                print!(" Core {} ({:.0}%)", index, usage);
            }
            println!();
        }
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

/// Print disk usage information
pub fn print_disk_info(monitor: &SystemMonitor) {
    let disks = monitor.disks_info();

    if disks.is_empty() {
        return;
    }

    println!();
    println!("Disk Usage:");

    for disk in disks {
        let bar = create_bar(disk.percentage as f32, 15);

        // Format mount point (limit to 8 chars for alignment)
        let mount = if disk.mount_point.len() > 8 {
            format!("{}...", &disk.mount_point[..5])
        } else {
            disk.mount_point.clone()
        };

        println!(
            "  {:8} {:6.1}/{:6.1} GB ({:5.1}%)  {}",
            mount, disk.used_gb, disk.total_gb, disk.percentage, bar
        );
    }
}

/// Print top processes by CPU usage
///
/// # Arguments
/// * `monitor` - System monitor instance
/// * `detailed` - If true, shows 10 processes; if false, shows 5
pub fn print_top_processes_cpu(monitor: &SystemMonitor, detailed: bool) {
    let count = if detailed { 10 } else { 5 };
    let processes = monitor.top_processes_by_cpu(count);

    if processes.is_empty() {
        return;
    }

    println!();
    println!("Top {} Processes (by CPU):", count);

    for (i, proc) in processes.iter().enumerate() {
        // Format memory
        let mem_str = if proc.memory_mb >= 1024.0 {
            format!("{:.1} GB", proc.memory_mb / 1024.0)
        } else {
            format!("{:.0} MB", proc.memory_mb)
        };

        println!(
            "  {:2}. {:20}  PID {:5}  {:5.1}%  {:>8}",
            i + 1,
            truncate_string(&proc.name, 20),
            proc.pid,
            proc.cpu_usage,
            mem_str
        );
    }
}

/// Print top processes by memory usage
///
/// # Arguments
/// * `monitor` - System monitor instance  
/// * `detailed` - If true, shows 5 processes; if false, shows 3
pub fn print_top_processes_memory(monitor: &SystemMonitor, detailed: bool) {
    let count = if detailed { 5 } else { 3 };
    let processes = monitor.top_processes_by_memory(count);

    if processes.is_empty() {
        return;
    }

    println!();
    println!("Top {} Processes (by Memory):", count);

    for (i, proc) in processes.iter().enumerate() {
        // Format memory
        let mem_str = if proc.memory_mb >= 1024.0 {
            format!("{:.1} GB", proc.memory_mb / 1024.0)
        } else {
            format!("{:.0} MB", proc.memory_mb)
        };

        println!(
            "  {:2}. {:20}  PID {:5}  {:5.1}%  {:>8}",
            i + 1,
            truncate_string(&proc.name, 20),
            proc.pid,
            proc.cpu_usage,
            mem_str
        );
    }
}

/// Truncate string to max length
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
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
