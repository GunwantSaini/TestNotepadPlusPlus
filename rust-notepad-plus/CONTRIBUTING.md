# Contributing to Notepad++ Rust Edition

Thank you for your interest in contributing to the Notepad++ Rust Edition! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Code Review Checklist](#code-review-checklist)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Enhancements](#suggesting-enhancements)

---

## Getting Started

### Prerequisites

- **Rust**: 1.70.0 or later ([Install Rust](https://rustup.rs/))
- **Git**: For version control
- **Windows**: Visual Studio 2019+ or Windows 10 SDK (for Windows build)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/TestNotepadPlusPlus.git
   cd TestNotepadPlusPlus/rust-notepad-plus
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/GunwantSaini/TestNotepadPlusPlus.git
   ```

---

## Development Setup

### Build the Project

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run the application
cargo run

# Run with logging
RUST_LOG=debug cargo run
```

### Run Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p notepad-core

# With output
cargo test -- --nocapture

# Single test
cargo test test_name
```

---

## Code Style

### Formatting

We use `rustfmt` for consistent code formatting:

```bash
# Format all code
cargo fmt

# Check formatting (CI runs this)
cargo fmt -- --check
```

**Configuration**: See `rustfmt.toml` in the project root.

### Linting

We use `clippy` for linting:

```bash
# Run clippy
cargo clippy --all-targets --all-features

# Fix auto-fixable warnings
cargo clippy --fix

# Treat warnings as errors (CI does this)
cargo clippy -- -D warnings
```

**Configuration**: See `.clippy.toml` and workspace lints in `Cargo.toml`.

### Code Guidelines

1. **No `unsafe` without justification**
   - Portable crates (`core`, `editor`, `search`, etc.) must be 100% safe
   - Platform crates (`ui`) may use unsafe for FFI, but wrap in safe APIs
   - Document all unsafe usage with `// SAFETY:` comments

2. **Error handling**
   - Use `Result<T>` for fallible operations
   - Use `anyhow::Result` for application-level errors
   - Use custom error types for library crates
   - Never use `.unwrap()` or `.expect()` in production code paths

3. **Documentation**
   - All public items must have doc comments (`///`)
   - Include examples in doc comments when applicable
   - Update module-level docs (`//!`) when adding features

4. **Naming conventions**
   - Types: `PascalCase`
   - Functions/variables: `snake_case`
   - Constants: `SCREAMING_SNAKE_CASE`
   - Private items: prefix with `_` if intentionally unused

5. **Module organization**
   - One primary type per file
   - Group related functionality
   - Keep files under 500 lines when possible

---

## Testing

### Test Requirements

All new features must include tests:

1. **Unit tests**: Test individual functions/methods
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_feature() {
           // Test implementation
       }
   }
   ```

2. **Integration tests**: Test crate interactions
   - Place in `tests/` directory
   - Test public API only

3. **Documentation tests**: Ensure examples in docs work
   ```rust
   /// Example:
   /// ```
   /// use notepad_core::example;
   /// assert_eq!(example(), 42);
   /// ```
   ```

### Test Coverage Goals

- **Portable crates**: >80% coverage
- **Platform crates**: >60% coverage
- **Overall project**: >75% coverage

---

## Pull Request Process

### Before Submitting

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**:
   - Write code following style guidelines
   - Add tests for new functionality
   - Update documentation

3. **Ensure quality**:
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --workspace
   cargo build --release
   ```

4. **Commit with clear messages**:
   ```bash
   git add -A
   git commit -m "Add feature: brief description

   Detailed explanation of what changed and why.

   - Bullet point 1
   - Bullet point 2"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

### Submitting the Pull Request

1. Go to GitHub and create a Pull Request
2. Fill out the PR template completely
3. Link related issues with "Fixes #123" or "Closes #123"
4. Request review from maintainers

### After Submission

- **Respond to feedback**: Address reviewer comments promptly
- **Keep PR updated**: Rebase on main if needed
- **CI must pass**: All checks must be green before merge

---

## Code Review Checklist

Before submitting, verify:

- [ ] Code follows Rust style guidelines
- [ ] All tests pass (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt -- --check`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if user-facing change)
- [ ] No `unsafe` without safety comments
- [ ] Error handling is proper (no unwrap in production paths)
- [ ] New public APIs have doc comments with examples
- [ ] Performance is acceptable (benchmark if applicable)

---

## Reporting Bugs

### Before Reporting

1. Check if the bug is already reported in [Issues](https://github.com/GunwantSaini/TestNotepadPlusPlus/issues)
2. Verify it's not fixed in latest `main` branch
3. Test with release build (`cargo build --release`)

### Bug Report Template

```markdown
**Description**
Clear description of the bug

**Steps to Reproduce**
1. Step 1
2. Step 2
3. Step 3

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: Windows 10/11
- Rust version: 1.75.0
- Build: Debug/Release
- Commit: abc123

**Logs**
```
Paste relevant logs here (run with RUST_LOG=debug)
```

**Additional Context**
Any other information
```

---

## Suggesting Enhancements

### Feature Requests

Open an issue with:
- Clear description of the feature
- Use cases and motivation
- Proposed implementation (if you have ideas)
- Willingness to implement it yourself

### Discussion

For major features:
1. Open a discussion issue first
2. Get feedback from maintainers
3. Create design doc if complex
4. Implement after approval

---

## Project Structure

```
rust-notepad-plus/
├── crates/
│   ├── core/           # Business logic (portable)
│   ├── editor/         # Text buffer (portable)
│   ├── search/         # Search engine (portable)
│   ├── io/             # File I/O (portable)
│   ├── ui/             # Windows UI (platform-specific)
│   ├── lexer/          # Syntax highlighting
│   ├── plugins/        # Plugin system
│   └── config/         # Configuration
├── src/
│   └── main.rs         # Entry point
├── docs/               # Additional documentation
└── tests/              # Integration tests
```

### Crate Guidelines

- **Portable crates**: No platform-specific dependencies
- **Platform crates**: FFI allowed, wrap in safe APIs
- **All crates**: Follow module structure, include tests

---

## Development Workflow

### Typical Workflow

1. **Update main**:
   ```bash
   git checkout main
   git pull upstream main
   ```

2. **Create feature branch**:
   ```bash
   git checkout -b feature/my-feature
   ```

3. **Develop**:
   - Write code
   - Write tests
   - Run `cargo fmt` and `cargo clippy`
   - Commit frequently with clear messages

4. **Push and create PR**:
   ```bash
   git push origin feature/my-feature
   ```
   Then create PR on GitHub

5. **Address review feedback**:
   ```bash
   git add -A
   git commit -m "Address review feedback"
   git push
   ```

6. **After merge**:
   ```bash
   git checkout main
   git pull upstream main
   git branch -d feature/my-feature
   ```

---

## Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/GunwantSaini/TestNotepadPlusPlus/discussions)
- **Bugs**: [Report an issue](https://github.com/GunwantSaini/TestNotepadPlusPlus/issues/new)
- **Documentation**: See [README.md](README.md) and docs in `docs/`

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). By participating, you are expected to uphold this code.

---

## License

By contributing, you agree that your contributions will be licensed under the GPL-3.0 license.

---

Thank you for contributing to Notepad++ Rust Edition! 🎉
