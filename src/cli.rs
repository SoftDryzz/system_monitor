// CLI argument parsing module
// This module handles command-line argument parsing using clap

use clap::Parser;

/// System Monitor - A lightweight system monitoring tool
#[derive(Parser, Debug)]
#[command(
    name = "sysmon",
    version = "0.4.0",
    author = "SoftDryzz",
    about = "A cross-platform CLI system monitoring tool",
    long_about = None
)]
pub struct Args {
    /// Enable watch mode (continuous updates)
    #[arg(short, long)]
    pub watch: bool,

    /// Update interval in seconds (default: 1, only works with --watch)
    #[arg(short, long, default_value_t = 1)]
    pub interval: u64,

    /// Show detailed information (all CPU cores, more processes)
    #[arg(short, long)]
    pub detailed: bool,
}

/// Parse command-line arguments
pub fn parse_args() -> Args {
    Args::parse()
}
