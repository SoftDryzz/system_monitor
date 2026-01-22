# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned for v0.4.0
- Network statistics (download/upload speed)
- Export to JSON/CSV
- Color-coded warnings for high CPU/memory usage
- Process filtering and search

---

## [0.3.0] - 2025-01-22

### Added
- **Process Monitoring** - Display top processes by CPU and memory usage
- **Compact Mode** (default) - Shows top 3 busiest CPU cores and top 5 processes
- **Detailed Mode** (`--detailed` flag) - Shows all CPU cores and top 10 processes
- **Intelligent Scaling** - Adapts display for systems with 4 to 128+ CPU cores
- **Top Cores Detection** - Automatically identifies and displays busiest cores
- **Smart Process Display** - Process name truncation and memory format conversion

### Changed
- CPU display now shows core count in header
- Enhanced SystemMonitor with process querying methods
- Improved formatter with compact/detailed mode support
- Updated CLI with `--detailed` argument
- Version bump to 0.3.0

### Technical
- Added `process.rs` module with ProcessInfo struct
- Process sorting algorithms (by CPU and memory)
- Enhanced `system.rs` with process methods
- Updated `formatter.rs` with process display functions
- Modified `main.rs` to support detailed argument

### Fixed
- CPU display becoming unwieldy on high-core-count systems
- Memory formatting for large values
- Process name display for long names

---

## [0.2.1] - 2025-01-21

### Added
- **Disk Usage Monitoring** - Display space usage for all mounted drives
- Visual progress bars for each disk
- Automatic filtering of small virtual drives (< 1GB)
- GB formatting for disk space

### Changed
- Enhanced display layout with disk information
- Updated SystemMonitor to include disk queries
- Version bump to 0.2.1

### Technical
- Added `disk.rs` module with DiskInfo struct
- Disk space calculation and formatting utilities
- Integration with sysinfo Disks API

---

## [0.2.0] - 2025-01-21

### Added
- **Watch Mode** - Continuous monitoring with `--watch` flag
- **Custom Intervals** - Set update frequency with `--interval` option
- **Graceful Shutdown** - Clean exit with Ctrl+C handling
- **Terminal Clearing** - Smooth screen refresh between updates
- Responsive sleep with quick Ctrl+C response

### Changed
- CLI arguments now functional (was placeholder in 0.1.0)
- Main entry point refactored for watch/single modes
- Enhanced header to show watch mode and interval
- Version bump to 0.2.0

### Technical
- Added `crossterm` dependency for terminal manipulation
- Added `ctrlc` dependency for signal handling
- Atomic boolean flag for thread-safe shutdown
- Platform-specific screen clearing implementation

---

## [0.1.0] - 2025-01-20

### Added
- Initial release
- Real-time CPU monitoring (global and per-core)
- Memory usage tracking (RAM)
- System uptime display
- Beautiful CLI output with visual progress bars
- Cross-platform support (Windows, Linux, macOS)
- Comprehensive documentation (README, User Guide)
- Modular code structure
- MIT License
- Basic CLI argument parsing (foundation)
- Unit tests for display formatter

### Technical Details
- Built with Rust 2021 edition
- Uses `sysinfo` crate for system information
- Uses `clap` crate for CLI parsing
- Organized into modules: monitor, display, cli
- Facade pattern in SystemMonitor
- Separation of concerns architecture

---

## Version Comparison

| Version | CPU | Memory | Disk | Processes | Watch | Notes |
|---------|-----|--------|------|-----------|-------|-------|
| 0.1.0 | ✅ | ✅ | ❌ | ❌ | ❌ | Initial release |
| 0.2.0 | ✅ | ✅ | ❌ | ❌ | ✅ | Watch mode |
| 0.2.1 | ✅ | ✅ | ✅ | ❌ | ✅ | Disk monitoring |
| 0.3.0 | ✅+ | ✅ | ✅ | ✅ | ✅ | Smart scaling |

**✅+** = Enhanced with compact/detailed modes

---

## Migration Guides

### From 0.2.x to 0.3.0

No breaking changes. All 0.2.x commands work identically.

**New features available:**
```bash
# Use compact mode (default)
sysmon

# Use detailed mode (all cores, more processes)
sysmon --detailed

# Combine with watch mode
sysmon --watch --detailed
```

### From 0.1.0 to 0.2.0

No breaking changes. 0.1.0 behavior (single snapshot) is maintained as default.

**New features available:**
```bash
# Continuous monitoring
sysmon --watch

# Custom interval
sysmon --watch --interval 5
```

---

## Known Issues

### v0.3.0
- First process reading may show 0% CPU (warmup needed)
- Process names truncated to 20 characters in display
- Memory usage reported by processes may differ from system total

### v0.2.1
- Disk monitoring may not detect all virtual drives
- Network drives may show incorrect sizes

### All Versions
- Very first CPU reading after start may be inaccurate (sysinfo limitation)

---

[Unreleased]: https://github.com/SoftDryzz/system_monitor/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.3.0
[0.2.1]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.2.1
[0.2.0]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.2.0
[0.1.0]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.1.0
