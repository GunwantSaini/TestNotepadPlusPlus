# Building Notepad++ Rust Edition

## Overview

This is a Windows-native application that uses the Win32 API. It can be built natively on Windows or cross-compiled from Linux.

## Building on Windows

### Prerequisites
- Rust toolchain (1.70 or later)
- MSVC build tools or MinGW-w64

### Build Commands
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run the application
cargo run
```

## Cross-Compiling from Linux

The codebase compiles successfully on Linux, but linking requires Windows libraries. To actually build a runnable executable on Linux, you need to set up cross-compilation.

### Prerequisites
- Rust toolchain with Windows target
- MinGW-w64 cross-compiler

### Setup
```bash
# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW-w64 (Ubuntu/Debian)
sudo apt-get install mingw-w64

# Configure cargo to use the cross-compiler
```

Create or edit `~/.cargo/config.toml`:
```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"
```

### Build Commands
```bash
# Build for Windows from Linux
cargo build --target x86_64-pc-windows-gnu

# Build release
cargo build --target x86_64-pc-windows-gnu --release
```

## Development on Linux

For development and testing code changes on Linux without full linking:

```bash
# Check that code compiles without linking
cargo check

# Run tests
cargo test

# Run examples (non-GUI ones)
cargo run --example text_editing
cargo run --example search_demo
cargo run --example encoding_demo
```

## Current Status

### ✅ Completed
- **Win32 Window Creation**: Full window class registration and creation
- **Message Loop**: Complete Windows message pump implementation
- **Menu System**: All menus (File, Edit, Search, View, Help) with 30+ menu items
- **Menu Command Routing**: Command ID mapping and basic handling
- **Window Messages**: WM_CREATE, WM_DESTROY, WM_PAINT, WM_SIZE, WM_COMMAND handlers
- **Core Functionality**: Text buffer, file I/O, search/replace, encoding support
- **All workspace crates compile successfully** (`cargo check` passes)

### 🚧 In Progress
- Toolbar and status bar implementation
- Scintilla editor integration
- Find/Replace dialog

### 📋 Pending
- Complete editor integration
- Keyboard shortcuts/accelerators
- Full command implementations
- Plugin system activation

## Architecture

### Crate Structure
```
notepad-plus-rust/
├── crates/
│   ├── core/         - Core application logic
│   ├── ui/           - Win32 GUI (main_window, menu)
│   ├── editor/       - Text editor with Ropey buffer
│   ├── search/       - Search/replace engine
│   ├── lexer/        - Syntax highlighting
│   ├── io/           - File operations
│   ├── config/       - Configuration management
│   └── plugins/      - Plugin system
└── src/main.rs       - Entry point with message loop
```

### Key Files
- `crates/ui/src/main_window.rs` - Win32 window implementation
- `crates/ui/src/menu.rs` - Menu bar creation and command handling
- `src/main.rs` - Windows message loop
- `crates/editor/src/buffer.rs` - Ropey-based text buffer
- `crates/search/src/engine.rs` - Search/replace with regex

## Running Examples

Several working examples demonstrate core functionality:

```bash
# Text editing with Ropey buffer
cargo run --example text_editing

# Search and replace with regex
cargo run --example search_demo

# Multi-encoding file I/O
cargo run --example encoding_demo
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p notepad-core
cargo test -p notepad-search
```

Current test status: **14/14 tests passing**

## Windows Features Used

The application requires these Windows API features (defined in workspace Cargo.toml):
- `Win32_Foundation` - Basic Windows types (HWND, LRESULT, etc.)
- `Win32_UI_WindowsAndMessaging` - Window creation and messaging
- `Win32_UI_Controls` - UI controls
- `Win32_Graphics_Gdi` - Graphics Device Interface
- `Win32_System_LibraryLoader` - Module loading
- `Win32_System_Threading` - Threading support
- `Win32_System_Memory` - Memory management
- `Win32_UI_Shell` - Shell integration
- `Win32_UI_Input_KeyboardAndMouse` - Input handling

## Notes

- The code is designed for Windows and uses Win32 API directly
- On Linux, `cargo check` succeeds but `cargo build` fails at linking (expected)
- To run the actual GUI, build and run on Windows or use cross-compilation
- All core functionality (text editing, search, file I/O) works cross-platform and is tested
