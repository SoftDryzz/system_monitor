# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned for v0.5.0
- Export to JSON/CSV
- Configuration file support
- Custom color themes
- Historical data tracking
- Process filtering and search

---

## [0.4.0] - 2025-01-22

### Added
- **Network Statistics** - Real-time download and upload speed monitoring
- **Network Traffic Totals** - Track total bytes received and transmitted
- **Color-Coded UI** - Visual health indicators throughout the application
- **Green/Yellow/Red Indicators** - Based on usage percentage thresholds
- **Automatic Unit Conversion** - Network speed (B/s to GB/s) and bytes (B to TB)
- **Speed Calculation** - Delta-based calculation using timestamps
- **Colored Headers** - Cyan section titles for better organization
- **Network Module** - New `network.rs` with `NetworkInfo` struct

### Changed
- Enhanced formatter with color support using `colored` crate
- SystemMonitor now tracks network state for speed calculations
- CPU, memory, disk, and process metrics now display with colors
- Improved visual feedback on system health status
- Version bump to 0.4.0

### Technical
- Added `colored = "2.1"` dependency for terminal colors
- Network tracking with `Instant` timestamps for accurate speed calculation
- `NetworkInfo::format_speed()` and `format_bytes()` utility functions
- Color selection function based on percentage thresholds
- Cross-platform color support (Windows, Linux, macOS)

### Fixed
- Improved readability with color-coded metrics
- Better visual distinction between healthy and critical states

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

| Version | CPU | Memory | Disk | Processes | Network | Watch | Colors |
|---------|-----|--------|------|-----------|---------|-------|--------|
| 0.1.0 | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| 0.2.0 | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ |
| 0.2.1 | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ |
| 0.3.0 | ✅+ | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ |
| 0.4.0 | ✅+ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

**✅+** = Enhanced with compact/detailed modes

---

## Feature Timeline

```
v0.1.0 ──► v0.2.0 ──► v0.2.1 ──► v0.3.0 ──► v0.4.0
  │          │          │          │          │
  │          │          │          │          └─ Network + Colors
  │          │          │          └─ Processes + Intelligent Scaling
  │          │          └─ Disk Usage
  │          └─ Watch Mode
  └─ Basic Monitoring (CPU, Memory, Uptime)
```

---

## Migration Guides

### From 0.3.x to 0.4.0

No breaking changes. All 0.3.x commands work identically.

**New features available:**
- Colors are automatically applied (no flag needed)
- Network statistics displayed by default

**Visual changes:**
- All metrics now color-coded (green/yellow/red)
- Network section added to output
- Headers in cyan for better organization

### From 0.2.x to 0.3.0

No breaking changes. All 0.2.x commands work identically.

**New features available:**
```bash
# Use detailed mode (all cores, more processes)
sysmon --detailed
```

### From 0.1.0 to 0.2.0

No breaking changes.

**New features available:**
```bash
# Continuous monitoring
sysmon --watch

# Custom interval
sysmon --watch --interval 5
```

---

## Known Issues

### v0.4.0
- First network speed reading shows 0 (requires delta calculation)
- Colors may not display in very old terminals (falls back gracefully)
- Some terminals may need configuration for full color support

### v0.3.0
- First process reading may show 0% CPU (warmup needed)
- Process names truncated to 20 characters in display

### v0.2.1
- Disk monitoring may not detect all virtual drives
- Network drives may show incorrect sizes

### All Versions
- Very first CPU reading after start may be inaccurate (sysinfo limitation)

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| sysinfo | 0.32 | System information |
| clap | 4.5 | CLI argument parsing |
| crossterm | 0.28 | Terminal manipulation |
| ctrlc | 3.4 | Signal handling |
| colored | 2.1 | Terminal colors (v0.4.0+) |

---

[Unreleased]: https://github.com/SoftDryzz/system_monitor/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.4.0
[0.3.0]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.3.0
[0.2.1]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.2.1
[0.2.0]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.2.0
[0.1.0]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.1.0