# Notepad++ Rust Edition

A complete port of [Notepad++](https://notepad-plus-plus.org/) from C++ to Rust for Windows.

## Overview

This project aims to maintain feature parity with the original Notepad++ while leveraging Rust's memory safety, modern tooling, and performance benefits.

### Original Codebase
- **Language**: C++ (149,301 lines)
- **Architecture**: Win32 API with Scintilla editor
- **Features**: 200+ commands, 90+ language support, plugin system
- **Repository**: [notepad-plus-plus/notepad-plus-plus](https://github.com/notepad-plus-plus/notepad-plus-plus)

## Project Structure

```
rust-notepad-plus/
├── src/
│   └── main.rs              # Application entry point
├── crates/
│   ├── core/                # Core application logic
│   ├── ui/                  # User interface (Win32)
│   ├── editor/              # Text editor component (Ropey-based)
│   ├── lexer/               # Syntax highlighting
│   ├── plugins/             # Plugin system (FFI)
│   ├── config/              # Configuration management
│   ├── io/                  # File I/O and encoding
│   └── search/              # Search and replace
└── Cargo.toml               # Workspace configuration
```

## Architecture

### Tech Stack

- **UI Framework**: Windows API via `windows-rs` crate
- **Text Buffer**: Ropey (rope data structure)
- **Syntax Highlighting**: tree-sitter + syntect
- **Encoding**: encoding_rs + chardetng
- **Search**: regex + fancy-regex (PCRE-compatible)
- **Config**: quick-xml
- **Plugins**: libloading (FFI)

### Key Components

1. **notepad-core**: Application state, buffer management, command routing
2. **notepad-ui**: Main window, menus, toolbars, dialogs
3. **notepad-editor**: Text editing widget with syntax highlighting
4. **notepad-lexer**: Language definitions and syntax parsers
5. **notepad-plugins**: Plugin loader and manager
6. **notepad-config**: XML configuration parsing
7. **notepad-io**: File operations, encoding detection, monitoring
8. **notepad-search**: Search/replace engine with regex support

## Features

### Implemented ✅
- [x] Project structure and workspace
- [x] Core application architecture
- [x] Buffer management
- [x] Command system
- [x] Basic Windows UI
- [x] File I/O with encoding detection
- [x] Text buffer (Ropey)
- [x] Plugin system foundation

### In Progress 🚧
- [ ] Main window with editor view
- [ ] Menu and toolbar
- [ ] Find/Replace dialog
- [ ] Syntax highlighting
- [ ] Settings dialog

### Planned 📋
- [ ] Full Win32 UI implementation
- [ ] Scintilla FFI bindings (alternative approach)
- [ ] Dark mode support
- [ ] Plugin compatibility layer
- [ ] Macro recording/playback
- [ ] Auto-completion
- [ ] Function list
- [ ] Document map
- [ ] Multi-view (split)
- [ ] Session management
- [ ] DPI awareness
- [ ] Localization

## Building

### Prerequisites

- Rust toolchain (1.70+)
- Windows SDK
- Visual Studio Build Tools (MSVC)

### Build Commands

```powershell
# Debug build
cargo build

# Release build
cargo build --release

# Run
cargo run

# Test
cargo test

# Check (faster than build)
cargo check
```

### Platform Support

- **Windows 10/11**: Primary target
- **Architecture**: x64, x86, ARM64

## Development

### Code Organization

- **Modular**: Each crate handles a specific concern
- **Type-safe**: Leverage Rust's type system
- **Memory-safe**: No unsafe code except FFI boundaries
- **Tested**: Unit tests for core functionality

### Contribution Guidelines

1. Follow Rust idioms and best practices
2. Add tests for new features
3. Document public APIs
4. Run `cargo fmt` and `cargo clippy`

## Comparison with Original

### Advantages of Rust Port

✅ **Memory Safety**: No null pointer dereferences, buffer overflows, or use-after-free
✅ **Concurrency**: Safe multi-threading with Rust's ownership system
✅ **Modern Tooling**: Cargo, cargo-edit, rust-analyzer
✅ **Better Error Handling**: Result types instead of error codes
✅ **Package Management**: Easy dependency management

### Challenges

⚠️ **Win32 FFI**: Extensive use of unsafe code for Windows APIs
⚠️ **Plugin Compatibility**: Maintaining binary compatibility with C++ plugins
⚠️ **Scintilla Integration**: Options are FFI or full rewrite
⚠️ **Development Time**: Large codebase to port

## License

This project is licensed under the GPL-3.0 License, same as the original Notepad++.

## Acknowledgments

- Original Notepad++ by Don Ho and contributors
- Scintilla editing component by Neil Hodgson
- Rust community and crate authors

## Roadmap

### Phase 1: Foundation ✅
- [x] Project structure
- [x] Core types and traits
- [x] Basic Windows application

### Phase 2: Core Features 🚧
- [ ] Text editing with Ropey
- [ ] File operations
- [ ] Search and replace
- [ ] Basic UI

### Phase 3: Advanced Features
- [ ] Syntax highlighting
- [ ] Plugin system
- [ ] Macro support
- [ ] Advanced UI

### Phase 4: Polish
- [ ] Dark mode
- [ ] DPI awareness
- [ ] Performance optimization
- [ ] Documentation

### Phase 5: Release
- [ ] Installer
- [ ] Continuous integration
- [ ] Beta testing
- [ ] v1.0 release

## Resources

- [Notepad++ Source](https://github.com/notepad-plus-plus/notepad-plus-plus)
- [Windows API (windows-rs)](https://github.com/microsoft/windows-rs)
- [Ropey Documentation](https://docs.rs/ropey/)
- [Scintilla Documentation](https://www.scintilla.org/ScintillaDoc.html)

## Contact

For questions or contributions, please open an issue on the repository.

---

**Status**: 🚧 Work in Progress
**Version**: 0.1.0-alpha
**Target**: Feature parity with Notepad++ 8.6+
