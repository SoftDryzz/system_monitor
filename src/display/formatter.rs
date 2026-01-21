// Formatter module - Visual output formatting utilities
// Provides functions to display system information in a beautiful format

use crate::monitor::system::SystemMonitor;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Print the header banner
pub fn print_header() {
    println!("╭─────────────────────────────────────╮");
    println!("│   System Monitor v{}              │", VERSION);
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
pub fn print_footer() {
    println!();
    println!("Press Ctrl+C to exit");
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
