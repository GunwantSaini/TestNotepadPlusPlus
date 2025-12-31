//! Application state management

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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_file: None,
            is_dirty: false,
            cursor_line: 1,
            cursor_column: 1,
            total_lines: 1,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current file path
    pub fn set_current_file(&mut self, path: Option<PathBuf>) {
        self.current_file = path;
        self.is_dirty = false;
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
