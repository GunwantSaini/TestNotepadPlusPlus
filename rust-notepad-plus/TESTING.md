# Testing Guide - Notepad++ Rust Edition

This document provides comprehensive testing procedures for the cross-platform Notepad++ Rust Edition.

## Testing Overview

### Testing Phases
1. **Build Verification** - Ensure compilation succeeds
2. **Unit Tests** - Test individual components
3. **Integration Tests** - Test component interactions
4. **Manual Testing** - User-facing functionality
5. **Cross-Platform Testing** - Platform-specific behavior
6. **Performance Testing** - Benchmarks and profiling

---

## 1. Build Verification

### Compile All Crates
```bash
# Check all workspace crates compile
cargo check --workspace

# Build all workspace crates
cargo build --workspace

# Build with all features
cargo build --all-features
```

### Platform-Specific Builds
```bash
# Linux: Build GTK backend
cargo build -p notepad-ui-gtk

# Windows: Build Win32 backend (on Windows)
cargo build -p notepad-ui-windows

# Platform-independent: Always should compile
cargo build -p notepad-ui-core
cargo build -p notepad-core
cargo build -p notepad-editor
```

### Expected Output
```
✅ Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

---

## 2. Unit Tests

### Run All Tests
```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for specific crate
cargo test -p notepad-core
cargo test -p notepad-editor
cargo test -p notepad-search
```

### Core Components Testing
```bash
# Test encoding detection
cargo test -p notepad-ui-core encoding

# Test recent files (MRU)
cargo test -p notepad-ui-core recent_files

# Test application state
cargo test -p notepad-ui-core app_state

# Test rope editor
cargo test -p notepad-editor

# Test search engine
cargo test -p notepad-search
```

### Expected Test Coverage
| Component | Test Coverage | Status |
|-----------|---------------|--------|
| encoding.rs | Encoding detection, BOM, line endings | ✅ |
| recent_files.rs | MRU list operations | ✅ |
| app_state.rs | State management | ✅ |
| editor (rope) | Insert, delete, search | ✅ |
| search | Find, replace, regex | ✅ |

---

## 3. Integration Tests

### File Operations Test Suite

Create test file: `tests/file_operations.rs`

```rust
#[test]
fn test_new_file() {
    // Create new file
    // Verify empty content
    // Verify clean state (not dirty)
}

#[test]
fn test_open_utf8_file() {
    // Open UTF-8 file
    // Verify content matches
    // Verify encoding detected as UTF-8
}

#[test]
fn test_save_file() {
    // Create content
    // Save to file
    // Verify file exists
    // Verify content matches
}

#[test]
fn test_save_as() {
    // Open file
    // Modify content
    // Save as new file
    // Verify both files exist
}
```

Run:
```bash
cargo test --test file_operations
```

### Encoding Test Suite

```bash
# Create test files with various encodings
echo "UTF-8 content" > test_utf8.txt
echo -e "\xFF\xFEU\x00T\x00F\x00-\x001\x006\x00" > test_utf16le.txt

# Test encoding detection
cargo run -- test_utf8.txt
# Verify: Status bar shows "UTF-8"

