# Quick Start Guide

## Installation

### 1. Install Rust (if not already installed)

Visit https://rustup.rs/ or run:

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download and run rustup-init.exe from https://rustup.rs/
```

### 2. Clone and Build

```bash
# Clone the repository
git clone https://github.com/SoftDryzz/system_monitor.git
cd system_monitor

# Build release version
cargo build --release

# The binary will be in: target/release/sysmon (or sysmon.exe on Windows)
```

### 3. Test the Binary

```bash
# Linux/macOS
./target/release/sysmon

# Windows
.\target\release\sysmon.exe
```

### 4. Install Globally (Optional)

```bash
# This installs to ~/.cargo/bin (must be in PATH)
cargo install --path .

# Now you can run from anywhere:
sysmon
```

## Verify Installation

```bash
# Check version
sysmon --version

# Show help
sysmon --help

# Run the monitor
sysmon
```

## First Build Tips

- First compilation will take 2-5 minutes (downloading dependencies)
- Subsequent builds are much faster (incremental compilation)
- Use `cargo build` for faster dev builds
- Use `cargo build --release` for optimized production builds

## Troubleshooting

### "cargo: command not found"

Make sure Rust is installed and in your PATH:

```bash
# Add to ~/.bashrc or ~/.zshrc (Linux/macOS)
export PATH="$HOME/.cargo/bin:$PATH"

# Reload shell
source ~/.bashrc
```

### Permission Denied (Linux/macOS)

```bash
chmod +x target/release/sysmon
```

### Build Errors

Update Rust to latest:

```bash
rustup update
```

## Next Steps

- Read the [User Guide](docs/USER_GUIDE.md)
- Check [Contributing Guide](CONTRIBUTING.md)
- Report issues on GitHub
- Star the repository ⭐

---

**Happy Monitoring! 📊**
