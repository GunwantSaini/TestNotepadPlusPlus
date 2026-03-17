//! Integration tests for file operations
//!
//! Tests the complete file lifecycle: create, open, edit, save, close

use notepad_core::{NotepadApp, EncodingType, EolFormat};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Create a temporary directory for test files
fn setup_test_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

/// Create a test file with content
fn create_test_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let path = dir.path().join(name);
    fs::write(&path, content).expect("Failed to write test file");
    path
}

#[test]
fn test_open_and_read_file() {
    // Setup
    let temp_dir = setup_test_dir();
    let test_content = "Hello, Notepad++!\nThis is a test file.";
    let test_file = create_test_file(&temp_dir, "test.txt", test_content);

    // Create app and open file
    let app = NotepadApp::new().expect("Failed to create app");
    let buffer_id = app.open_file(test_file.clone()).expect("Failed to open file");

    // Verify
    let buffer = app.get_buffer(buffer_id).expect("Buffer not found");
    assert_eq!(buffer.content(), test_content);
    assert_eq!(buffer.display_name(), "test.txt");
    assert_eq!(buffer.file_path(), Some(&test_file));
    assert!(!buffer.is_dirty());
    assert_eq!(buffer.encoding(), EncodingType::Utf8);
}

#[test]
fn test_save_file() {
    // Setup
    let temp_dir = setup_test_dir();
    let original_content = "Original content";
    let test_file = create_test_file(&temp_dir, "save_test.txt", original_content);

    // Open file and modify
    let app = NotepadApp::new().expect("Failed to create app");
    let buffer_id = app.open_file(test_file.clone()).expect("Failed to open file");

    // Note: In the current implementation, we can't directly modify buffer content
    // This test verifies the save mechanism works with the file manager
    app.save_buffer(buffer_id).expect("Failed to save buffer");

    // Verify file still exists
    assert!(test_file.exists());
    let saved_content = fs::read_to_string(&test_file).expect("Failed to read saved file");
    assert_eq!(saved_content, original_content);
}

#[test]
fn test_close_buffer() {
    // Setup
    let temp_dir = setup_test_dir();
    let test_file = create_test_file(&temp_dir, "close_test.txt", "Test content");

    // Open and close
    let app = NotepadApp::new().expect("Failed to create app");
    let buffer_id = app.open_file(test_file).expect("Failed to open file");

    assert!(app.get_buffer(buffer_id).is_some(), "Buffer should exist");

    app.close_buffer(buffer_id).expect("Failed to close buffer");

    assert!(app.get_buffer(buffer_id).is_none(), "Buffer should be closed");
}

#[test]
fn test_multiple_files() {
    // Setup
    let temp_dir = setup_test_dir();
    let file1 = create_test_file(&temp_dir, "file1.txt", "Content 1");
    let file2 = create_test_file(&temp_dir, "file2.txt", "Content 2");
    let file3 = create_test_file(&temp_dir, "file3.txt", "Content 3");

    // Open multiple files
    let app = NotepadApp::new().expect("Failed to create app");
    let id1 = app.open_file(file1).expect("Failed to open file1");
    let id2 = app.open_file(file2).expect("Failed to open file2");
    let id3 = app.open_file(file3).expect("Failed to open file3");

    // Verify all are open
    let all_buffers = app.get_all_buffers();
    assert_eq!(all_buffers.len(), 3, "Should have 3 open buffers");
    assert!(all_buffers.contains(&id1));
    assert!(all_buffers.contains(&id2));
    assert!(all_buffers.contains(&id3));

    // Close one and verify
    app.close_buffer(id2).expect("Failed to close file2");
    let remaining = app.get_all_buffers();
    assert_eq!(remaining.len(), 2, "Should have 2 buffers after close");
    assert!(!remaining.contains(&id2));
}

#[test]
fn test_active_buffer_tracking() {
    // Setup
    let temp_dir = setup_test_dir();
    let test_file = create_test_file(&temp_dir, "active_test.txt", "Test");

    // Open file
    let app = NotepadApp::new().expect("Failed to create app");
    let buffer_id = app.open_file(test_file).expect("Failed to open file");

    // The first opened file should become active in main view
    let active = app.get_active_buffer();
    assert!(active.is_some(), "Should have an active buffer");
    assert_eq!(active.unwrap(), buffer_id, "Opened buffer should be active");
}

#[test]
fn test_eol_format_detection() {
    // Setup
    let temp_dir = setup_test_dir();

    // Test Windows EOL (CRLF)
    let windows_file = create_test_file(&temp_dir, "windows.txt", "Line 1\r\nLine 2\r\n");
    let app = NotepadApp::new().expect("Failed to create app");
    let id = app.open_file(windows_file).expect("Failed to open");
    let buffer = app.get_buffer(id).expect("Buffer not found");
    assert_eq!(buffer.eol_format(), EolFormat::Windows);

    // Test Unix EOL (LF)
    let unix_file = create_test_file(&temp_dir, "unix.txt", "Line 1\nLine 2\n");
    let id = app.open_file(unix_file).expect("Failed to open");
    let buffer = app.get_buffer(id).expect("Buffer not found");
    assert_eq!(buffer.eol_format(), EolFormat::Unix);

    // Test Mac EOL (CR)
    let mac_file = create_test_file(&temp_dir, "mac.txt", "Line 1\rLine 2\r");
    let id = app.open_file(mac_file).expect("Failed to open");
    let buffer = app.get_buffer(id).expect("Buffer not found");
    assert_eq!(buffer.eol_format(), EolFormat::Mac);
}

#[test]
fn test_nonexistent_file() {
    let app = NotepadApp::new().expect("Failed to create app");
    let result = app.open_file(PathBuf::from("/nonexistent/path/to/file.txt"));

    assert!(result.is_err(), "Opening nonexistent file should fail");
}

#[test]
fn test_utf8_encoding() {
    // Setup
    let temp_dir = setup_test_dir();
    let utf8_content = "Hello 世界! 🦀 Rust";
    let test_file = create_test_file(&temp_dir, "utf8.txt", utf8_content);

    // Open and verify
    let app = NotepadApp::new().expect("Failed to create app");
    let buffer_id = app.open_file(test_file).expect("Failed to open file");
    let buffer = app.get_buffer(buffer_id).expect("Buffer not found");

    assert_eq!(buffer.content(), utf8_content);
    assert_eq!(buffer.encoding(), EncodingType::Utf8);
}

#[test]
fn test_empty_file() {
    // Setup
    let temp_dir = setup_test_dir();
    let test_file = create_test_file(&temp_dir, "empty.txt", "");

    // Open and verify
    let app = NotepadApp::new().expect("Failed to create app");
    let buffer_id = app.open_file(test_file).expect("Failed to open file");
    let buffer = app.get_buffer(buffer_id).expect("Buffer not found");

    assert_eq!(buffer.content(), "");
    assert!(!buffer.is_dirty());
}

#[test]
fn test_large_file() {
    // Setup
    let temp_dir = setup_test_dir();
    let large_content = "A".repeat(10_000) + "\n" + &"B".repeat(10_000);
    let test_file = create_test_file(&temp_dir, "large.txt", &large_content);

    // Open and verify
    let app = NotepadApp::new().expect("Failed to create app");
    let buffer_id = app.open_file(test_file).expect("Failed to open file");
    let buffer = app.get_buffer(buffer_id).expect("Buffer not found");

    assert_eq!(buffer.content().len(), large_content.len());
}
