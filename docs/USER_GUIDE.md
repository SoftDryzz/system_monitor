# System Monitor - Complete User Guide

**Version 0.4.0**

---

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Features & Usage](#features--usage)
4. [Color Coding](#color-coding)
5. [Network Statistics](#network-statistics)
6. [Watch Mode](#watch-mode)
7. [Development](#development)
8. [Troubleshooting](#troubleshooting)
9. [Contributing](#contributing)

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
# Single snapshot with colors
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
Shows CPU percentage and top 3 busiest cores with color coding.

```bash
sysmon
```

**Output:**
```
CPU:  34.5% (8 cores)  [██████░░░░░░░░░░░░░░]
      ↑ Yellow (moderate usage)
  Top 3: Core 0 (52%) Core 4 (48%) Core 7 (46%)
         ↑ Red     ↑ Yellow  ↑ Yellow
```

#### Detailed Mode
Shows all individual cores with colors.

```bash
sysmon --detailed
```

---

### 2. Memory Monitoring

Displays RAM usage with visual bar and color coding.

```
Memory:  8.34/16.00 GB (52.1%)  [██████████░░░░░░░░░░]
         ↑ Yellow (moderate)
```

---

### 3. Disk Usage

Shows space usage for all mounted drives with colors.

```
Disk Usage:
  C:\       450.0/1000.0 GB ( 45.0%)  [███████░░░░░░░░]  ← Yellow
  D:\       850.0/1000.0 GB ( 85.0%)  [█████████████░░]  ← Red (high!)
```

---

### 4. Process Monitoring

#### Top Processes by CPU (color-coded)
```
Top 5 Processes (by CPU):
   1. firefox.exe        PID  1234   25.2%    2.5 GB  ← Red
   2. chrome.exe         PID  5678   15.1%    1.8 GB
   3. Code.exe           PID  9012   10.4%    1.2 GB  ← Green
```

#### Top Processes by Memory (color-coded)
```
Top 3 Processes (by Memory):
   1. chrome.exe         PID  5678   15.1%    2.8 GB  ← Red (>2GB)
   2. firefox.exe        PID  1234   25.2%    2.5 GB  ← Red
   3. Code.exe           PID  9012   10.4%    512 MB  ← Green (<512MB)
```

---

### 5. Network Statistics

Real-time network traffic monitoring with speed calculation.

```
Network:
  ↓ Download: 15.2 MB/s   ← Green (active download)
  ↑ Upload:   2.3 MB/s    ← Green (active upload)
  Total RX:   45.7 GB
  Total TX:   12.3 GB
```

**Features:**
- Real-time download/upload speeds
- Automatic unit conversion (B/s, KB/s, MB/s, GB/s)
- Total bytes received (RX) and transmitted (TX)
- Color indicators for active network usage

**How Speed is Calculated:**
1. First reading captures current totals
2. Subsequent readings calculate delta over time interval
3. Speed = (bytes_now - bytes_before) / time_elapsed

---

### 6. System Uptime

Shows time since last boot.

```
Uptime: 2 days, 0 hours, 32 minutes
```

---

## Color Coding

### Usage-Based Colors

System Monitor uses a consistent color scheme across all metrics:

| Color | Percentage | Meaning | Action |
|-------|------------|---------|--------|
| 🟢 **Green** | 0-30% | Healthy | All good |
| 🟡 **Yellow** | 30-70% | Moderate | Normal usage |
| 🔴 **Red** | 70-100% | High | May need attention |

### Where Colors Are Applied

| Component | What's Colored |
|-----------|----------------|
| CPU | Global percentage, per-core percentages |
| Memory | Usage percentage |
| Disk | Usage percentage per drive |
| Processes | CPU percentage, Memory size |
| Network | Speed indicators (green when active) |
| Headers | Section titles (cyan) |

### Memory-Specific Colors (Processes)

For process memory display:
- 🟢 Green: < 512 MB
- 🟡 Yellow: 512 MB - 2 GB
- 🔴 Red: > 2 GB

### Network-Specific Colors

- 🟢 Green: Download > 10 MB/s or Upload > 1 MB/s (active)
- ⚪ White: Low or no activity

### Terminal Compatibility

Colors work best on:
- **Windows Terminal** ⭐⭐⭐⭐⭐
- **PowerShell 7+** ⭐⭐⭐⭐
- **Git Bash** ⭐⭐⭐
- **Linux terminals** ⭐⭐⭐⭐⭐
- **macOS Terminal** ⭐⭐⭐⭐
- **CMD** ⭐⭐ (basic support)

If colors don't display, the `colored` crate falls back gracefully.

---

## Network Statistics

### Understanding Network Output

```
Network:
  ↓ Download: 5.2 MB/s     # Current download speed
  ↑ Upload:   1.3 MB/s     # Current upload speed
  Total RX:   2.5 GB       # Total received since boot
  Total TX:   850 MB       # Total transmitted since boot
```

### Speed Units

Automatically converted based on value:
- **B/s** - Bytes per second (< 1 KB/s)
- **KB/s** - Kilobytes per second (< 1 MB/s)
- **MB/s** - Megabytes per second (< 1 GB/s)
- **GB/s** - Gigabytes per second (very fast connections)

### Total Units

- **B** - Bytes
- **KB** - Kilobytes
- **MB** - Megabytes
- **GB** - Gigabytes
- **TB** - Terabytes

### Notes

- First reading shows 0 speed (needs delta)
- Aggregates all network interfaces
- Updates with each refresh in watch mode

---

## Watch Mode

### Basic Watch

```bash
sysmon --watch
```

Updates every 1 second by default.

### Custom Interval

```bash
sysmon --watch --interval 3
sysmon -w -i 5
```

### Watch + Detailed

```bash
sysmon --watch --detailed
sysmon -w -d
```

### Exit Watch Mode

Press **Ctrl+C** for clean exit.

### Network in Watch Mode

Watch mode is ideal for network monitoring:
- Speed updates with each refresh
- Shows real-time bandwidth usage
- Delta calculation becomes accurate after first interval

---

## Development

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run
cargo run
cargo run -- --watch --detailed
```

### Testing

```bash
cargo test
cargo clippy
cargo fmt
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
│   │   ├── network.rs          # Network monitoring
│   │   ├── process.rs          # Process monitoring
│   │   └── system.rs           # System facade
│   └── display/
│       ├── mod.rs
│       └── formatter.rs        # Output + colors
├── Cargo.toml                  # Dependencies
├── README.md                   # Documentation
├── CHANGELOG.md                # Version history
└── LICENSE                     # MIT License
```

### Key Dependencies

| Crate | Purpose |
|-------|---------|
| sysinfo | System information |
| clap | CLI parsing |
| crossterm | Terminal control |
| ctrlc | Signal handling |
| colored | Terminal colors |

---

## Troubleshooting

### Colors Not Showing

**Windows:**
- Use Windows Terminal (recommended)
- Or enable ANSI in CMD: `reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1`

**Linux/macOS:**
- Usually works out of the box
- Try: `export TERM=xterm-256color`

### Network Speed Shows 0

- Normal on first reading
- Wait for second refresh (delta calculation)
- In watch mode, accurate after first interval

### High CPU Usage by sysmon

- Use longer interval: `sysmon -w -i 5`
- Default 1s is fine for most systems

### Permission Denied (Linux/macOS)

```bash
chmod +x target/release/sysmon
```

### Build Errors

```bash
rustup update
cargo clean
cargo build --release
```

---

## Contributing

### How to Contribute

1. **Fork** the repository
2. **Create branch**: `git checkout -b feature/amazing`
3. **Make changes**
4. **Test**: `cargo test && cargo clippy && cargo fmt`
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

---

## Roadmap

### v0.5.0 (Next)
- Export to JSON/CSV
- Configuration file
- Custom color themes
- Historical data tracking

### v1.0.0 (Stable)
- All core features complete
- Full documentation
- Comprehensive tests

---

## FAQ

**Q: Why are colors important?**
A: Instant visual feedback - see problems at a glance without reading numbers.

**Q: Can I disable colors?**
A: Not currently. Feature planned for v0.5.0 config file.

**Q: Network speed seems wrong?**
A: First reading is always 0. Wait for delta calculation.

**Q: Does it work on ARM?**
A: Yes, Rust and sysinfo support ARM (Raspberry Pi, M1/M2).

**Q: How much overhead?**
A: < 1% CPU, ~5-10 MB RAM. Minimal impact.

---

## Resources

### Learning Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Dependencies Docs
- [sysinfo](https://docs.rs/sysinfo/)
- [clap](https://docs.rs/clap/)
- [colored](https://docs.rs/colored/)

---

## Support

- **Issues**: [GitHub Issues](https://github.com/SoftDryzz/system_monitor/issues)
- **Author**: [@SoftDryzz](https://github.com/SoftDryzz)

---

**Happy Monitoring! 📊🎨**

Version 0.4.0 | [GitHub](https://github.com/SoftDryzz/system_monitor) | [Changelog](CHANGELOG.md)