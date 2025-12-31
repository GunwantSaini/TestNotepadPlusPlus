# Architecture Documentation - Notepad++ Rust Edition

This document describes the software architecture of the cross-platform Notepad++ Rust Edition.

## Table of Contents
1. [Overview](#overview)
2. [Design Principles](#design-principles)
3. [Crate Structure](#crate-structure)
4. [Platform Abstraction](#platform-abstraction)
5. [Data Flow](#data-flow)
6. [Component Details](#component-details)
7. [Adding New Platforms](#adding-new-platforms)
8. [Performance Considerations](#performance-considerations)

---

## Overview

### Architecture Goals
- **Cross-Platform**: Support Windows, Linux, and future platforms (macOS, Web)
- **Zero Duplication**: Share all business logic across platforms
- **Type Safety**: Compile-time platform detection
- **Performance**: Native UI on each platform
- **Maintainability**: Clear separation of concerns

### High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                   src/main.rs                       │
│            (Platform Detection Layer)               │
│                                                     │
│  #[cfg(windows)] → ui-windows    (Win32 API)      │
│  #[cfg(linux)]   → ui-gtk        (GTK4)           │
│  #[cfg(macos)]   → ui-cocoa      (Future)         │
└─────────────────────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────┐
│              crates/ui-core                         │
│        (Platform-Independent Abstractions)          │
│                                                     │
│  • Traits: TextEditor, DialogProvider, etc.       │
│  • Types: FileFilter, MessageType, etc.           │
│  • Portable: Encoding, RecentFiles, AppState      │
└─────────────────────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────┐
│        Business Logic Crates (Portable)             │
│                                                     │
│  • core:    Application logic                      │
│  • editor:  Rope-based text buffer                 │
│  • search:  Find/replace engine                    │
│  • lexer:   Syntax highlighting                    │
│  • io:      File I/O                               │
│  • config:  Configuration management               │
│  • plugins: Plugin system                          │
└─────────────────────────────────────────────────────┘
```

---

## Design Principles

### 1. Platform Abstraction via Traits

Instead of `#ifdef` scattered throughout code, we use Rust's trait system:

```rust
// Platform-independent trait
pub trait TextEditor {
    fn set_text(&mut self, text: &str);
    fn get_text(&self) -> String;
    // ...
}

// Windows implementation
impl TextEditor for Win32TextEditor { /* ... */ }

// Linux implementation
impl TextEditor for GtkTextEditor { /* ... */ }
```

### 2. Conditional Compilation at Entry Point

Platform selection happens once in `main.rs`:

```rust
#[cfg(target_os = "windows")]
use notepad_ui_windows as ui_backend;

#[cfg(target_os = "linux")]
use notepad_ui_gtk as ui_backend;
```

All downstream code is platform-agnostic.

### 3. Shared State Management

Global state uses the same API on all platforms:

```rust
// ui-windows/src/global_state.rs
pub fn with_state<F, R>(f: F) -> R { /* ... */ }

// ui-gtk/src/app_state_manager.rs
pub fn with_state<F, R>(f: F) -> R { /* ... */ }
```

Implementation details differ, but the interface is identical.

### 4. Zero-Cost Abstraction

Traits compile to static dispatch (no vtables):
- No runtime overhead
- Monomorphization eliminates abstraction cost
- Same performance as hand-written platform code

---

## Crate Structure

### Workspace Layout

```
rust-notepad-plus/
├── Cargo.toml                    # Workspace root
├── src/main.rs                   # Entry point
│
├── crates/
│   ├── ui-core/                  # ★ Platform abstraction
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── traits.rs         # TextEditor, DialogProvider, etc.
│   │   │   ├── types.rs          # FileFilter, MessageType, etc.
│   │   │   ├── encoding.rs       # UTF-8/16, BOM, line endings
│   │   │   ├── recent_files.rs   # MRU list
│   │   │   └── app_state.rs      # Application state
│   │   └── Cargo.toml
│   │
│   ├── ui-windows/               # ★ Windows backend
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── main_window.rs    # Win32 window
│   │   │   ├── menu.rs           # Win32 menu
│   │   │   ├── editor_control.rs # Win32 edit control
│   │   │   ├── file_dialogs.rs   # Win32 file dialogs
│   │   │   ├── find_dialog.rs    # Find dialog
│   │   │   ├── replace_dialog.rs # Replace dialog
│   │   │   ├── goto_dialog.rs    # Go to line dialog
│   │   │   ├── statusbar.rs      # Status bar
│   │   │   ├── toolbar.rs        # Toolbar
│   │   │   ├── accelerators.rs   # Keyboard shortcuts
│   │   │   ├── command_handler.rs# Command dispatch
│   │   │   ├── window_updates.rs # Window title/status updates
│   │   │   └── global_state.rs   # State singleton
│   │   └── Cargo.toml
│   │
│   ├── ui-gtk/                   # ★ Linux backend
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── gtk_main_window.rs    # GTK ApplicationWindow
│   │   │   ├── gtk_text_editor.rs    # GTK TextView wrapper
│   │   │   ├── gtk_dialogs.rs        # GTK dialogs
│   │   │   ├── gtk_menu.rs           # GTK MenuBar
│   │   │   ├── gtk_statusbar.rs      # GTK status bar
│   │   │   └── app_state_manager.rs  # State singleton
│   │   └── Cargo.toml
│   │
│   ├── core/                     # Business logic
│   ├── editor/                   # Rope text buffer
│   ├── search/                   # Search engine
│   ├── lexer/                    # Syntax highlighter
│   ├── io/                       # File I/O
│   ├── config/                   # Config management
│   └── plugins/                  # Plugin system
│
├── BUILD.md                      # Build instructions
├── TESTING.md                    # Testing guide
├── ARCHITECTURE.md               # This file
└── CROSS_PLATFORM_PLAN.md        # Migration plan
```

### Dependency Graph

```
main
 ├─→ ui-windows (Windows only)
 │    └─→ ui-core
 ├─→ ui-gtk (Linux only)
 │    └─→ ui-core
 └─→ core, editor, search, lexer, io, config, plugins
       └─→ (no UI dependencies)

ui-core
 ├─→ core
 ├─→ editor
 └─→ search
```

**Key Insight**: Business logic crates never depend on UI crates.

---

## Platform Abstraction

### Trait Hierarchy

```
┌──────────────────────────────────────────┐
│            Platform Trait                │
│  fn initialize() -> Result<()>          │
│  fn run_event_loop() -> Result<()>     │
└──────────────────────────────────────────┘
                   △
                   │
         ┌─────────┴──────────┐
         │                    │
    Win32Platform        GtkPlatform
```

```
┌──────────────────────────────────────────┐
│          TextEditor Trait                │
│  fn set_text(&mut self, text: &str)     │
│  fn get_text(&self) -> String           │
│  fn undo(&mut self) -> bool             │
│  fn redo(&mut self) -> bool             │
│  ...                                     │
└──────────────────────────────────────────┘
                   △
                   │
         ┌─────────┴──────────┐
         │                    │
   Win32EditControl       GtkTextEditor
```

### Portable Types

Defined in `ui-core/src/types.rs`:

```rust
pub struct FileFilter {
    pub name: String,
    pub patterns: Vec<String>,
}

pub enum MessageType {
    Info,
    Warning,
    Error,
    Question,
}

pub enum DialogResult {
    Yes,
    No,
    Cancel,
}
```

Used by all platforms without modification.

### Portable Components

Defined in `ui-core/src/`:

#### 1. Encoding (`encoding.rs`)
- UTF-8, UTF-8 BOM, UTF-16 LE, UTF-16 BE, ANSI
- BOM detection: `detect_bom(&[u8]) -> Option<Encoding>`
- Line ending detection: `detect_line_ending(&str) -> LineEnding`
- Conversion: `convert_to_encoding(&str, Encoding, LineEnding) -> Vec<u8>`

#### 2. Recent Files (`recent_files.rs`)
- MRU (Most Recently Used) list
- Fixed capacity (default 10)
- Serialization support
- API: `add()`, `get_list()`, `clear()`

#### 3. Application State (`app_state.rs`)
- Current file, dirty flag
- Cursor position (line, column)
- Total lines
- Encoding, line ending
- Word wrap state
- Recent files list

---

## Data Flow

### File Open Flow

```
User clicks File → Open
    │
    ↓
DialogProvider::show_open_dialog()    [Platform-specific]
    │
    ↓
User selects file.txt
    │
    ↓
std::fs::read() → Vec<u8>             [Portable: io crate]
    │
    ↓
encoding::detect_encoding(&[u8])      [Portable: ui-core]
    │
    ↓
String content + Encoding + LineEnding
    │
    ↓
TextEditor::set_text(&str)            [Platform-specific]
    │
    ↓
AppState::update()                    [Portable: ui-core]
    │  - current_file = Some(path)
    │  - is_dirty = false
    │  - current_encoding = detected
    │  - current_line_ending = detected
    │
    ↓
StatusBar::update()                   [Platform-specific]
    │
    ↓
Window title updated                  [Platform-specific]
    │
    ↓
Recent files list updated             [Portable: ui-core]
```

### File Save Flow

```
User clicks File → Save
    │
    ↓
AppState: current_file?
    │
    ├─ None → show_save_dialog()      [Platform-specific]
    │           │
    │           ↓
    │        User enters path
    │
    └─ Some(path)
        │
        ↓
TextEditor::get_text() → String       [Platform-specific]
        │
        ↓
encoding::convert_to_encoding()       [Portable: ui-core]
  (String + Encoding + LineEnding → Vec<u8>)
        │
        ↓
std::fs::write(path, bytes)           [Portable: io crate]
        │
        ↓
AppState::update()                    [Portable: ui-core]
  - is_dirty = false
        │
        ↓
Window title updated (remove *)       [Platform-specific]
```

### Search Flow

```
User clicks Search → Find
    │
    ↓
DialogProvider::show_find_dialog()    [Platform-specific]
    │
    ↓
User enters search term + options
    │
    ↓
search::find(&str, &SearchOptions)    [Portable: search crate]
    │
    ↓
Vec<Match> results
    │
    ↓
TextEditor::set_selection()           [Platform-specific]
  (highlight first match)
```

---

## Component Details

### 1. main.rs - Entry Point

**Responsibility**: Platform detection and initialization

```rust
// Import platform backend
#[cfg(target_os = "windows")]
use notepad_ui_windows as ui_backend;

#[cfg(target_os = "linux")]
use notepad_ui_gtk as ui_backend;

// Platform-specific main()
#[cfg(target_os = "windows")]
fn main() -> Result<()> {
    let app = NotepadApp::new()?;
    ui_backend::init_global_state();
    let window = MainWindow::new(&app)?;
    window.show();
    // Win32 message loop
}

#[cfg(target_os = "linux")]
fn main() -> Result<()> {
    let app = NotepadApp::new()?;
    ui_backend::init_global_state();
    let window = GtkMainWindow::new(&app)?;
    window.show();
    window.run();  // GTK event loop
}
```

### 2. ui-core - Platform Abstraction

**Responsibility**: Define interfaces, implement portable logic

**Key Files**:
- `traits.rs`: Platform-agnostic trait definitions
- `types.rs`: Common data structures
- `encoding.rs`: Encoding detection and conversion (430 lines)
- `recent_files.rs`: MRU list (119 lines)
- `app_state.rs`: Application state (100+ lines)

**No Platform Dependencies**: This crate must compile on all platforms.

### 3. ui-windows - Windows Backend

**Responsibility**: Implement traits using Win32 API

**Key Technologies**:
- `windows` crate (Win32 API bindings)
- Edit control for text editing
- Win32 dialogs, menus, status bar
- Message loop with accelerators

**File Structure**:
- `main_window.rs`: Window creation, WndProc
- `editor_control.rs`: Edit control wrapper
- `menu.rs`: Menu bar construction
- `file_dialogs.rs`: GetOpenFileName, GetSaveFileName
- `statusbar.rs`: Status bar management
- `accelerators.rs`: Keyboard shortcuts

**State Management**: `global_state.rs` using `OnceLock<Mutex<AppState>>`

### 4. ui-gtk - Linux Backend

**Responsibility**: Implement traits using GTK4

**Key Technologies**:
- `gtk4` crate (GTK4 Rust bindings)
- `glib` for event loop
- TextView for text editing
- GTK dialogs, menus, status bar
- Action-based menu system

**File Structure**:
- `gtk_main_window.rs`: ApplicationWindow, event loop (488 lines)
- `gtk_text_editor.rs`: TextView wrapper (158 lines)
- `gtk_dialogs.rs`: FileChooser, MessageDialog, custom dialogs (312 lines)
- `gtk_menu.rs`: MenuBar with gio::Menu (88 lines)
- `gtk_statusbar.rs`: Custom status bar (87 lines)

**State Management**: `app_state_manager.rs` using `OnceLock<Mutex<AppState>>`

**GTK4 Specifics**:
- Uses `gio::SimpleAction` for menu actions
- Async dialog handling with `run_future()`
- Signal handlers for text changes

### 5. core - Business Logic

**Responsibility**: Application logic (platform-independent)

**Key Types**:
- `NotepadApp`: Main application state
- `Buffer`: Text buffer management
- `View`: Document view
- `Command`: Command pattern for undo/redo

### 6. editor - Text Buffer

**Responsibility**: Efficient text editing data structure

**Implementation**: Rope (balanced tree)
- Fast insert/delete at any position
- Efficient line indexing
- Memory-efficient for large files

**API**:
```rust
pub struct Rope { /* ... */ }

impl Rope {
    pub fn insert(&mut self, pos: usize, text: &str);
    pub fn delete(&mut self, start: usize, end: usize);
    pub fn slice(&self, start: usize, end: usize) -> String;
    pub fn line(&self, line_num: usize) -> Option<String>;
}
```

### 7. search - Search Engine

**Responsibility**: Find/replace operations

**Features**:
- Plain text search
- Regular expression search
- Case-sensitive toggle
- Whole word toggle
- Replace single/all

**API**:
```rust
pub struct SearchEngine { /* ... */ }

impl SearchEngine {
    pub fn find(&self, haystack: &str, needle: &str, options: &SearchOptions) -> Vec<Match>;
    pub fn replace(&self, text: &str, pattern: &str, replacement: &str) -> String;
}
```

---

## Adding New Platforms

### Adding macOS Support

**Step 1**: Create `crates/ui-cocoa/`

```bash
mkdir -p crates/ui-cocoa/src
```

**Step 2**: Add to workspace in `Cargo.toml`:

```toml
[workspace]
members = [
    "crates/ui-cocoa",  # Add this
    # ...
]

[target.'cfg(target_os = "macos")'.dependencies]
notepad-ui-cocoa = { path = "crates/ui-cocoa" }
objc2 = "0.5"  # Objective-C bindings
```

**Step 3**: Implement traits in `ui-cocoa/src/`:

```rust
// cocoa_text_editor.rs
pub struct CocoaTextEditor {
    ns_text_view: /* NSTextView reference */
}

impl TextEditor for CocoaTextEditor {
    fn set_text(&mut self, text: &str) {
        // Call NSTextView Objective-C methods
    }
    // ...
}
```

**Step 4**: Update `main.rs`:

```rust
#[cfg(target_os = "macos")]
use notepad_ui_cocoa as ui_backend;

#[cfg(target_os = "macos")]
fn main() -> Result<()> {
    // Initialize Cocoa
    let window = ui_backend::CocoaMainWindow::new()?;
    window.run();  // Cocoa event loop
}
```

**Step 5**: Test:

```bash
cargo build --target aarch64-apple-darwin
```

### Adding Web Support (via Tauri)

Similar process, but using `ui-tauri` crate with web view.

---

## Performance Considerations

### 1. Zero-Cost Abstraction

Traits compile to direct function calls (monomorphization):

```rust
// Source code
let editor: &dyn TextEditor = &gtk_editor;
editor.set_text("hello");

// Compiled (monomorphized)
gtk_editor.set_text("hello");  // Direct call, no indirection
```

### 2. State Access Optimization

Global state uses efficient locking:

```rust
// Bad: Multiple lock acquisitions
let file = with_state(|s| s.current_file.clone());
let dirty = with_state(|s| s.is_dirty);

// Good: Single lock acquisition
let (file, dirty) = with_state(|s| (s.current_file.clone(), s.is_dirty));
```

### 3. Text Buffer Efficiency

Rope data structure provides:
- O(log n) insert/delete
- O(log n) indexing
- No full-buffer copies

### 4. Search Optimization

Search engine uses:
- Boyer-Moore for plain text
- Regex crate (optimized state machine)
- Early termination for single matches

### 5. Platform-Specific Optimizations

**Windows**:
- Direct Win32 API calls (no abstraction overhead)
- Native Edit control (optimized for large text)

**Linux**:
- GTK4's optimized TextView
- Pango for text rendering
- GtkSourceView option for advanced features

---

## Thread Safety

### State Management

```rust
static GLOBAL_STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

// Thread-safe access
pub fn with_state<F, R>(f: F) -> R
where F: FnOnce(&mut AppState) -> R {
    let mut state = GLOBAL_STATE.get().unwrap().lock().unwrap();
    f(&mut *state)
}
```

**Thread Safety Guarantees**:
- `OnceLock`: Initialize once, immutable reference after
- `Mutex`: Exclusive access to mutable state
- No data races possible

### UI Thread Constraints

**Windows**: All UI operations must run on main thread
**Linux/GTK**: All GTK operations must run on main thread

**Solution**: Both backends run single-threaded event loops.

---

## Error Handling

### Error Types

```rust
// ui-core/src/types.rs
#[derive(Error, Debug)]
pub enum UiError {
    #[error("Platform initialization failed: {0}")]
    PlatformInit(String),

    #[error("Window creation failed: {0}")]
    WindowCreation(String),

    #[error("File operation failed: {0}")]
    FileOperation(#[from] std::io::Error),

    #[error("Encoding error: {0}")]
    Encoding(String),
}
```

### Error Propagation

```rust
// Using anyhow::Result for flexibility
pub fn open_file(path: &Path) -> anyhow::Result<String> {
    let bytes = std::fs::read(path)?;  // io::Error auto-converted
    let (text, _, _) = encoding::detect_encoding_and_convert(&bytes)?;
    Ok(text)
}
```

### User-Facing Errors

```rust
// Show error dialog on failure
match open_file(&path) {
    Ok(text) => { /* ... */ },
    Err(e) => {
        dialogs.show_message(
            "Error",
            &format!("Failed to open file: {}", e),
            MessageType::Error,
        );
    }
}
```

---

## Logging

### Log Levels

```rust
use log::{trace, debug, info, warn, error};

info!("Application started");           // Important events
debug!("File opened: {}", path);        // Development debugging
warn!("Slow operation detected");       // Potential issues
error!("Failed to save: {}", e);        // Failures
trace!("Function called: foo()");       // Detailed tracing
```

### Configuration

```bash
# Set log level
export RUST_LOG=info    # info, warn, error only
export RUST_LOG=debug   # debug and above
export RUST_LOG=trace   # everything
```

---

## Future Enhancements

### 1. Async File I/O
```rust
pub async fn open_file_async(path: &Path) -> Result<String> {
    tokio::fs::read(path).await?;
    // ...
}
```

### 2. Plugin System
```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn on_load(&mut self, app: &mut NotepadApp);
    fn on_text_changed(&self, text: &str);
}
```

### 3. Scripting Support
- Lua or Rhai scripting
- User automation
- Custom commands

### 4. Advanced Features
- Multiple tabs
- Split view
- Syntax highlighting
- Code folding
- Auto-completion

---

## References

- Rust Book: https://doc.rust-lang.org/book/
- windows-rs: https://github.com/microsoft/windows-rs
- gtk-rs: https://gtk-rs.org/
- Rope data structure: https://xi-editor.io/docs/rope_science_00.html

---

For questions or contributions, see CONTRIBUTING.md
