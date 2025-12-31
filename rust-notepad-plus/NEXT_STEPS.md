# Next Steps for Production-Ready Features

## Overview

The following features are implemented but need integration:
- Global state management (`global_state.rs`)
- Window update helpers (`window_updates.rs`)
- App state tracking (`app_state.rs`)

##  Remaining Integration Work

### 1. Initialize Global State in main.rs

```rust
// In main() before creating MainWindow:
notepad_ui::init_global_state();
```

### 2. Update Main Window to Handle EN_CHANGE

Add to `window_proc` in `main_window.rs`:

```rust
WM_COMMAND => {
    let notification_code = (wparam.0 >> 16) & 0xFFFF;

    // EN_CHANGE = 0x0300
    if notification_code == 0x0300 {
        // Text changed - mark as dirty
        crate::global_state::with_state(|state| {
            state.set_dirty(true);
        });

        // Update window title
        crate::global_state::read_state(|state| {
            crate::window_updates::update_window_title(hwnd, state);
        });

        // Update cursor position
        let (line, col, total) = crate::window_updates::get_editor_cursor_position(hwnd);
        crate::global_state::with_state(|state| {
            state.update_cursor_position(line, col, total);
        });

        crate::global_state::read_state(|state| {
            crate::window_updates::update_status_bar_position(hwnd, state);
        });
    }

    // Existing command handling...
}
```

### 3. Add EN_SETSEL Handling for Cursor Movement

```rust
// In window_proc, add case for selection changes
const EN_SETSEL: u32 = 0x0702;
if notification_code == EN_SETSEL {
    let (line, col, total) = crate::window_updates::get_editor_cursor_position(hwnd);
    crate::global_state::with_state(|state| {
        state.update_cursor_position(line, col, total);
    });
    crate::global_state::read_state(|state| {
        crate::window_updates::update_status_bar_position(hwnd, state);
    });
}
```

### 4. Update File Operations in command_handler.rs

**FileOpen:**
```rust
if let Some(file_path) = show_open_dialog(hwnd) {
    // ... existing file loading code ...

    // Update state
    crate::global_state::with_state(|state| {
        state.set_current_file(Some(file_path.clone()));
        state.set_dirty(false);
    });

    // Update UI
    crate::global_state::read_state(|state| {
        crate::window_updates::update_window_title(hwnd, state);
    });
}
```

**FileSave:**
```rust
// After successful save
crate::global_state::with_state(|state| {
    state.set_current_file(Some(file_path.clone()));
    state.set_dirty(false);
});

crate::global_state::read_state(|state| {
    crate::window_updates::update_window_title(hwnd, state);
});
```

### 5. Add Unsaved Changes Prompts

Add helper function in `window_updates.rs`:

```rust
pub unsafe fn prompt_save_changes(hwnd: HWND) -> i32 {
    use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_YESNOCANCEL, MB_ICONQUESTION, IDYES, IDNO, IDCANCEL};

    let msg = "Do you want to save changes?";
    let msg_wide: Vec<u16> = msg.encode_utf16().chain(std::iter::once(0)).collect();
    let title_wide: Vec<u16> = "Notepad++".encode_utf16().chain(std::iter::once(0)).collect();

    MessageBoxW(
        hwnd,
        PCWSTR(msg_wide.as_ptr()),
        PCWSTR(title_wide.as_ptr()),
        MB_YESNOCANCEL | MB_ICONQUESTION,
    ).0
}
```

Use in `command_handler.rs`:

```rust
CommandId::FileNew => {
    // Check if dirty
    let is_dirty = crate::global_state::read_state(|s| s.is_dirty);

    if is_dirty {
        let result = crate::window_updates::prompt_save_changes(hwnd);
        if result == 6 { // IDYES
            // Save file first
            handle_command(hwnd, CommandId::FileSave);
        } else if result == 2 { // IDCANCEL
            return false;
        }
    }

    // Clear editor
    // ... existing code ...

    // Reset state
    crate::global_state::with_state(|state| {
        state.set_current_file(None);
        state.set_dirty(false);
    });
}
```

### 6. Add WM_CLOSE Handler

In `window_proc`:

```rust
const WM_CLOSE: u32 = 0x0010;

match msg {
    WM_CLOSE => {
        let is_dirty = crate::global_state::read_state(|s| s.is_dirty);

        if is_dirty {
            let result = crate::window_updates::prompt_save_changes(hwnd);
            if result == 6 { // IDYES
                // Save and close
                if command_handler::handle_command(hwnd, CommandId::FileSave) {
                    DestroyWindow(hwnd);
                }
            } else if result == 7 { // IDNO
                DestroyWindow(hwnd);
            }
            // IDCANCEL - do nothing
        } else {
            DestroyWindow(hwnd);
        }
        LRESULT(0)
    }
    // ...
}
```

## Testing Checklist

After integration:

- [ ] Window title shows "Untitled - Notepad++" on startup
- [ ] Typing text marks window as dirty (adds *)
- [ ] Status bar shows "Ln 1, Col 1" and updates as cursor moves
- [ ] Opening file updates title to show filename
- [ ] Saving file removes * from title
- [ ] File → New prompts to save if dirty
- [ ] File → Open prompts to save if dirty
- [ ] File → Exit prompts to save if dirty
- [ ] Cursor position updates in real-time

## Build & Test

```bash
cargo build --release
cargo test
```

## Performance Notes

- Use debouncing for EN_CHANGE if performance is an issue
- Consider async UI updates for large files
- Status bar updates are lightweight (< 1ms)
