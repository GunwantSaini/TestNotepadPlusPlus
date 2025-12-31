# Notepad++ C++ to Rust Conversion Plan

## Executive Summary

This document outlines the complete conversion strategy for porting Notepad++ (149,301 lines of C++ code) to Rust for Windows. The conversion will maintain feature parity while leveraging Rust's memory safety and modern tooling.

## Project Scope

### Original Codebase Statistics
- **Total C++ LOC**: 149,301 lines (PowerEditor/src)
- **C++ Files**: 127 files
- **Header Files**: 166 files
- **Commands**: 200+ menu commands
- **Supported Languages**: 90+ programming languages
- **UI Dialogs**: 67+ dialog classes
- **Dependencies**: Scintilla, Lexilla, Boost.Regex, TinyXML2, uchardet

### Key Components
1. **Scintilla Text Editor** - Core editing engine
2. **Lexilla** - Syntax highlighting lexers
3. **Win32 UI** - Native Windows interface
4. **Plugin System** - DLL-based plugin architecture
5. **Configuration System** - XML-based settings
6. **File Management** - Multi-encoding support, monitoring
7. **Search/Replace** - Regex-based search engine
8. **Dark Mode** - Full theme support

## Architecture Design

### Rust Project Structure

```
notepad-plus-rust/
├── Cargo.toml                      # Root workspace
├── src/
│   └── main.rs                     # Entry point
├── crates/
│   ├── core/                       # Core application logic
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app.rs              # Main Notepad_plus class
│   │       ├── buffer.rs           # Document buffer management
│   │       ├── commands.rs         # Command handlers
│   │       └── session.rs          # Session management
│   │
│   ├── ui/                         # UI layer
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── main_window.rs      # Main window
│   │       ├── dialogs/            # Dialog implementations
│   │       ├── controls/           # Custom controls
│   │       ├── toolbar.rs
│   │       ├── statusbar.rs
│   │       └── docking.rs          # Docking manager
│   │
│   ├── editor/                     # Text editor component
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── scintilla_ffi.rs    # Scintilla FFI bindings
│   │       ├── view.rs             # Editor view
│   │       └── styling.rs          # Syntax highlighting
│   │
│   ├── lexer/                      # Syntax highlighting
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── registry.rs         # Language registry
│   │       └── languages/          # Language definitions
│   │
│   ├── plugins/                    # Plugin system
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── manager.rs          # Plugin manager
│   │       ├── interface.rs        # Plugin API
│   │       └── loader.rs           # DLL loading
│   │
│   ├── config/                     # Configuration management
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── parameters.rs       # Settings
│   │       ├── xml_parser.rs       # XML handling
│   │       └── themes.rs           # Theme management
│   │
│   ├── io/                         # File operations
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── file_manager.rs     # File I/O
│   │       ├── encoding.rs         # Encoding detection
│   │       └── monitor.rs          # Directory watching
│   │
│   └── search/                     # Search and replace
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── engine.rs           # Search engine
│           ├── regex.rs            # Regex support
│           └── replace.rs          # Replace operations
│
├── assets/                         # Resources
│   ├── icons/
│   ├── themes/
│   └── languages/
│
└── build.rs                        # Build script
```

### Rust Crate Dependencies

