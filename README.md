# System Monitor 📊

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.4.0-green.svg)](https://github.com/SoftDryzz/system_monitor/releases)

**System Monitor** is a lightweight, cross-platform CLI tool for real-time system monitoring with intelligent scaling and color-coded feedback. Built with Rust for maximum performance and minimal resource usage.

[English](README.md) | [Español](README_ES.md)

---

## ✨ Features

* 🖥️ **CPU Monitoring** - Intelligent display with compact/detailed modes
* 💾 **Memory Tracking** - RAM usage with visual indicators
* 💽 **Disk Usage** - Space usage for all mounted drives
* 📊 **Process Monitoring** - Top processes by CPU and memory
* 🌐 **Network Statistics** - Real-time download/upload speeds
* 🎨 **Color-Coded UI** - Green/Yellow/Red based on usage levels
* ⏱️ **System Uptime** - Track system runtime
* 🔄 **Watch Mode** - Continuous real-time updates
* 🚀 **Blazing Fast** - Native Rust performance
* 🌍 **Cross-Platform** - Windows, Linux, and macOS

---

## 🚀 Quick Start

### Installation

```bash
# Clone and build
git clone https://github.com/SoftDryzz/system_monitor.git
cd system_monitor
cargo build --release

# Run
./target/release/sysmon
```

Or download pre-compiled binaries from [Releases](https://github.com/SoftDryzz/system_monitor/releases).

### Basic Usage

```bash
# Single snapshot with colors
sysmon

# Detailed view (all CPU cores, more processes)
sysmon --detailed

# Watch mode (continuous updates)
sysmon --watch

# Watch mode with details and custom interval
sysmon --watch --detailed --interval 2
```

---

## 📊 Example Output

```
╭─────────────────────────────────────────────────────────╮
│   System Monitor v0.4.0                                 │
╰─────────────────────────────────────────────────────────╯

CPU:  34.5% (8 cores)  [██████░░░░░░░░░░░░░░]    ← Yellow (moderate)
  Top 3: Core 0 (52%) Core 4 (48%) Core 7 (46%)

Memory:  8.34/16.00 GB (52.1%)  [██████████░░░░░░░░░░]  ← Yellow

Disk Usage:
  C:\       450.0/1000.0 GB ( 45.0%)  [███████░░░░░░░░]  ← Yellow
  D:\       200.0/ 500.0 GB ( 40.0%)  [██████░░░░░░░░░]  ← Yellow

Network:
  ↓ Download: 5.2 MB/s     ← Green (active)
  ↑ Upload:   1.3 MB/s     ← Green (active)
  Total RX:   2.5 GB
  Total TX:   850 MB

Top 5 Processes (by CPU):
   1. firefox.exe        PID  1234   25.2%    2.5 GB  ← Red (high CPU)
   2. chrome.exe         PID  5678   15.1%    1.8 GB
   3. Code.exe           PID  9012   10.4%    1.2 GB

Top 3 Processes (by Memory):
   1. chrome.exe         PID  5678   15.1%    2.8 GB  ← Red (high memory)
   2. firefox.exe        PID  1234   25.2%    2.5 GB

Uptime: 2 days, 0 hours, 32 minutes
```

---

## 🎨 Color Coding

| Color | Usage Level | Meaning |
|-------|-------------|---------|
| 🟢 Green | 0-30% | Healthy, low usage |
| 🟡 Yellow | 30-70% | Normal, moderate usage |
| 🔴 Red | 70-100% | High usage, attention needed |

**Applied to:** CPU, Memory, Disk, and Process metrics

---

## 💻 Commands

| Command | Description |
|---------|-------------|
| `sysmon` | Compact view with colors |
| `sysmon --detailed` | Detailed view (all cores) |
| `sysmon --watch` | Continuous updates |
| `sysmon -w -d -i 3` | Watch detailed, 3s interval |
| `sysmon --help` | Show help |
| `sysmon --version` | Show version |

---

## 📦 What's New in v0.4.0

### 🌐 Network Statistics
- Real-time download/upload speeds
- Automatic unit conversion (B/s → GB/s)
- Total bytes received/transmitted

### 🎨 Color-Coded UI
- Visual health indicators throughout
- Instant feedback on system status
- Professional terminal appearance

[Full Changelog](CHANGELOG.md) | [User Guide](USER_GUIDE.md)

---

## 🗺️ Version History

| Version | Features |
|---------|----------|
| **v0.4.0** | Network stats, Color-coded UI |
| v0.3.0 | Process monitoring, Intelligent scaling |
| v0.2.1 | Disk usage monitoring |
| v0.2.0 | Watch mode, Continuous updates |
| v0.1.0 | Initial release |

---

## 🛠️ Development

### Build
```bash
cargo build --release
```

### Test
```bash
cargo test
cargo clippy
cargo fmt
```

### Project Structure
```
src/
├── main.rs              # Entry point
├── cli.rs               # CLI parsing
├── monitor/
│   ├── cpu.rs           # CPU monitoring
│   ├── memory.rs        # Memory monitoring
│   ├── disk.rs          # Disk monitoring
│   ├── network.rs       # Network monitoring
│   ├── process.rs       # Process monitoring
│   └── system.rs        # System facade
└── display/
    └── formatter.rs     # Output formatting + colors
```

---

## 🤝 Contributing

Contributions welcome! See [USER_GUIDE.md](USER_GUIDE.md) for details.

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing`
3. Commit changes: `git commit -m 'feat: add amazing feature'`
4. Push and create Pull Request

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file.

---

## 👤 Author

**SoftDryzz**
- GitHub: [@SoftDryzz](https://github.com/SoftDryzz)
- Project: [system_monitor](https://github.com/SoftDryzz/system_monitor)

---

## 🙏 Acknowledgments

- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information
- [clap](https://github.com/clap-rs/clap) - CLI parsing
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [colored](https://github.com/colored-rs/colored) - Terminal colors
- [ctrlc](https://github.com/Detegr/rust-ctrlc) - Signal handling

---

**⭐ Star this project if you find it useful!**