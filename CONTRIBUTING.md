# Contributing to System Monitor

First off, thank you for considering contributing to System Monitor! It's people like you that make this tool better for everyone.

## Code of Conduct

This project adheres to a simple code of conduct: be respectful, be constructive, and help create a welcoming environment for everyone.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title** describing the problem
- **Steps to reproduce** the behavior
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, etc.)
- **Screenshots** if applicable

**Example:**

```markdown
**Bug**: CPU usage shows 0% on Windows 11

**Steps to reproduce:**
1. Run `sysmon` on Windows 11
2. Observe CPU usage display

**Expected**: Should show actual CPU usage
**Actual**: Shows 0% for all cores

**Environment:**
- OS: Windows 11 22H2
- Rust: 1.75.0
- System Monitor: 0.1.0
```

### Suggesting Enhancements

Enhancement suggestions are welcome! Include:

- **Clear description** of the feature
- **Use case**: Why is it useful?
- **Proposed implementation** (if you have ideas)

### Pull Requests

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
4. **Test** your changes:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
5. **Commit** with descriptive messages (see below)
6. **Push** to your fork
7. **Open a Pull Request**

## Development Setup

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git
- A text editor or IDE (VSCode with rust-analyzer recommended)

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/system_monitor.git
cd system_monitor

# Build
cargo build

# Run
cargo run

# Test
cargo test
```

## Coding Standards

### Rust Style

We follow standard Rust style guidelines:

```bash
# Format your code before committing
cargo fmt

# Check for common mistakes
cargo clippy
```

### Code Organization

- Keep functions small and focused
- Use descriptive names
- Add documentation comments (`///`)
- Group related functionality in modules

### Example

```rust
/// Calculate the percentage of used memory
///
/// # Arguments
/// * `used` - Used memory in bytes
/// * `total` - Total memory in bytes
///
/// # Returns
/// Percentage as f64
pub fn memory_percentage(used: u64, total: u64) -> f64 {
    (used as f64 / total as f64) * 100.0
}
```

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

### Examples

```bash
feat(cpu): add per-core temperature monitoring

Adds temperature monitoring for each CPU core using
the sysinfo crate's temperature feature.

Closes #42
```

```bash
fix(memory): correct GB to MB conversion

The conversion was using 1000 instead of 1024.
Now properly converts using binary units (GiB).
```

## Testing

### Writing Tests

Add tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_percentage() {
        let result = memory_percentage(50, 100);
        assert_eq!(result, 50.0);
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_memory_percentage

# Run with output
cargo test -- --nocapture
```

## Documentation

### Code Documentation

Use Rust doc comments:

```rust
/// Brief description
///
/// Longer description if needed.
///
/// # Arguments
/// * `arg1` - Description
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// let result = function(arg);
/// ```
pub fn function(arg: Type) -> ReturnType {
    // implementation
}
```

### Markdown Documentation

- Keep lines under 80-100 characters
- Use code blocks with language specification
- Include examples where helpful

## Project Structure

Understanding the codebase:

```
src/
├── main.rs              # Entry point
├── cli.rs               # CLI parsing
├── monitor/             # System monitoring
│   ├── mod.rs           # Module declaration
│   ├── cpu.rs           # CPU metrics
│   ├── memory.rs        # Memory metrics
│   └── system.rs        # System facade
└── display/             # Output formatting
    ├── mod.rs
    └── formatter.rs     # Display functions
```

### Module Responsibilities

- **cli**: Parse command-line arguments
- **monitor**: Gather system information
- **display**: Format and display information

## Feature Development Workflow

### Planning

1. Open an issue describing the feature
2. Discuss implementation approach
3. Get feedback before coding

### Implementation

1. Create a feature branch
2. Write tests first (TDD)
3. Implement the feature
4. Ensure tests pass
5. Update documentation

### Review

1. Self-review your changes
2. Ensure CI passes
3. Request review from maintainers

## Questions?

- Open an issue for questions
- Check existing issues and PRs
- Read the [User Guide](docs/USER_GUIDE.md)

## Recognition

Contributors will be acknowledged in:
- Release notes
- README (for significant contributions)
- GitHub contributors page

---

Thank you for contributing to System Monitor! 🚀
