# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Watch mode for continuous monitoring
- Disk usage information
- Network statistics
- Process list (top N by CPU/RAM)
- Export to JSON/CSV
- Interactive TUI mode
- Custom refresh intervals
- Configuration file support

## [0.1.0] - 2025-01-21

### Added
- Initial release
- Real-time CPU monitoring (global and per-core)
- Memory usage tracking (RAM)
- System uptime display
- Beautiful CLI output with visual progress bars
- Cross-platform support (Windows, Linux, macOS)
- Comprehensive documentation (README, User Guide, Contributing)
- Modular code structure
- MIT License
- Basic CLI argument parsing (foundation for future features)
- Unit tests for display formatter

### Technical Details
- Built with Rust 2021 edition
- Uses `sysinfo` crate for system information
- Uses `clap` crate for CLI parsing
- Organized into modules: monitor, display, cli

---

## Version History

### Version 0.1.0 - Initial Release
First public release of System Monitor. Provides basic but functional system monitoring capabilities with a focus on clean code architecture and professional documentation.

**Key Features:**
- CPU monitoring with per-core breakdown
- Memory usage with visual indicators
- System uptime tracking
- Cross-platform compatibility

**Known Limitations:**
- No watch mode (single snapshot only)
- No disk or network monitoring
- No process information
- No export capabilities

These limitations will be addressed in future releases.

---

[Unreleased]: https://github.com/SoftDryzz/system_monitor/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/SoftDryzz/system_monitor/releases/tag/v0.1.0
