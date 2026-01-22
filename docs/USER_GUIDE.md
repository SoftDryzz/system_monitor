# System Monitor - Complete User Guide

**Version 0.3.0**

---

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Features & Usage](#features--usage)
4. [Understanding Output](#understanding-output)
5. [Watch Mode](#watch-mode)
6. [Development](#development)
7. [Troubleshooting](#troubleshooting)
8. [Contributing](#contributing)

---

## Installation

### Prerequisites

- **Rust 1.70+**: [Install from rustup.rs](https://rustup.rs/)
- **Git** (optional): For cloning the repository

### Method 1: Build from Source

```bash
git clone https://github.com/SoftDryzz/system_monitor.git
cd system_monitor
cargo build --release
# Binary: target/release/sysmon
```

### Method 2: Download Binary

Download pre-compiled binaries from [Releases](https://github.com/SoftDryzz/system_monitor/releases):
- `sysmon-windows-x86_64.exe` (Windows)
- `sysmon-linux-x86_64` (Linux)
- `sysmon-macos-x86_64` (macOS)

### Method 3: Install with Cargo

```bash
cargo install --path .
# Installs to ~/.cargo/bin (must be in PATH)
```

---

## Quick Start

### Basic Commands

```bash
# Single snapshot (compact mode)
sysmon

# Detailed view (all cores, more processes)
sysmon --detailed

# Watch mode (continuous updates)
sysmon --watch

# Watch with custom interval
sysmon --watch --interval 3

# Watch detailed mode
sysmon --watch --detailed

# Show help
sysmon --help
```

---

## Features & Usage

### 1. CPU Monitoring

#### Compact Mode (Default)
Shows CPU percentage and top 3 busiest cores.

```bash
sysmon
```

**Output:**
```
CPU:  34.5% (8 cores)  [██████░░░░░░░░░░░░░░]
  Top 3: Core 0 (52%) Core 4 (48%) Core 7 (46%)
```

**Why Compact?**
- Works great with 4-128+ cores
- Shows what matters most
- Fits on one screen

#### Detailed Mode
Shows all individual cores.

```bash
sysmon --detailed
```

**Output:**
```
CPU:  34.5% (8 cores)  [██████░░░░░░░░░░░░░░]
  Core  0:   52.0%  [███████████░░░░]
  Core  1:   48.0%  [██████████░░░░░]
  Core  2:   35.0%  [███████░░░░░░░░]
  ...all 8 cores shown...
```

---

### 2. Memory Monitoring

Displays RAM usage with visual bar.

```
Memory:  8.34/16.00 GB (52.1%)  [██████████░░░░░░░░░░]
```

**Interpretation:**
- < 50%: Healthy
- 50-80%: Normal usage
- \> 80%: High usage (may slow down system)

---

### 3. Disk Usage

Shows space usage for all mounted drives.

```
Disk Usage:
  C:\       450.0/1000.0 GB ( 45.0%)  [███████░░░░░░░░]
  D:\       200.0/ 500.0 GB ( 40.0%)  [██████░░░░░░░░░]
```

**Notes:**
- Filters out small virtual drives (< 1GB)
- Includes all physical and network drives
- GB formatting for easy reading

---

### 4. Process Monitoring

#### Top Processes by CPU

**Compact Mode** (5 processes):
```
Top 5 Processes (by CPU):
   1. firefox.exe        PID  1234   25.2%    2.5 GB
   2. chrome.exe         PID  5678   15.1%    1.8 GB
   3. Code.exe           PID  9012   10.4%    1.2 GB
   4. Discord.exe        PID  3456    8.3%    800 MB
   5. Spotify.exe        PID  7890    5.2%    650 MB
```

**Detailed Mode** (10 processes):
```bash
sysmon --detailed
```

#### Top Processes by Memory

**Compact Mode** (3 processes):
```
Top 3 Processes (by Memory):
   1. chrome.exe         PID  5678   15.1%    2.8 GB
   2. firefox.exe        PID  1234   25.2%    2.5 GB
   3. Code.exe           PID  9012   10.4%    1.2 GB
```

**Detailed Mode** (5 processes):
```bash
sysmon --detailed
```

**Understanding Process Info:**
- **Name**: Process executable name
- **PID**: Process ID
- **CPU%**: Current CPU usage
- **Memory**: RAM usage (auto MB/GB)

---

### 5. System Uptime

Shows time since last boot.

```
Uptime: 2 days, 0 hours, 32 minutes
```

**Why It Matters:**
- Check system stability
- Know when restart is needed
- Troubleshoot issues

---

### 6. Watch Mode

Continuous monitoring with real-time updates.

#### Basic Watch

```bash
sysmon --watch
```

**Behavior:**
- Updates every 1 second (default)
- Clears screen between updates
- Shows "Watch" in header
- Press **Ctrl+C** to exit cleanly

#### Custom Interval

```bash
# Update every 3 seconds
sysmon --watch --interval 3

# Update every 5 seconds
sysmon -w -i 5
```

**Recommended Intervals:**
- 1s: Real-time monitoring
- 2-3s: Normal usage
- 5s+: Low-impact monitoring

#### Watch + Detailed

```bash
sysmon --watch --detailed
```

Combines continuous updates with full information.

---

## Understanding Output

### Visual Bars

```
[██████████░░░░░░░░░░]
 ↑ filled  ↑ empty
```

- **█**: Used/occupied
- **░**: Free/available
- Length: 20 characters (100% = 20 filled)

### Percentages

- **0-30%**: Low (green zone)
- **30-70%**: Moderate (yellow zone)
- **70-100%**: High (red zone)

### Memory Units

- **MB**: < 1024 MB
- **GB**: ≥ 1024 MB

Auto-converts for readability.

---

## Watch Mode Technical Details

### How It Works

```
1. Parse arguments
2. Setup Ctrl+C handler
3. Loop:
   a. Clear screen
   b. Refresh system data
   c. Display info
   d. Sleep interval
   e. Check Ctrl+C
4. Exit cleanly
```

### Ctrl+C Handling

- **Responsive**: Checks every 100ms
- **Graceful**: Prints exit message
- **Thread-safe**: Uses atomic boolean

### Screen Clearing

**Platform-specific implementation:**

- **Windows**: Uses `cmd /c cls`
- **Linux/macOS**: Uses ANSI escape codes

Works across all major terminals:
- PowerShell
- Windows Terminal
- Git Bash
- Bash/Zsh
- Terminal.app

---

## Development

### Build Commands

```bash
# Debug build (fast compilation)
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check

# Run
cargo run

# Run with arguments
cargo run -- --watch --detailed
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Specific test
cargo test test_create_bar
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check format
cargo fmt -- --check
```

### Project Structure

```
system_monitor/
├── src/
│   ├── main.rs                 # Entry point
│   ├── cli.rs                  # Argument parsing
│   ├── monitor/
│   │   ├── mod.rs              # Module exports
│   │   ├── cpu.rs              # CPU monitoring
│   │   ├── memory.rs           # Memory monitoring
│   │   ├── disk.rs             # Disk monitoring
│   │   ├── process.rs          # Process monitoring
│   │   └── system.rs           # System facade
│   └── display/
│       ├── mod.rs
│       └── formatter.rs        # Output formatting
├── Cargo.toml                  # Dependencies
├── README.md                   # Documentation
└── LICENSE                     # MIT License
```

### Key Rust Concepts

#### Ownership & Borrowing
```rust
// monitor borrows SystemMonitor
pub fn print_cpu_info(monitor: &SystemMonitor, detailed: bool) {
    // Read-only access
}
```

#### Modules
```rust
mod monitor;        // Declare module
use monitor::cpu;   // Import from module
pub mod cpu;        // Public module
```

#### Pattern Matching
```rust
if args.watch {
    watch_mode(...);
} else {
    single_snapshot(...);
}
```

---

## Troubleshooting

### Installation Issues

#### "cargo: command not found"

**Solution**: Add Rust to PATH

**Linux/macOS:**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
source ~/.bashrc  # or ~/.zshrc
```

**Windows:**
Add to PATH: `%USERPROFILE%\.cargo\bin`

#### Build Fails

**Solution 1**: Update Rust
```bash
rustup update
```

**Solution 2**: Clean and rebuild
```bash
cargo clean
cargo build --release
```

---

### Runtime Issues

#### Inaccurate First Reading

**Cause**: System info needs warmup
**Solution**: Normal behavior, wait 1-2 seconds

#### Screen Not Clearing (Watch Mode)

**Cause**: Terminal doesn't support clearing
**Solution**: Try different terminal:
- Windows Terminal (recommended)
- PowerShell 7+
- Git Bash

#### High CPU Usage

**Cause**: Too fast refresh interval
**Solution**: Use longer interval
```bash
sysmon --watch --interval 2
```

#### Permission Denied (Linux/macOS)

**Solution**: Make executable
```bash
chmod +x target/release/sysmon
```

---

## Contributing

### How to Contribute

1. **Fork** the repository
2. **Create branch**: `git checkout -b feature/amazing`
3. **Make changes**
4. **Test**: `cargo test && cargo clippy`
5. **Commit**: `git commit -m 'feat: add amazing feature'`
6. **Push**: `git push origin feature/amazing`
7. **Pull Request**

### Commit Convention

Follow [Conventional Commits](https://conventionalcommits.org):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `refactor:` Code refactoring
- `test:` Tests
- `chore:` Maintenance

**Examples:**
```bash
feat: add network statistics
fix: correct memory calculation on Windows
docs: update installation guide
refactor: simplify CPU display logic
```

### Code Standards

- Format: `cargo fmt`
- Lint: `cargo clippy`
- Test: `cargo test`
- Document: Use `///` comments

---

## Roadmap

### v0.4.0 (Next)
- Network statistics (download/upload speed)
- Export to JSON/CSV
- Color-coded warnings
- Process filtering

### v0.5.0 (Future)
- Interactive TUI mode
- Historical data tracking
- Configuration file
- Custom themes

### v1.0.0 (Stable)
- All core features complete
- Full documentation
- Comprehensive tests
- Performance optimizations

---

## Resources

### Learning Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Dependencies
- [sysinfo](https://docs.rs/sysinfo/) - System information
- [clap](https://docs.rs/clap/) - CLI parsing
- [crossterm](https://docs.rs/crossterm/) - Terminal control
- [ctrlc](https://docs.rs/ctrlc/) - Signal handling

### Similar Projects
- [htop](https://htop.dev/) - Interactive process viewer
- [bottom](https://github.com/ClementTsang/bottom) - Rust system monitor
- [gtop](https://github.com/aksakalli/gtop) - Node.js system monitor

---

## Support

- **Issues**: [GitHub Issues](https://github.com/SoftDryzz/system_monitor/issues)
- **Discussions**: [GitHub Discussions](https://github.com/SoftDryzz/system_monitor/discussions)
- **Author**: [@SoftDryzz](https://github.com/SoftDryzz)

---

## FAQ

**Q: Why Rust?**
A: Maximum performance, memory safety, zero-cost abstractions.

**Q: Does it support ARM processors?**
A: Yes, Rust and sysinfo support ARM (Raspberry Pi, M1/M2 Macs).

**Q: Can I monitor remote systems?**
A: Not yet. Planned for future versions.

**Q: How much overhead does it add?**
A: < 1% CPU, ~5-10 MB RAM. Minimal impact.

**Q: Is it better than Task Manager?**
A: Different use case. Lightweight, scriptable, cross-platform.

---

**Happy Monitoring! 📊**

Version 0.3.0 | [GitHub](https://github.com/SoftDryzz/system_monitor) | [[Changelog]([\CHANGELOG.md](https://github.com/SoftDryzz/system_monitor/blob/main/CHANGELOG.md))