cargo run -- test_utf16le.txt
# Verify: Status bar shows "UTF-16 LE"
```

---

## 4. Manual Testing Checklist

### 4.1 File Operations

#### New File
- [ ] Launch application
- [ ] Click File → New (or Ctrl+N)
- [ ] Verify: Empty editor
- [ ] Verify: Window title shows "Untitled - Notepad++"
- [ ] Verify: Status bar shows "Ln: 1 Col: 1"

#### Open File
- [ ] Click File → Open (or Ctrl+O)
- [ ] Select text file
- [ ] Verify: File content displayed
- [ ] Verify: Window title shows filename
- [ ] Verify: Recent files list updated
- [ ] Verify: Encoding detected correctly

#### Save File
- [ ] Type some text
- [ ] Click File → Save (or Ctrl+S)
- [ ] Enter filename
- [ ] Verify: File saved to disk
- [ ] Verify: Dirty flag cleared (no * in title)
- [ ] Close and reopen file
- [ ] Verify: Content matches

#### Save As
- [ ] Open existing file
- [ ] Modify content
- [ ] Click File → Save As
- [ ] Enter new filename
- [ ] Verify: New file created
- [ ] Verify: Original file unchanged
- [ ] Verify: Window title updated

#### Exit
- [ ] Make changes without saving
- [ ] Click File → Exit
- [ ] Verify: "Save changes?" dialog appears
- [ ] Click Yes → Verify: File saved
- [ ] Repeat, click No → Verify: Changes discarded
- [ ] Repeat, click Cancel → Verify: Exit cancelled

---

### 4.2 Edit Operations

#### Undo / Redo
- [ ] Type text
- [ ] Click Edit → Undo (or Ctrl+Z)
- [ ] Verify: Last action undone
- [ ] Click Edit → Redo (or Ctrl+Y)
- [ ] Verify: Last action redone

**Note**: GTK backend may show "not yet implemented" - this is expected.

#### Cut / Copy / Paste
- [ ] Select text
- [ ] Click Edit → Cut (or Ctrl+X)
- [ ] Verify: Text removed
- [ ] Move cursor
- [ ] Click Edit → Paste (or Ctrl+V)
- [ ] Verify: Text pasted

- [ ] Select text
- [ ] Click Edit → Copy (or Ctrl+C)
- [ ] Verify: Text remains
- [ ] Move cursor
- [ ] Click Edit → Paste (or Ctrl+V)
- [ ] Verify: Text pasted

#### Select All
- [ ] Type multiple lines
- [ ] Click Edit → Select All (or Ctrl+A)
- [ ] Verify: All text selected
- [ ] Verify: Status bar shows selection range

---

### 4.3 Search Operations

#### Find
- [ ] Open file with content
- [ ] Click Search → Find (or Ctrl+F)
- [ ] Enter search text
- [ ] Click "Find Next"
- [ ] Verify: First occurrence highlighted
- [ ] Click "Find Next" again
- [ ] Verify: Next occurrence highlighted
- [ ] Test: Case sensitive toggle
- [ ] Test: Whole word toggle

#### Replace
- [ ] Click Search → Replace (or Ctrl+H)
- [ ] Enter find text
- [ ] Enter replace text
- [ ] Click "Replace"
- [ ] Verify: Single occurrence replaced
- [ ] Click "Replace All"
- [ ] Verify: All occurrences replaced

#### Go To Line
- [ ] Open file with 100+ lines
- [ ] Click Search → Go To Line (or Ctrl+G)
- [ ] Enter line number (e.g., 50)
- [ ] Verify: Cursor moves to line 50
- [ ] Verify: Status bar shows "Ln: 50"

---

### 4.4 View Options

#### Word Wrap
- [ ] Open file with long lines
- [ ] Verify: Horizontal scrollbar visible
- [ ] Click View → Word Wrap
- [ ] Verify: Long lines wrapped
- [ ] Verify: No horizontal scrollbar
- [ ] Click View → Word Wrap again
- [ ] Verify: Word wrap disabled

---

### 4.5 Encoding Operations

#### Encoding Detection
- [ ] Create UTF-8 file
- [ ] Open in editor
- [ ] Verify: Status bar shows "UTF-8"

- [ ] Create UTF-8 BOM file
- [ ] Open in editor
- [ ] Verify: Status bar shows "UTF-8 BOM"

- [ ] Create UTF-16 LE file
- [ ] Open in editor
- [ ] Verify: Status bar shows "UTF-16 LE"

#### Encoding Conversion
- [ ] Open UTF-8 file
- [ ] Click Encoding → UTF-16 LE
- [ ] Save file
- [ ] Verify: File saved as UTF-16 LE
- [ ] Close and reopen
- [ ] Verify: Status bar shows "UTF-16 LE"

---

### 4.6 Line Ending Operations

#### Line Ending Detection
- [ ] Create Windows CRLF file
- [ ] Open in editor
- [ ] Verify: Status bar shows "CRLF"

- [ ] Create Unix LF file
- [ ] Open in editor
- [ ] Verify: Status bar shows "LF"

#### Line Ending Conversion
- [ ] Open CRLF file
- [ ] Click Line Endings → Unix (LF)
- [ ] Save file
- [ ] Verify: File saved with LF endings
- [ ] Hexdump file: `hexdump -C filename`
- [ ] Verify: Only 0x0A (LF), no 0x0D (CR)

---

### 4.7 Recent Files

#### MRU List
- [ ] Open file A
- [ ] Close
- [ ] Open file B
- [ ] Close
- [ ] Open file C
- [ ] Click File menu
- [ ] Verify: Recent files shows C, B, A (in order)
- [ ] Click file B in recent files
- [ ] Verify: File B opens
- [ ] Click File menu
- [ ] Verify: Recent files shows B, C, A (B moved to top)

---

### 4.8 Status Bar

#### Cursor Position
- [ ] Type text
- [ ] Move cursor with arrow keys
- [ ] Verify: Status bar updates in real-time
- [ ] Verify: "Ln: X Col: Y" shows correct position
- [ ] Click at random position
- [ ] Verify: Status bar updates immediately

#### Encoding Indicator
- [ ] Open different encoded files
- [ ] Verify: Encoding indicator matches file encoding
- [ ] Change encoding via menu
- [ ] Verify: Status bar updates

#### Line Ending Indicator
- [ ] Open files with different line endings
- [ ] Verify: Line ending indicator shows CRLF/LF/CR
- [ ] Change line ending via menu
- [ ] Verify: Status bar updates

---

## 5. Cross-Platform Testing

### 5.1 Linux-Specific Testing

#### GTK4 Integration
- [ ] Launch application
- [ ] Verify: GTK4 window appears
- [ ] Verify: Native GTK theme applied
- [ ] Verify: System clipboard integration works
- [ ] Verify: File dialogs use GTK FileChooser
- [ ] Verify: Keyboard shortcuts work (Ctrl+N, Ctrl+O, etc.)

#### Desktop Integration
- [ ] Create .desktop file
- [ ] Install to ~/.local/share/applications/
- [ ] Verify: Appears in application menu
- [ ] Launch from menu
- [ ] Verify: Application starts correctly

#### Multiple Distros
Test on:
- [ ] Ubuntu 22.04 LTS
- [ ] Fedora 39
- [ ] Arch Linux (rolling)
- [ ] Debian 12

### 5.2 Windows-Specific Testing

#### Win32 Integration
- [ ] Launch application
- [ ] Verify: Native Windows window appears
- [ ] Verify: Windows theme applied
- [ ] Verify: System clipboard integration works
- [ ] Verify: File dialogs use Windows native dialogs
- [ ] Verify: Keyboard shortcuts work (Ctrl+N, Ctrl+O, etc.)
- [ ] Verify: Accelerator keys work (Alt+F for File menu)

#### Windows Versions
Test on:
- [ ] Windows 11
- [ ] Windows 10 (latest)
- [ ] Windows 10 (older builds)

---

## 6. Performance Testing

### 6.1 Large File Handling

#### Open Large File
```bash
# Create 10MB test file
yes "Lorem ipsum dolor sit amet" | head -n 100000 > large_test.txt

