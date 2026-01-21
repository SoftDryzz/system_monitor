# System Monitor - User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Basic Usage](#basic-usage)
4. [Understanding the Output](#understanding-the-output)
5. [Development](#development)
6. [Troubleshooting](#troubleshooting)
7. [Contributing](#contributing)

---

## Introduction

System Monitor (`sysmon`) is a lightweight, cross-platform command-line tool for monitoring system resources. Built with Rust, it provides real-time information about your CPU, memory, and system uptime with minimal resource overhead.

### Key Features

- **Real-time CPU monitoring** per core
- **Memory usage tracking** with visual indicators
- **System uptime** display
- **Cross-platform** support (Windows, Linux, macOS)
- **Lightweight** and fast execution

---

## Installation

### Prerequisites

Before installing System Monitor, ensure you have:

- **Rust 1.70 or higher**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Git** (optional, for cloning): [Install Git](https://git-scm.com/downloads)

### Installation Methods

#### Method 1: Build from Source

```bash
# Clone the repository
git clone https://github.com/SoftDryzz/system_monitor.git
cd system_monitor

# Build the release version
cargo build --release

# The binary will be in target/release/sysmon (or sysmon.exe on Windows)
```

#### Method 2: Install with Cargo

```bash
# Install directly from the project directory
cargo install --path .

# This installs sysmon to ~/.cargo/bin (or %USERPROFILE%\.cargo\bin on Windows)
# Make sure this directory is in your PATH
```

#### Method 3: Download Binary (Future)

Pre-compiled binaries will be available in future releases on GitHub.

---

## Basic Usage

### Running System Monitor

Simply run the command:

```bash
sysmon
```

This will display a snapshot of your system's current state.

### Command-Line Options

```bash
# Display help
sysmon --help

# Display version
sysmon --version
```

**Note**: More options like `--watch` mode and custom intervals will be added in future versions.

---

## Understanding the Output

### Example Output

```
╭─────────────────────────────────────╮
│   System Monitor v0.1.0             │
╰─────────────────────────────────────╯

CPU Usage:  34.5%  [██████░░░░░░░░░░░░░░]
  Core  0:   45.2%  [██████░░░░░░░░░]
  Core  1:   28.3%  [████░░░░░░░░░░░]
  Core  2:   31.1%  [████░░░░░░░░░░░]
  Core  3:   33.8%  [█████░░░░░░░░░░]

Memory:     8.34/16.00 GB (52.1%)
            [██████████░░░░░░░░░░]

Uptime:     2 days, 0 hours, 32 minutes

Press Ctrl+C to exit
```

### Output Components

#### 1. Header
Shows the program name and version.

#### 2. CPU Usage
- **Overall CPU Usage**: Average across all cores
- **Per-Core Usage**: Individual usage for each CPU core
- **Visual Bar**: `█` represents used capacity, `░` represents available capacity

**Interpretation**:
- 0-30%: Low usage (idle)
- 30-70%: Moderate usage (normal)
- 70-100%: High usage (heavy load)

#### 3. Memory Usage
- **Used/Total**: RAM currently in use vs. total available
- **Percentage**: Memory utilization as a percentage
- **Visual Bar**: Similar to CPU bar

**Interpretation**:
- <50%: Healthy
- 50-80%: Moderate usage
- >80%: High usage (may impact performance)

#### 4. System Uptime
Shows how long the system has been running since last boot.

---

## Development

### Project Structure

```
system_monitor/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # CLI argument parsing
│   ├── monitor/             # System monitoring logic
│   │   ├── mod.rs
│   │   ├── cpu.rs           # CPU monitoring
│   │   ├── memory.rs        # Memory monitoring
│   │   └── system.rs        # System facade
│   └── display/             # Output formatting
│       ├── mod.rs
│       └── formatter.rs     # Display utilities
├── Cargo.toml               # Project dependencies
└── README.md                # Documentation
```

### Building for Development

```bash
# Build debug version (faster compilation, slower execution)
cargo build

# Run debug version
cargo run

# Check code without building
cargo check

# Run with Rust linter
cargo clippy
```

### Building for Production

```bash
# Build optimized release version
cargo build --release

# The binary will be significantly faster and smaller
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_create_bar
```

---

## Troubleshooting

### Common Issues

#### Issue: "cargo: command not found"

**Solution**: Ensure Rust is installed and in your PATH.

```bash
# Linux/macOS: Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"

# Windows: Add to PATH environment variable
%USERPROFILE%\.cargo\bin
```

#### Issue: Permission Denied (Linux/macOS)

**Solution**: Make the binary executable.

```bash
chmod +x target/release/sysmon
```

#### Issue: Build Fails

**Solution**: Update Rust to the latest version.

```bash
rustup update
```

#### Issue: Inaccurate CPU/Memory Readings

**Note**: First reading after starting may be less accurate. This is normal behavior due to the way system metrics are gathered.

---

## Advanced Topics

### Understanding the Code

#### Ownership and Borrowing

System Monitor demonstrates Rust's ownership system:

```rust
// The SystemMonitor owns the System instance
pub struct SystemMonitor {
    sys: System,  // Owned data
}

// Functions borrow data with &
pub fn print_cpu_info(monitor: &SystemMonitor) {
    // monitor is borrowed, not owned
}
```

#### Module System

The project uses Rust's module system for organization:

- `mod.rs`: Declares submodules
- Each file is a module
- `pub` keyword makes items public

#### Error Handling

Future versions will use Result types:

```rust
pub fn refresh(&mut self) -> Result<(), Error> {
    // Error handling logic
}
```

---

## Contributing

### How to Contribute

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write tests
5. Submit a pull request

### Coding Standards

- Follow Rust style guidelines (`rustfmt`)
- Write documentation comments (`///`)
- Add tests for new features
- Keep functions small and focused

### Future Features

Want to contribute? Here are some ideas:

- [ ] Watch mode (continuous updates)
- [ ] Disk usage monitoring
- [ ] Network statistics
- [ ] Process list (top N)
- [ ] Export to JSON/CSV
- [ ] Interactive TUI

---

## Resources

### Learning Rust

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### Libraries Used

- [sysinfo](https://docs.rs/sysinfo/) - System information
- [clap](https://docs.rs/clap/) - Command-line parsing

### Related Projects

- [htop](https://htop.dev/) - Interactive process viewer
- [bottom](https://github.com/ClementTsang/bottom) - System monitor in Rust
- [gtop](https://github.com/aksakalli/gtop) - System monitor in Node.js

---

## Support

- **Issues**: [GitHub Issues](https://github.com/SoftDryzz/system_monitor/issues)
- **Discussions**: [GitHub Discussions](https://github.com/SoftDryzz/system_monitor/discussions)
- **Author**: [@SoftDryzz](https://github.com/SoftDryzz)

---

**Happy Monitoring! 📊**
