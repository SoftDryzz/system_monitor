// Process monitoring module
// Provides information about running processes

use sysinfo::{Pid, System};

/// Individual process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub cpu_usage: f32,
    pub memory_mb: f64,
}

impl ProcessInfo {
    /// Create process info from sysinfo Process
    pub fn new(pid: Pid, process: &sysinfo::Process) -> Self {
        const MB: f64 = 1_048_576.0; // 1024^2

        ProcessInfo {
            name: process.name().to_string_lossy().to_string(),
            pid: pid.as_u32(),
            cpu_usage: process.cpu_usage(),
            memory_mb: process.memory() as f64 / MB,
        }
    }
}

/// Get top N processes sorted by CPU usage
pub fn get_top_processes_by_cpu(sys: &System, n: usize) -> Vec<ProcessInfo> {
    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo::new(*pid, process))
        .collect();

    // Sort by CPU usage (descending)
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    // Take top N
    processes.into_iter().take(n).collect()
}

/// Get top N processes sorted by memory usage
pub fn get_top_processes_by_memory(sys: &System, n: usize) -> Vec<ProcessInfo> {
    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo::new(*pid, process))
        .collect();

    // Sort by memory usage (descending)
    processes.sort_by(|a, b| b.memory_mb.partial_cmp(&a.memory_mb).unwrap());

    // Take top N
    processes.into_iter().take(n).collect()
}

/// Get top 3 busiest CPU cores
pub fn get_top_cores(cpus: &[sysinfo::Cpu], n: usize) -> Vec<(usize, f32)> {
    let mut cores: Vec<(usize, f32)> = cpus
        .iter()
        .enumerate()
        .map(|(index, cpu)| (index, cpu.cpu_usage()))
        .collect();

    // Sort by usage (descending)
    cores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Take top N
    cores.into_iter().take(n).collect()
}