# Test opening
time cargo run -- large_test.txt
```

Expected: Opens in < 1 second

#### Edit Large File
- [ ] Open 10MB file
- [ ] Add text at beginning
- [ ] Verify: Instant response
- [ ] Add text at end
- [ ] Verify: Instant response
- [ ] Search in file
- [ ] Verify: Results in < 100ms

### 6.2 Search Performance

```bash
# Create test file with pattern
python3 -c "print('test\n' * 100000)" > search_test.txt

# Benchmark search
cargo run -- search_test.txt
# Search for "test"
# Measure time to find all occurrences
```

Expected: < 50ms for 100K lines

### 6.3 Memory Usage

```bash
# Monitor memory while running
/usr/bin/time -v cargo run -- large_test.txt
```

Expected: < 100MB for 10MB file

### 6.4 Startup Time

```bash
# Cold start
time cargo run

# Warm start (second run)
time cargo run
```

Expected:
- Cold start: < 200ms
- Warm start: < 100ms

---

## 7. Regression Testing

### Automated Regression Suite

Create `tests/regression.rs`:

```rust
#[test]
fn test_encoding_regression_utf8_bom() {
    // Regression: UTF-8 BOM detection failed in v0.1
    // Fixed in v0.2
}

#[test]
fn test_word_wrap_crash() {
    // Regression: Word wrap caused crash with long lines
    // Fixed in v0.3
}
```

### Manual Regression Checklist
- [ ] All Phase 1 features still work
- [ ] All Phase 2 features still work
- [ ] All Phase 3 features still work
- [ ] No new crashes introduced
- [ ] No performance degradation

---

## 8. Stress Testing

### 8.1 File Operations Stress Test
```bash
# Open/close 1000 times
for i in {1..1000}; do
    cargo run -- test.txt &
    sleep 0.1
    killall notepad-plus
