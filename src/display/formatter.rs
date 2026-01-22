// Formatter module - Visual output formatting utilities
// Provides functions to display system information with colors

use crate::monitor::network::NetworkInfo;
use crate::monitor::system::SystemMonitor;
use colored::*;
use std::io::{self, Write};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get color based on percentage (0-100)
fn get_color_for_percentage(percentage: f32) -> Color {
    if percentage < 30.0 {
        Color::Green
    } else if percentage < 70.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Clear the terminal screen
/// Platform-specific implementation for best results
pub fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        if std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .is_err()
        {
            print!("\x1B[2J\x1B[1;1H");
            let _ = io::stdout().flush();
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
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
    println!("╭─────────────────────────────────────────────────────────╮");
    if watch_mode {
        println!(
            "│   System Monitor v{} (Watch)                    │",
            VERSION
        );
        println!(
            "│   Updating every {} second(s)...                       │",
            interval
        );
        println!("│   Press Ctrl+C to exit                              │");
    } else {
        println!(
            "│   System Monitor v{}                              │",
            VERSION
        );
    }
    println!("╰─────────────────────────────────────────────────────────╯");
    println!();
}

/// Print CPU information with visual bars and colors
///
/// # Arguments
/// * `monitor` - System monitor instance
/// * `detailed` - If true, shows all cores; if false, shows top 3
pub fn print_cpu_info(monitor: &SystemMonitor, detailed: bool) {
    let cpu_info = monitor.cpu_info();
    let cpu_count = monitor.cpu_count();
    let bar = create_bar(cpu_info.global_usage, 20);
    let color = get_color_for_percentage(cpu_info.global_usage);

    println!(
        "CPU:  {} ({} cores)  {}",
        format!("{:.1}%", cpu_info.global_usage).color(color).bold(),
        cpu_count,
        bar
    );

    if detailed {
        // Show all cores
        for core in &cpu_info.cores {
            let core_bar = create_bar(core.usage, 15);
            let core_color = get_color_for_percentage(core.usage);
            println!(
                "  Core {:2}:  {}  {}",
                core.index,
                format!("{:5.1}%", core.usage).color(core_color),
                core_bar
            );
        }
    } else {
        // Show only top 3 busiest cores
        let top_cores = monitor.top_cpu_cores(3);
        if !top_cores.is_empty() {
            print!("  Top 3:");
            for (index, usage) in top_cores {
                let core_color = get_color_for_percentage(usage);
                print!(
                    " Core {} ({})",
                    index,
                    format!("{:.0}%", usage).color(core_color)
                );
            }
            println!();
        }
    }
    println!();
}

/// Print memory information with visual bar and colors
pub fn print_memory_info(monitor: &SystemMonitor) {
    let mem_info = monitor.memory_info();
    let bar = create_bar(mem_info.percentage as f32, 20);
    let color = get_color_for_percentage(mem_info.percentage as f32);

    println!(
        "Memory:  {}/{:.2} GB ({})  {}",
        format!("{:.2}", mem_info.used_gb).color(color).bold(),
        mem_info.total_gb,
        format!("{:.1}%", mem_info.percentage).color(color),
        bar
    );
    println!();
}

/// Print disk usage information with colors
pub fn print_disk_info(monitor: &SystemMonitor) {
    let disks = monitor.disks_info();

    if disks.is_empty() {
        return;
    }

    println!("{}", "Disk Usage:".bright_cyan().bold());

    for disk in disks {
        let bar = create_bar(disk.percentage as f32, 15);
        let color = get_color_for_percentage(disk.percentage as f32);

        let mount = if disk.mount_point.len() > 8 {
            format!("{}...", &disk.mount_point[..5])
        } else {
            disk.mount_point.clone()
        };

        println!(
            "  {:8} {:6.1}/{:6.1} GB ({})  {}",
            mount,
            disk.used_gb,
            disk.total_gb,
            format!("{:5.1}%", disk.percentage).color(color),
            bar
        );
    }
    println!();
}

/// Print network statistics with colors
pub fn print_network_info(monitor: &mut SystemMonitor) {
    let net_info = monitor.network_info();

    println!("{}", "Network:".bright_cyan().bold());

    // Download speed
    let (dl_value, dl_unit) = NetworkInfo::format_speed(net_info.download_speed);
    let dl_color = if dl_value > 10.0 {
        Color::Green
    } else {
        Color::White
    };
    println!(
        "  ↓ Download: {}",
        format!("{:.1} {}", dl_value, dl_unit)
            .color(dl_color)
            .bold()
    );

    // Upload speed
    let (ul_value, ul_unit) = NetworkInfo::format_speed(net_info.upload_speed);
    let ul_color = if ul_value > 1.0 {
        Color::Green
    } else {
        Color::White
    };
    println!(
        "  ↑ Upload:   {}",
        format!("{:.1} {}", ul_value, ul_unit)
            .color(ul_color)
            .bold()
    );

    // Total received
    let (rx_value, rx_unit) = NetworkInfo::format_bytes(net_info.total_received);
    println!("  Total RX:   {:.2} {}", rx_value, rx_unit);

    // Total transmitted
    let (tx_value, tx_unit) = NetworkInfo::format_bytes(net_info.total_transmitted);
    println!("  Total TX:   {:.2} {}", tx_value, tx_unit);

    println!();
}

/// Print top processes by CPU usage with colors
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

    println!(
        "{}",
        format!("Top {} Processes (by CPU):", count)
            .bright_cyan()
            .bold()
    );

    for (i, proc) in processes.iter().enumerate() {
        let mem_str = if proc.memory_mb >= 1024.0 {
            format!("{:.1} GB", proc.memory_mb / 1024.0)
        } else {
            format!("{:.0} MB", proc.memory_mb)
        };

        let cpu_color = get_color_for_percentage(proc.cpu_usage);

        println!(
            "  {:2}. {:20}  PID {:5}  {}  {:>8}",
            i + 1,
            truncate_string(&proc.name, 20),
            proc.pid,
            format!("{:5.1}%", proc.cpu_usage).color(cpu_color),
            mem_str
        );
    }
    println!();
}

