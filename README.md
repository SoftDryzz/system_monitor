# System Monitor 📊

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-In%20Development-yellow.svg)]()

**System Monitor** is a lightweight, cross-platform CLI tool for real-time system monitoring. Built with Rust for maximum performance and minimal resource usage.

[English](README.md) | [Español](README_ES.md)

> ⚠️ **In Active Development:** This project is under construction. Features and APIs may change.

---

## ✨ Features

* 🖥️ **CPU Monitoring** - Real-time CPU usage per core with visual bars
* 💾 **Memory Tracking** - RAM usage with detailed statistics
* ⏱️ **System Uptime** - Track how long your system has been running
* 🎨 **Beautiful CLI Output** - Formatted with colors and progress bars
* 🚀 **Blazing Fast** - Written in Rust for native performance
* 🌐 **Cross-Platform** - Works on Windows, Linux, and macOS

---

## 📋 Requirements

* Rust 1.70 or higher
* Cargo (comes with Rust)

---

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/SoftDryzz/system_monitor.git
cd system_monitor

# Build the project
cargo build --release

# The binary will be in target/release/sysmon
```

### Quick Install (Linux/macOS)

```bash
# Build and install to ~/.cargo/bin
cargo install --path .

# Now you can run from anywhere
sysmon
```

### Windows

```powershell
# Build
cargo build --release

# The executable will be in target\release\sysmon.exe
# Add to PATH or copy to desired location
```

---

## 💻 Usage

### Basic Usage

```bash
# Show current system metrics
sysmon

# Output example:
╭─────────────────────────────────────╮
│      System Monitor v0.1.0          │
╰─────────────────────────────────────╯

CPU Usage:  34.5%  [██████░░░░░░░░░░░░░░]
  Core  0:   45.2%  [██████░░░░░░░░░]
  Core  1:   28.3%  [████░░░░░░░░░░░]
  Core  2:   31.1%  [████░░░░░░░░░░░]
  Core  3:   33.8%  [█████░░░░░░░░░░]

Memory:     8.34/16.00 GB (52.1%)
            [██████████░░░░░░░░░░]

Uptime:     2 days, 0 hours, 32 minutes
```

### Available Commands

| Command | Description |
|---------|-------------|
| `sysmon` | Display current system metrics |
| `sysmon --help` | Show help information |
| `sysmon --version` | Show version |

---

## 🏗️ Project Structure

```
system_monitor/
├── Cargo.toml              # Project configuration
├── README.md               # This file
├── README_ES.md            # Spanish documentation
├── LICENSE                 # MIT License
├── docs/
│   └── USER_GUIDE.md       # Detailed user guide
└── src/
    ├── main.rs             # Entry point
    ├── cli.rs              # CLI argument parsing
    ├── monitor/            # System monitoring logic
    │   ├── mod.rs
    │   ├── cpu.rs          # CPU monitoring
    │   ├── memory.rs       # Memory monitoring
    │   └── system.rs       # System info
    └── display/            # Output formatting
        ├── mod.rs
        └── formatter.rs    # Display utilities
```

---

## 🛠️ Development

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run

```bash
# Run in development mode
cargo run

# Run release version
cargo run --release
```

### Check

```bash
# Check for errors without building
cargo check

# Run with warnings
cargo clippy
```

---

## 🗺️ Roadmap

### ✅ Version 0.1.0 (Current)
- [x] Basic CPU monitoring
- [x] Memory usage tracking
- [x] System uptime
- [x] Formatted CLI output
- [x] Cross-platform support

### 🔨 Version 0.2.0 (Next)
- [ ] Disk usage monitoring
- [ ] Network statistics
- [ ] Process list (top N by CPU/RAM)
- [ ] Watch mode (continuous updates)
- [ ] CLI arguments with clap

### 🚀 Version 0.3.0 (Future)
- [ ] Interactive TUI (Terminal UI)
- [ ] Export metrics to JSON/CSV
- [ ] Historical data tracking
- [ ] Alerts and notifications
- [ ] Custom refresh intervals

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `test:` Adding tests
- `chore:` Maintenance tasks

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👤 Author

**SoftDryzz**

* GitHub: [@SoftDryzz](https://github.com/SoftDryzz)
* Portfolio: [More projects](https://github.com/SoftDryzz?tab=repositories)

---

## 🙏 Acknowledgments

* [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information library
* [clap](https://github.com/clap-rs/clap) - Command line argument parser
* The Rust community for excellent documentation and tools

---

## 📚 Learn More

* [The Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
* [User Guide](docs/USER_GUIDE.md) - Detailed usage guide
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Rust examples

---

**⭐ If you like this project, give it a star on GitHub!**
