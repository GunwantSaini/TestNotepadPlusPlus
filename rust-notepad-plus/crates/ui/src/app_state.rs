//! Application state management

use crate::recent_files::RecentFiles;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub current_file: Option<PathBuf>,
    pub is_dirty: bool,
    pub cursor_line: usize,
    pub cursor_column: usize,
    pub total_lines: usize,
    pub recent_files: RecentFiles,
    pub word_wrap_enabled: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_file: None,
            is_dirty: false,
            cursor_line: 1,
            cursor_column: 1,
            total_lines: 1,
            recent_files: RecentFiles::new(),
            word_wrap_enabled: false,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current file path
    pub fn set_current_file(&mut self, path: Option<PathBuf>) {
        // Add to recent files if we're setting a real file
        if let Some(ref p) = path {
            self.recent_files.add_file(p.clone());
        }
        self.current_file = path;
        self.is_dirty = false;
    }

    /// Add a file to recent files without changing current file
    pub fn add_to_recent(&mut self, path: PathBuf) {
        self.recent_files.add_file(path);
    }

    /// Get recent files list
    pub fn get_recent_files(&self) -> &RecentFiles {
        &self.recent_files
    }

    /// Toggle word wrap
    pub fn toggle_word_wrap(&mut self) {
        self.word_wrap_enabled = !self.word_wrap_enabled;
    }

    /// Set word wrap state
    pub fn set_word_wrap(&mut self, enabled: bool) {
        self.word_wrap_enabled = enabled;
    }

    /// Mark the document as modified
    pub fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }

    /// Update cursor position
    pub fn update_cursor_position(&mut self, line: usize, column: usize, total_lines: usize) {
        self.cursor_line = line;
        self.cursor_column = column;
        self.total_lines = total_lines;
    }

    /// Get the window title
    pub fn get_window_title(&self) -> String {
        let filename = self.current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled");

        let dirty_marker = if self.is_dirty { "*" } else { "" };

        format!("{}{} - Notepad++ Rust Edition", filename, dirty_marker)
    }

    /// Get the position status text
    pub fn get_position_status(&self) -> String {
        format!("Ln {}, Col {}", self.cursor_line, self.cursor_column)
    }
}

/// Thread-safe application state
pub type SharedAppState = Arc<Mutex<AppState>>;

/// Create a new shared application state
pub fn create_shared_state() -> SharedAppState {
    Arc::new(Mutex::new(AppState::new()))
}