done
```

Expected: No memory leaks, no crashes

### 8.2 Edit Operations Stress Test
- [ ] Type continuously for 5 minutes
- [ ] Verify: No lag
- [ ] Verify: No memory growth
- [ ] Verify: Undo stack maintained

### 8.3 Search Stress Test
- [ ] Search 1000 times in large file
- [ ] Verify: Consistent performance
- [ ] Verify: No memory leaks

---

## 9. Error Handling Testing

### 9.1 File I/O Errors
- [ ] Try to open non-existent file
- [ ] Verify: Error dialog shown
- [ ] Try to save to read-only directory
- [ ] Verify: Error dialog shown
- [ ] Try to open binary file
- [ ] Verify: Warning shown

### 9.2 Invalid Encoding
- [ ] Open file with invalid UTF-8
- [ ] Verify: Fallback to ANSI or error
- [ ] Try to save with invalid encoding
- [ ] Verify: Error shown

### 9.3 Disk Full
- [ ] Fill disk to 100%
- [ ] Try to save file
- [ ] Verify: Error dialog shown
- [ ] Verify: Original file not corrupted

---

## 10. Accessibility Testing

### Keyboard Navigation
- [ ] Navigate menus with keyboard only
- [ ] Verify: All features accessible
- [ ] Verify: Tab order logical
- [ ] Verify: Shortcuts consistent

### Screen Reader
- [ ] Enable screen reader (Orca on Linux, NVDA on Windows)
- [ ] Navigate application
- [ ] Verify: UI elements announced
- [ ] Verify: Dialogs announced

---

## 11. CI/CD Testing

### GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install GTK4
        run: sudo apt-get install -y libgtk-4-dev
      - name: Build
        run: cargo build --release
      - name: Test
        run: cargo test --workspace

  test-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: cargo build --release
      - name: Test
        run: cargo test --workspace
```

---

## 12. Test Reporting

### Test Report Template

```markdown
# Test Report - Notepad++ Rust Edition

**Version**: 8.0.0
**Date**: YYYY-MM-DD
**Platform**: Linux / Windows
**Tester**: Name

## Summary
- Tests Passed: X / Y
- Tests Failed: X / Y
- Coverage: X%

## Detailed Results

### Build Verification
✅ All crates compile
✅ No warnings

### Unit Tests
✅ encoding tests (12/12)
✅ recent_files tests (8/8)
❌ undo_redo tests (0/5) - Not implemented

### Manual Tests
✅ File operations (5/5)
✅ Edit operations (4/5)
❌ Undo/Redo (0/1) - Not working on GTK

## Issues Found
1. Undo/Redo not implemented on GTK backend
2. ...

## Recommendations
1. Implement GtkSourceView for undo/redo
2. ...
```

---

## Quick Test Commands

```bash
# Full test suite
cargo test --workspace

# Build all platforms
cargo build --workspace

# Run with logging
RUST_LOG=debug cargo run

# Performance test
time cargo run -- large_file.txt

# Memory test
/usr/bin/time -v cargo run
```

---

## Test Coverage Goals

| Component | Target | Current |
|-----------|--------|---------|
| encoding.rs | 90% | 85% |
| recent_files.rs | 90% | 80% |
| app_state.rs | 80% | 75% |
| gtk_main_window.rs | 70% | 60% |
| Overall | 75% | 70% |

---

For issues found during testing, report at:
https://github.com/your-org/notepad-plus-rust/issues
