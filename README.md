# System Monitor 📊

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.3.0-green.svg)](https://github.com/SoftDryzz/system_monitor/releases)

**System Monitor** is a lightweight, cross-platform CLI tool for real-time system monitoring with intelligent scaling. Built with Rust for maximum performance and minimal resource usage.

[English](README.md) | [Español](README_ES.md)

---

## ✨ Features

* 🖥️ **CPU Monitoring** - Intelligent display with compact/detailed modes
* 💾 **Memory Tracking** - RAM usage with visual indicators
* 💽 **Disk Usage** - Space usage for all mounted drives
* 📊 **Process Monitoring** - Top processes by CPU and memory
* ⏱️ **System Uptime** - Track system runtime
* 🔄 **Watch Mode** - Continuous real-time updates
* 🎨 **Beautiful CLI** - Formatted with progress bars
* 🚀 **Blazing Fast** - Native Rust performance
* 🌐 **Cross-Platform** - Windows, Linux, and macOS

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
# Single snapshot (compact mode)
sysmon

# Detailed view (all CPU cores, more processes)
sysmon --detailed

# Watch mode (continuous updates)
sysmon --watch

# Watch mode with details
sysmon --watch --detailed --interval 2
```

---

## 📊 Example Output

### Compact Mode (Default)
```
╭─────────────────────────────────────────────────────────╮
│   System Monitor v0.3.0                                 │
╰─────────────────────────────────────────────────────────╯

CPU:  34.5% (8 cores)  [██████░░░░░░░░░░░░░░]
  Top 3: Core 0 (52%) Core 4 (48%) Core 7 (46%)

Memory:  8.34/16.00 GB (52.1%)  [██████████░░░░░░░░░░]

Disk Usage:
  C:\       450.0/1000.0 GB ( 45.0%)  [███████░░░░░░░░]

Top 5 Processes (by CPU):
   1. firefox.exe        PID  1234   25.2%    2.5 GB
   2. chrome.exe         PID  5678   15.1%    1.8 GB
   3. Code.exe           PID  9012   10.4%    1.2 GB

Top 3 Processes (by Memory):
   1. chrome.exe         PID  5678   15.1%    2.8 GB
   2. firefox.exe        PID  1234   25.2%    2.5 GB

Uptime: 2 days, 0 hours, 32 minutes
```

---

## 💻 Commands

| Command | Description |
|---------|-------------|
| `sysmon` | Compact view (default) |
| `sysmon --detailed` | Detailed view (all cores) |
| `sysmon --watch` | Continuous updates |
| `sysmon -w -i 3` | Watch with 3s interval |
| `sysmon --help` | Show help |
| `sysmon --version` | Show version |

---

## 📦 What's New in v0.3.0

### 🎉 Major Features
- **Process Monitoring**: Top processes by CPU and memory
- **Intelligent CPU Display**: Compact mode for high-core systems
- **Detailed Mode**: Optional verbose output with `--detailed` flag

### 🔧 Improvements
- Smart scaling for systems with 4 to 128+ cores
- Better memory formatting (MB/GB auto-conversion)
- Enhanced CLI arguments

[Full Changelog](CHANGELOG.md) | [User Guide](USER_GUIDE.md)

---

## 🗺️ Version History

- **v0.3.0** (Current) - Process monitoring & intelligent scaling
- **v0.2.1** - Disk usage monitoring
- **v0.2.0** - Watch mode & continuous updates
- **v0.1.0** - Initial release

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
├── monitor/             # System monitoring
│   ├── cpu.rs
│   ├── memory.rs
│   ├── disk.rs
│   ├── process.rs
│   └── system.rs
└── display/
    └── formatter.rs     # Output formatting
```

---

## 🤝 Contributing

Contributions welcome! See [USER_GUIDE.md](USER_GUIDE.md) for details.

### Quick Guide
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
- [ctrlc](https://github.com/Detegr/rust-ctrlc) - Signal handling

---

**⭐ Star this project if you find it useful!**