/// Print top processes by memory usage with colors
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

    println!(
        "{}",
        format!("Top {} Processes (by Memory):", count)
            .bright_cyan()
            .bold()
    );

    for (i, proc) in processes.iter().enumerate() {
        let mem_str = if proc.memory_mb >= 1024.0 {
            format!("{:.1} GB", proc.memory_mb / 1024.0)
        } else {
            format!("{:.0} MB", proc.memory_mb)
        };

        let mem_color = if proc.memory_mb >= 2048.0 {
            Color::Red
        } else if proc.memory_mb >= 512.0 {
            Color::Yellow
        } else {
            Color::Green
        };

        println!(
            "  {:2}. {:20}  PID {:5}  {:5.1}%  {}",
            i + 1,
            truncate_string(&proc.name, 20),
            proc.pid,
            proc.cpu_usage,
            mem_str.color(mem_color).bold()
        );
    }
    println!();
}

/// Print system uptime
pub fn print_uptime(monitor: &SystemMonitor) {
    let uptime = monitor.uptime();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;

    println!(
        "Uptime: {} days, {} hours, {} minutes",
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
        println!(
            "{}",
            "Run with --watch for continuous monitoring".bright_black()
        );
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

/// Truncate string to max length
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
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

    #[test]
    fn test_get_color_green() {
        assert_eq!(get_color_for_percentage(20.0), Color::Green);
    }

    #[test]
    fn test_get_color_yellow() {
        assert_eq!(get_color_for_percentage(50.0), Color::Yellow);
    }

    #[test]
    fn test_get_color_red() {
        assert_eq!(get_color_for_percentage(85.0), Color::Red);
    }
}
