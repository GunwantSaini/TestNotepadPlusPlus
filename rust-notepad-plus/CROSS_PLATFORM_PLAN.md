# Cross-Platform Architecture Plan

## Overview
Convert Notepad++ Rust Edition from Windows-only (Win32 API) to cross-platform (Windows + Linux + macOS).

## Current Status
- ✅ **Portable**: Core logic, search engine, editor, encoding, file I/O
- ❌ **Windows-only**: Entire UI layer (crates/ui)

## Architecture Strategy

### Phase 1: Restructure Crates (Current Branch)
Separate platform-independent code from platform-specific code.

**New Crate Structure:**
```
rust-notepad-plus/
├── crates/
│   ├── core/              ✅ Already portable - no changes needed
│   ├── search/            ✅ Already portable - no changes needed
│   ├── editor/            ✅ Already portable - no changes needed
│   ├── io/                ✅ Already portable - no changes needed
│   ├── plugins/           ✅ Already portable - no changes needed
│   │
│   ├── ui-core/           🆕 Platform-independent UI traits
│   │   ├── traits.rs      - TextEditor, DialogProvider, MenuBuilder, etc.
│   │   ├── events.rs      - Platform-independent event types
│   │   └── models.rs      - UI state models
│   │
│   ├── ui-windows/        🔧 Refactor current ui/ → ui-windows/
│   │   ├── win32_window.rs
│   │   ├── win32_editor.rs
│   │   ├── win32_dialogs.rs
│   │   └── win32_menus.rs
│   │
│   ├── ui-gtk/            🆕 Linux GTK4 backend
│   │   ├── gtk_window.rs
│   │   ├── gtk_editor.rs
│   │   ├── gtk_dialogs.rs
│   │   └── gtk_menus.rs
│   │
│   └── app/               🆕 Main application orchestrator
│       ├── lib.rs         - Platform detection, backend selection
│       └── controller.rs  - Business logic coordinator
```

### Phase 2: Define Platform Abstraction Traits

**Core Traits** (`crates/ui-core/src/traits.rs`):

```rust
/// Platform-independent text editor widget
pub trait TextEditor {
    fn set_text(&mut self, text: &str);
    fn get_text(&self) -> String;
    fn set_selection(&mut self, start: usize, end: usize);
    fn get_selection(&self) -> (usize, usize);
    fn undo(&mut self);
    fn redo(&mut self);
    fn cut(&mut self);
    fn copy(&mut self);
    fn paste(&mut self);
    fn set_word_wrap(&mut self, enabled: bool);
}

/// File dialogs
pub trait DialogProvider {
    fn show_open_dialog(&self, filters: &[FileFilter]) -> Option<PathBuf>;
    fn show_save_dialog(&self, default_name: &str, filters: &[FileFilter]) -> Option<PathBuf>;
    fn show_message(&self, title: &str, message: &str, msg_type: MessageType);
    fn show_question(&self, title: &str, message: &str) -> DialogResult;
}

/// Menu management
pub trait MenuBuilder {
    fn create_menu(&mut self, items: &[MenuItem]) -> MenuHandle;
    fn update_recent_files(&mut self, files: &[PathBuf]);
    fn set_menu_enabled(&mut self, id: CommandId, enabled: bool);
    fn set_menu_checked(&mut self, id: CommandId, checked: bool);
}

/// Main window
pub trait MainWindow {
    fn create(app: &AppController) -> Self;
    fn run(&mut self);
    fn set_title(&mut self, title: &str);
    fn get_editor(&mut self) -> &mut dyn TextEditor;
    fn get_dialogs(&self) -> &dyn DialogProvider;
    fn get_menu(&mut self) -> &mut dyn MenuBuilder;
    fn close(&mut self);
}
```

### Phase 3: Implementation Options

#### Option A: GTK4 for Linux (Recommended for Native Feel)

**Pros:**
- Native Linux integration
- Well-established, mature
- Good documentation (gtk-rs)
- Follows GNOME HIG guidelines

**Cons:**
- Different codebase for Windows vs Linux
- More maintenance overhead

**Dependencies:**
```toml
[target.'cfg(target_os = "linux")'.dependencies]
gtk4 = "0.7"
glib = "0.18"
```

**Implementation Effort:** ~2-3 weeks
- Week 1: Trait definitions, Windows refactor
- Week 2: GTK window, editor, menus
- Week 3: Dialogs, testing, polish

#### Option B: egui (Cross-Platform, Same Code Everywhere)

**Pros:**
- Single codebase for all platforms
- Modern immediate-mode UI
- Very active development
- Built-in widgets