```toml
[dependencies]
# Windows API
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Controls",
    "Win32_Graphics_Gdi",
    "Win32_Graphics_Direct2D",
    "Win32_Graphics_DirectWrite",
    "Win32_System_Com",
    "Win32_System_Threading",
    "Win32_System_Registry",
    "Win32_Storage_FileSystem",
    "Win32_UI_Shell",
    "Win32_Security",
] }

# UI Framework (choose one approach)
native-windows-gui = "1.0"          # Option 1: NWG for Win32
# OR use raw windows-rs for maximum control

# Text editing
ropey = "1.6"                        # Rope data structure for text
tree-sitter = "0.20"                 # Modern syntax parsing
syntect = "5.1"                      # Syntax highlighting

# File operations
encoding_rs = "0.8"                  # Encoding support
chardetng = "0.1"                    # Character encoding detection
notify = "6.1"                       # File system monitoring

# Regex
regex = "1.10"                       # Rust regex engine
fancy-regex = "0.13"                 # PCRE-compatible regex

# XML parsing
quick-xml = "0.31"                   # Fast XML parser
serde = { version = "1.0", features = ["derive"] }
serde_xml_rs = "0.6"

# Plugin system
libloading = "0.8"                   # Dynamic library loading
dlopen2 = "0.7"                      # Alternative DLL loader

# Utilities
anyhow = "1.0"                       # Error handling
thiserror = "1.0"                    # Custom errors
log = "0.4"                          # Logging
env_logger = "0.11"                  # Logger implementation
crossbeam = "0.8"                    # Concurrency primitives
parking_lot = "0.12"                 # Better synchronization

# Registry access
winreg = "0.52"                      # Windows registry

# Clipboard
clipboard-win = "5.0"                # Clipboard operations

# Hashing
md-5 = "0.10"
sha1 = "0.10"
sha2 = "0.10"

[build-dependencies]
winres = "0.1"                       # Windows resource compiler
cc = "1.0"                           # C/C++ compiler integration (for Scintilla FFI)
```

## Implementation Strategy

### Phase 1: Foundation (Weeks 1-2)
1. Set up Rust workspace with cargo
2. Create basic project structure
3. Set up build system with `build.rs`
4. Configure Windows resources and manifest

### Phase 2: Scintilla Integration (Weeks 3-4)
**Approach**: Use existing Scintilla via FFI rather than full rewrite
1. Create Rust FFI bindings to Scintilla DLL
2. Wrap Scintilla in safe Rust interface
3. Implement `ScintillaEditView` equivalent
4. Test basic text editing functionality

### Phase 3: Core Application (Weeks 5-8)
1. Port `Notepad_plus` core class
2. Implement `Buffer` and `FileManager`
3. Create command routing system
4. Implement document/view architecture
5. Add multi-view support (split view)

### Phase 4: UI Components (Weeks 9-12)
1. Port main window (`Notepad_plus_Window`)
2. Implement menu system
3. Create toolbar and status bar
4. Implement tab bar for documents
5. Create basic dialogs (About, Settings)

### Phase 5: File Operations (Weeks 13-14)
1. Implement file I/O with encoding detection
2. Add file monitoring for external changes
3. Implement backup system
4. Add session management

### Phase 6: Search & Replace (Weeks 15-16)
1. Port Find/Replace dialog
2. Implement regex search engine
3. Add Find in Files functionality
4. Implement incremental search

### Phase 7: Configuration System (Weeks 17-18)
1. Create XML parser for config files
2. Implement `Parameters` equivalent
3. Add theme support
4. Create preferences dialog

### Phase 8: Advanced Features (Weeks 19-22)
1. Implement syntax highlighting with Lexilla/tree-sitter
2. Create macro recording/playback
3. Add auto-completion
4. Implement function list
5. Create document map

### Phase 9: Plugin System (Weeks 23-24)
1. Design Rust plugin API
2. Implement plugin loader
3. Create FFI bridge for C++ plugins
4. Test with sample plugins

### Phase 10: Dark Mode & Theming (Weeks 25-26)
1. Implement dark mode support
2. Create theme system
3. Add DPI awareness
4. Implement custom window styling

### Phase 11: Docking & Advanced UI (Weeks 27-28)
1. Implement docking manager
2. Create dockable panels (File Browser, Function List)
3. Add clipboard history
4. Implement column editor

### Phase 12: Polish & Testing (Weeks 29-32)
1. Comprehensive testing
2. Performance optimization
3. Bug fixes
4. Documentation
5. Build installer

## Key Technical Challenges

