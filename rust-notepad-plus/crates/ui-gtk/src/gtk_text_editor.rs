//! GTK TextView-based text editor implementation

use gtk4::prelude::*;
use gtk4::{TextBuffer, TextView, WrapMode};
use log::debug;

/// GTK-based text editor widget
pub struct GtkTextEditor {
    view: TextView,
    buffer: TextBuffer,
}

impl GtkTextEditor {
    /// Create a new GTK text editor
    pub fn new() -> Self {
        let buffer = TextBuffer::new(None);
        let view = TextView::with_buffer(&buffer);

        // Configure the text view
        view.set_monospace(true);
        view.set_left_margin(5);
        view.set_right_margin(5);
        view.set_top_margin(5);
        view.set_bottom_margin(5);
        view.set_wrap_mode(WrapMode::None);
        view.set_show_line_numbers(true);

        Self { view, buffer }
    }

    /// Get the GTK TextView widget
    pub fn widget(&self) -> &TextView {
        &self.view
    }

    /// Set text content
    pub fn set_text(&self, text: &str) {
        self.buffer.set_text(text);
        debug!("Text set ({} bytes)", text.len());
    }

    /// Get text content
    pub fn get_text(&self) -> String {
        let start = self.buffer.start_iter();
        let end = self.buffer.end_iter();
        self.buffer.text(&start, &end, false).to_string()
    }

    /// Set selection range
    pub fn set_selection(&self, start: usize, end: usize) {
        let start_iter = self.buffer.iter_at_offset(start as i32);
        let end_iter = self.buffer.iter_at_offset(end as i32);
        self.buffer.select_range(&start_iter, &end_iter);
    }

    /// Get selection range
    pub fn get_selection(&self) -> (usize, usize) {
        let (start, end) = self.buffer.selection_bounds()
            .unwrap_or_else(|| {
                let cursor = self.buffer.cursor_position();
                let iter = self.buffer.iter_at_offset(cursor);
                (iter.clone(), iter)
            });
        (start.offset() as usize, end.offset() as usize)
    }

    /// Undo last operation
    pub fn undo(&self) -> bool {
        // GTK4 TextBuffer doesn't have built-in undo/redo
        // We'll need to implement this using GtkSourceView or custom implementation
        debug!("Undo not yet implemented for GTK backend");
        false
    }

    /// Redo last undone operation
    pub fn redo(&self) -> bool {
        debug!("Redo not yet implemented for GTK backend");
        false
    }

    /// Cut selected text to clipboard
    pub fn cut(&self) {
        let clipboard = self.view.clipboard();
        self.buffer.cut_clipboard(&clipboard, true);
    }

    /// Copy selected text to clipboard
    pub fn copy(&self) {
        let clipboard = self.view.clipboard();
        self.buffer.copy_clipboard(&clipboard);
    }

    /// Paste from clipboard
    pub fn paste(&self) {
        let clipboard = self.view.clipboard();
        self.buffer.paste_clipboard(&clipboard, None, true);
    }

    /// Enable or disable word wrap
    pub fn set_word_wrap(&self, enabled: bool) {
        let wrap_mode = if enabled {
            WrapMode::Word
        } else {
            WrapMode::None
        };
        self.view.set_wrap_mode(wrap_mode);
        debug!("Word wrap: {}", enabled);
    }

    /// Get cursor position (line, column)
    pub fn get_cursor_position(&self) -> (usize, usize) {
        let cursor = self.buffer.cursor_position();
        let iter = self.buffer.iter_at_offset(cursor);
        (iter.line() as usize, iter.line_offset() as usize)
    }

    /// Get total line count
    pub fn get_line_count(&self) -> usize {
        self.buffer.line_count() as usize
    }

    /// Select all text
    pub fn select_all(&self) {
        let start = self.buffer.start_iter();
        let end = self.buffer.end_iter();
        self.buffer.select_range(&start, &end);
    }

    /// Clear all text
    pub fn clear(&self) {
        self.buffer.set_text("");
    }

    /// Check if text has been modified
    pub fn is_modified(&self) -> bool {
        self.buffer.is_modified()
    }

    /// Set modified state
    pub fn set_modified(&self, modified: bool) {
        self.buffer.set_modified(modified);
    }
}

impl Default for GtkTextEditor {
    fn default() -> Self {
        Self::new()
    }
}