**Cons:**
- Non-native look and feel
- Custom styling needed

**Dependencies:**
```toml
egui = "0.24"
eframe = { version = "0.24", features = ["default_fonts", "glow"] }
```

**Implementation Effort:** ~1-2 weeks
- Week 1: Port UI to egui widgets
- Week 2: Polish, testing

#### Option C: Slint (Declarative, Native Performance)

**Pros:**
- Declarative UI (like QML)
- Native rendering performance
- Good cross-platform support

**Cons:**
- Smaller ecosystem than GTK/egui
- Less mature

**Dependencies:**
```toml
slint = "1.3"
```

**Implementation Effort:** ~2 weeks

### Phase 4: Migration Steps

#### Step 1: Extract Portable UI State (Week 1)
1. Create `crates/ui-core/`
2. Move encoding.rs, recent_files.rs, app_state.rs to ui-core
3. Define platform traits
4. Update existing crates to use ui-core types

#### Step 2: Refactor Windows Backend (Week 1-2)
1. Rename `crates/ui/` → `crates/ui-windows/`
2. Implement traits for Win32 components
3. Update Cargo.toml with platform-specific dependencies
4. Test Windows build

#### Step 3: Implement Linux Backend (Week 2-3)
1. Create `crates/ui-gtk/` (or `ui-egui/`)
2. Implement all traits for GTK4
3. Create GTK window, text editor, dialogs
4. Add platform detection in main.rs

#### Step 4: Integration & Testing (Week 3-4)
1. Create unified app crate
2. Platform detection at compile-time
3. Test on Linux VM/container
4. CI/CD for both platforms

### Phase 5: Conditional Compilation

**Cargo.toml:**
```toml
[target.'cfg(target_os = "windows")'.dependencies]
notepad-ui-windows = { path = "crates/ui-windows" }

[target.'cfg(target_os = "linux")'.dependencies]
notepad-ui-gtk = { path = "crates/ui-gtk" }
```

**main.rs:**
```rust
#[cfg(target_os = "windows")]
use notepad_ui_windows as ui_backend;

#[cfg(target_os = "linux")]
use notepad_ui_gtk as ui_backend;

fn main() {
    let mut window = ui_backend::create_main_window();
    window.run();
}
```

## Timeline

### Conservative Estimate (GTK Approach)
- **Week 1**: Restructure, define traits, refactor Windows
- **Week 2**: Implement GTK backend (window, editor, menus)
- **Week 3**: Dialogs, file operations, encoding integration
- **Week 4**: Testing, bug fixes, documentation

### Aggressive Estimate (egui Approach)
- **Week 1**: Port to egui, basic window
- **Week 2**: Full feature parity, testing

## Testing Strategy

1. **Unit Tests**: All portable logic (already passing)
2. **Platform Tests**: Windows-specific, Linux-specific
3. **Integration Tests**: File operations, encoding, search
4. **Manual Testing**:
   - Ubuntu 22.04 LTS
   - Fedora 39
   - Arch Linux (latest)

## Risk Mitigation

1. **Parallel Development**: Keep Windows version working
2. **Feature Flags**: Enable/disable backends
3. **CI/CD**: GitHub Actions for multi-platform builds
4. **Incremental Migration**: One component at a time

## Recommendation

**I recommend Option A (GTK4) for the following reasons:**

1. ✅ **Native Linux Experience**: Follows GNOME HIG, feels like a Linux app
2. ✅ **Proven Technology**: gtk-rs is mature and well-documented
3. ✅ **Best Long-Term**: Separate backends allow platform-specific optimizations
4. ✅ **Professional**: Most professional Linux apps use GTK or Qt

**Fallback**: If GTK proves too complex, pivot to egui (Option B) for faster delivery.

## Next Steps (Immediate)

1. ✅ Create `feature/cross-platform-architecture` branch
2. ⬜ Create `crates/ui-core/` with trait definitions
3. ⬜ Rename `crates/ui/` → `crates/ui-windows/`
4. ⬜ Move portable modules to `ui-core/`
5. ⬜ Implement Win32 trait wrappers
6. ⬜ Test Windows build still works
7. ⬜ Start GTK implementation

## Success Criteria

- [ ] Windows build works (no regressions)
- [ ] Linux build compiles and runs
- [ ] All features work on both platforms
- [ ] No code duplication in portable logic
- [ ] CI/CD builds for both platforms
- [ ] Documentation updated

---

**Status**: Planning Phase
**Branch**: `feature/cross-platform-architecture`
**Target Completion**: 4 weeks from start
**Risk Level**: Medium (well-defined scope, proven technologies)
