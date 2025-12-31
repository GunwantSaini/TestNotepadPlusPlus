//! Document buffer management
//!
//! Corresponds to the C++ Buffer class (PowerEditor/src/ScintillaComponent/Buffer.h)

use crate::app::{EolFormat, EncodingType};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique buffer identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(u64);

static BUFFER_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

impl BufferId {
    pub fn new() -> Self {
        Self(BUFFER_ID_COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

/// Document buffer
///
/// Represents an open file/document with its metadata and content.
/// In the C++ version, this manages the Scintilla document pointer.
#[derive(Debug)]
pub struct Buffer {
    /// Unique identifier
    id: BufferId,

    /// File path (None for new, unsaved documents)
    file_path: Option<PathBuf>,

    /// Display name (e.g., "new 1")
    display_name: String,

    /// Document content
    content: String,

    /// Dirty flag (has unsaved changes)
    is_dirty: bool,

    /// Read-only flag
    is_readonly: bool,

    /// Encoding type
    encoding: EncodingType,

    /// End-of-line format
    eol_format: EolFormat,

    /// Language/lexer type
    language: String,

    /// Timestamp of last file modification
    file_timestamp: Option<std::time::SystemTime>,

    /// Position info (cursor, scroll, etc.)
    position_info: PositionInfo,
}

#[derive(Debug, Clone, Default)]
pub struct PositionInfo {
    /// Cursor position (character offset)
    pub cursor_pos: usize,

    /// Selection start
    pub selection_start: Option<usize>,

    /// Selection end
    pub selection_end: Option<usize>,

    /// First visible line
    pub first_visible_line: usize,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new_empty() -> Self {
        static NEW_COUNTER: AtomicU64 = AtomicU64::new(1);
        let counter = NEW_COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            id: BufferId::new(),
            file_path: None,
            display_name: format!("new {}", counter),
            content: String::new(),
            is_dirty: false,
            is_readonly: false,
            encoding: EncodingType::Utf8,
            eol_format: EolFormat::Windows,
            language: String::from("Normal Text"),
            file_timestamp: None,
            position_info: PositionInfo::default(),
        }
    }

    /// Create a buffer from a file
    pub fn from_file(path: PathBuf, content: String, encoding: EncodingType) -> Self {
        let display_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let file_timestamp = std::fs::metadata(&path)
            .ok()
            .and_then(|m| m.modified().ok());

        let eol_format = Self::detect_eol_format(&content);

        Self {
            id: BufferId::new(),
            file_path: Some(path),
            display_name,
            content,
            is_dirty: false,
            is_readonly: false,
            encoding,
            eol_format,
            language: String::from("Normal Text"), // Will be detected later
            file_timestamp,
            position_info: PositionInfo::default(),
        }
    }

    /// Detect EOL format from content
    fn detect_eol_format(content: &str) -> EolFormat {
        if content.contains("\r\n") {
            EolFormat::Windows
        } else if content.contains('\n') {
            EolFormat::Unix
        } else if content.contains('\r') {
            EolFormat::Mac
        } else {
            EolFormat::Windows // Default
        }
    }

    // Getters
    pub fn id(&self) -> BufferId {
        self.id
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn is_readonly(&self) -> bool {
        self.is_readonly
    }

    pub fn encoding(&self) -> EncodingType {
        self.encoding
    }

    pub fn eol_format(&self) -> EolFormat {
        self.eol_format
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    // Setters
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.is_dirty = true;
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        self.is_readonly = readonly;
    }

    pub fn set_encoding(&mut self, encoding: EncodingType) {
        self.encoding = encoding;
        self.is_dirty = true;
    }

    pub fn set_eol_format(&mut self, format: EolFormat) {
        self.eol_format = format;
        self.is_dirty = true;
    }

    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }

    pub fn set_file_path(&mut self, path: PathBuf) {
        self.display_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        self.file_path = Some(path);
    }

    /// Mark buffer as saved
    pub fn mark_saved(&mut self) {
        self.is_dirty = false;
        if let Some(path) = &self.file_path {
            self.file_timestamp = std::fs::metadata(path)
                .ok()
                .and_then(|m| m.modified().ok());
        }
    }

    /// Check if file has been modified externally
    pub fn is_modified_externally(&self) -> bool {
        if let (Some(path), Some(timestamp)) = (&self.file_path, self.file_timestamp) {
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(current_time) = metadata.modified() {
                    return current_time > timestamp;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::new_empty();
        assert!(buffer.id().0 > 0);
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.encoding(), EncodingType::Utf8);
    }

    #[test]
    fn test_eol_detection() {
        let windows_content = "Line 1\r\nLine 2\r\n";
        let unix_content = "Line 1\nLine 2\n";
        let mac_content = "Line 1\rLine 2\r";

        assert_eq!(Buffer::detect_eol_format(windows_content), EolFormat::Windows);
        assert_eq!(Buffer::detect_eol_format(unix_content), EolFormat::Unix);
        assert_eq!(Buffer::detect_eol_format(mac_content), EolFormat::Mac);
    }

    #[test]
    fn test_dirty_flag() {
        let mut buffer = Buffer::new_empty();
        assert!(!buffer.is_dirty());

        buffer.set_content("Hello".to_string());
        assert!(buffer.is_dirty());

        buffer.mark_saved();
        assert!(!buffer.is_dirty());
    }
}
