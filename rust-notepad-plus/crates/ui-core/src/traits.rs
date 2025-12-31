//! Platform-independent UI trait definitions

use crate::types::*;
use notepad_core::CommandId;
use std::path::PathBuf;

/// Platform-independent text editor widget
pub trait TextEditor {
    /// Set the entire text content
    fn set_text(&mut self, text: &str);

    /// Get the entire text content
    fn get_text(&self) -> String;

    /// Set the current selection range
    fn set_selection(&mut self, start: usize, end: usize);

    /// Get the current selection range (start, end)
    fn get_selection(&self) -> (usize, usize);

    /// Get the current cursor position
    fn get_cursor_position(&self) -> usize;

    /// Undo the last operation
    fn undo(&mut self) -> bool;

    /// Redo the last undone operation
    fn redo(&mut self) -> bool;

    /// Cut selected text to clipboard
    fn cut(&mut self) -> bool;

    /// Copy selected text to clipboard
    fn copy(&mut self) -> bool;

    /// Paste text from clipboard
    fn paste(&mut self) -> bool;

    /// Delete selected text
    fn delete(&mut self) -> bool;

    /// Select all text
    fn select_all(&mut self);

    /// Enable or disable word wrap
    fn set_word_wrap(&mut self, enabled: bool);

    /// Get word wrap state
    fn get_word_wrap(&self) -> bool;

    /// Set read-only mode
    fn set_read_only(&mut self, read_only: bool);

    /// Check if the editor has been modified
    fn is_modified(&self) -> bool;

    /// Clear the modified flag
    fn clear_modified(&mut self);

    /// Go to a specific line number (1-based)
    fn go_to_line(&mut self, line: usize) -> bool;

    /// Get the total number of lines
    fn get_line_count(&self) -> usize;

    /// Get the current line and column (1-based)
    fn get_line_column(&self) -> (usize, usize);
}

/// File dialog provider
pub trait DialogProvider {
    /// Show file open dialog
    /// Returns the selected file path, or None if cancelled
    fn show_open_dialog(&self, filters: &[FileFilter]) -> Option<PathBuf>;

    /// Show file save dialog
    /// Returns the selected file path, or None if cancelled
    fn show_save_dialog(&self, default_name: &str, filters: &[FileFilter]) -> Option<PathBuf>;

    /// Show a simple message dialog
    fn show_message(&self, title: &str, message: &str, msg_type: MessageType);

    /// Show an error message dialog
    fn show_error(&self, title: &str, message: &str);

    /// Show a question dialog with Yes/No/Cancel buttons
    /// Returns the user's choice
    fn show_question(&self, title: &str, message: &str) -> DialogResult;

    /// Show the Find dialog
    fn show_find_dialog(&self) -> Option<FindDialogResult>;

    /// Show the Replace dialog
    fn show_replace_dialog(&self) -> Option<ReplaceDialogResult>;

    /// Show the Go To Line dialog
    fn show_goto_dialog(&self, max_line: usize) -> Option<usize>;

    /// Show the About dialog
    fn show_about_dialog(&self);
}

/// Menu builder and manager
pub trait MenuBuilder {
    /// Create the main menu bar
    fn create_menu_bar(&mut self, items: &[MenuGroup]) -> Result<(), UiError>;

    /// Update the recent files menu
    fn update_recent_files(&mut self, files: &[PathBuf]);

    /// Enable or disable a menu item
    fn set_menu_enabled(&mut self, id: CommandId, enabled: bool);

    /// Check or uncheck a menu item
    fn set_menu_checked(&mut self, id: CommandId, checked: bool);

    /// Get the menu item state
    fn get_menu_state(&self, id: CommandId) -> MenuItemState;
}

/// Status bar manager
pub trait StatusBar {
    /// Update the position section (e.g., "Ln 1, Col 1")
    fn set_position(&mut self, line: usize, column: usize);

    /// Update the encoding section (e.g., "UTF-8 | CRLF")
    fn set_encoding(&mut self, encoding: &str, line_ending: &str);

    /// Update the general message section
    fn set_message(&mut self, message: &str);

    /// Clear all sections
    fn clear(&mut self);
}

/// Main application window
pub trait MainWindow {
    /// Create a new main window
    fn new() -> Result<Self, UiError> where Self: Sized;

    /// Run the main event loop
    /// This blocks until the window is closed
    fn run(&mut self) -> Result<(), UiError>;

    /// Set the window title
    fn set_title(&mut self, title: &str);

    /// Get a mutable reference to the text editor
    fn get_editor(&mut self) -> &mut dyn TextEditor;

    /// Get a reference to the dialog provider
    fn get_dialogs(&self) -> &dyn DialogProvider;

    /// Get a mutable reference to the menu builder
    fn get_menu(&mut self) -> &mut dyn MenuBuilder;

    /// Get a mutable reference to the status bar
    fn get_status_bar(&mut self) -> &mut dyn StatusBar;

    /// Close the window
    fn close(&mut self);

    /// Request the window to close (can be cancelled by user)
    fn request_close(&mut self) -> bool;

    /// Handle a command
    fn handle_command(&mut self, cmd: CommandId) -> bool;
}

/// Trait for platform initialization
pub trait Platform {
    /// Initialize the platform backend
    fn init() -> Result<(), UiError>;

    /// Get the platform name (e.g., "Windows", "Linux", "macOS")
    fn name() -> &'static str;

    /// Check if the platform is supported
    fn is_supported() -> bool;
}