### 1. Scintilla Integration
**Challenge**: Scintilla is a mature C++ component
**Solution**:
- Use Scintilla DLL via FFI bindings
- Create safe Rust wrapper
- Alternative: Explore pure Rust editor components (xi-editor core, ropey + syntect)

### 2. Win32 API
**Challenge**: Heavy reliance on Win32 API
**Solution**:
- Use `windows` crate for type-safe bindings
- Consider `native-windows-gui` for higher-level abstractions
- Create safe wrappers around unsafe FFI calls

### 3. Message-Based Architecture
**Challenge**: C++ uses Windows message loop extensively
**Solution**:
- Maintain similar architecture in Rust
- Use `windows::Win32::UI::WindowsAndMessaging` APIs
- Consider event-driven design with channels

### 4. Plugin Compatibility
**Challenge**: Existing plugins are C++ DLLs
**Solution**:
- Maintain binary compatibility with plugin interface
- Use `libloading` for dynamic library loading
- Create FFI-safe structures matching original API
- Provide new Rust-native plugin API

### 5. String Handling
**Challenge**: Windows uses UTF-16, Scintilla uses UTF-8
**Solution**:
- Use `encoding_rs` for conversions
- Leverage Rust's `OsString`/`OsStr` for Windows paths
- Create conversion utilities

### 6. Resource Management
**Challenge**: Manual memory management in C++
**Solution**:
- Leverage Rust's ownership system
- Use RAII patterns with Drop trait
- Smart pointers (`Rc`, `Arc`) where needed

### 7. COM Interfaces
**Challenge**: File dialogs and shell integration use COM
**Solution**:
- Use `windows` crate COM support
- Create safe wrappers
- Handle reference counting properly

## Alternative Approaches

### Approach A: Pure Rust with Modern UI
**Pros**: Memory safety, modern architecture, cross-platform potential
**Cons**: Different look/feel, significant rewrite, plugin incompatibility
**Stack**: `windows-rs` + `egui`/`iced` + `tree-sitter` + `ropey`

### Approach B: Hybrid with Scintilla FFI (RECOMMENDED)
**Pros**: Maintain text editing quality, faster development, plugin compatibility
**Cons**: Still depends on C++ code, some unsafe code
**Stack**: `windows-rs` + `native-windows-gui` + Scintilla FFI + `quick-xml`

### Approach C: Minimal Port with C++ Interop
**Pros**: Fastest path to working application
**Cons**: Least "Rusty", limited benefits
**Stack**: Extensive FFI to existing C++ code

## Recommended Approach: Hybrid Strategy

1. **Core Logic**: Pure Rust (buffer management, commands, configuration)
2. **Text Editor**: Scintilla via FFI (mature, proven)
3. **UI**: `windows-rs` with `native-windows-gui` for common controls
4. **Syntax**: Lexilla via FFI or port to tree-sitter
5. **Plugins**: FFI bridge + new Rust API

## Success Metrics

1. ✅ Builds successfully for Windows (x64, ARM64)
2. ✅ All core features working (open, edit, save, search)
3. ✅ UI matches original functionality
4. ✅ Plugin API maintains compatibility
5. ✅ Performance comparable or better
6. ✅ Memory safety (no segfaults/undefined behavior)
7. ✅ Passes comprehensive test suite

## Risk Mitigation

1. **Scope Creep**: Focus on core features first, advanced features later
2. **Performance**: Profile early, optimize hot paths
3. **Compatibility**: Maintain plugin API contract strictly
4. **UI Consistency**: Reference original for behavior
5. **Testing**: Unit tests + integration tests + manual QA

## Next Steps

1. Create Rust workspace structure
2. Set up basic Windows application skeleton
3. Implement Scintilla FFI bindings
4. Port core `Buffer` class
5. Create main window with basic menu
6. Iterate on features incrementally

---

**Estimated Timeline**: 6-8 months for full feature parity
**Team Size**: 2-3 developers
**Priority**: Core editing → UI → Advanced features → Polish
