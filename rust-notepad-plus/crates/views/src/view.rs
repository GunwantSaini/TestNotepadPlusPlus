//! Editor view definition

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Unique identifier for a view
pub type ViewId = uuid::Uuid;

/// Represents a single editor view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    /// Unique view identifier
    pub id: ViewId,

    /// File path being viewed (if any)
    pub file_path: Option<PathBuf>,

    /// View title
    pub title: String,

    /// Scroll position (line number)
    pub scroll_line: usize,

    /// Scroll position (column offset)
    pub scroll_column: usize,

    /// Cursor position (line, column)
    pub cursor_position: (usize, usize),

    /// Selection range (start, end) if any
    pub selection: Option<((usize, usize), (usize, usize))>,

    /// Whether this view is read-only
    pub read_only: bool,

    /// Whether this view is currently focused
    pub focused: bool,

    /// View width in characters
    pub width: usize,

    /// View height in lines
    pub height: usize,

    /// Whether word wrap is enabled
    pub word_wrap: bool,

    /// Whether line numbers are shown
    pub show_line_numbers: bool,

    /// Tab size for this view
    pub tab_size: usize,

    /// Whether to use spaces instead of tabs
    pub use_spaces: bool,
}

impl View {
    /// Create a new view
    pub fn new() -> Self {
        Self {
            id: ViewId::new_v4(),
            file_path: None,
            title: "Untitled".to_string(),
            scroll_line: 0,
            scroll_column: 0,
            cursor_position: (0, 0),
            selection: None,
            read_only: false,
            focused: false,
            width: 80,
            height: 24,
            word_wrap: false,
            show_line_numbers: true,
            tab_size: 4,
            use_spaces: true,
        }
    }

    /// Create a new view with a file
    pub fn with_file(path: PathBuf) -> Self {
        let title = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        Self {
            file_path: Some(path),
            title,
            ..Self::new()
        }
    }

    /// Set the cursor position
    pub fn set_cursor(&mut self, line: usize, column: usize) {
        self.cursor_position = (line, column);
    }

    /// Get the cursor position
    pub fn cursor(&self) -> (usize, usize) {
        self.cursor_position
    }

    /// Set the scroll position
    pub fn set_scroll(&mut self, line: usize, column: usize) {
        self.scroll_line = line;
        self.scroll_column = column;
    }

    /// Get the scroll position
    pub fn scroll(&self) -> (usize, usize) {
        (self.scroll_line, self.scroll_column)
    }

    /// Set the selection
    pub fn set_selection(&mut self, start: (usize, usize), end: (usize, usize)) {
        self.selection = Some((start, end));
    }

    /// Clear the selection
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// Check if there is a selection
    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    /// Get the selection range
    pub fn get_selection(&self) -> Option<((usize, usize), (usize, usize))> {
        self.selection
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Check if view is focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Set read-only state
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Check if view is read-only
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Set view dimensions
    pub fn set_dimensions(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    /// Get view dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Set word wrap
    pub fn set_word_wrap(&mut self, enabled: bool) {
        self.word_wrap = enabled;
    }

    /// Check if word wrap is enabled
    pub fn is_word_wrap_enabled(&self) -> bool {
        self.word_wrap
    }

    /// Set line numbers visibility
    pub fn set_show_line_numbers(&mut self, show: bool) {
        self.show_line_numbers = show;
    }

    /// Check if line numbers are shown
    pub fn are_line_numbers_shown(&self) -> bool {
        self.show_line_numbers
    }

    /// Set tab size
    pub fn set_tab_size(&mut self, size: usize) {
        self.tab_size = size.max(1).min(16);
    }

    /// Get tab size
    pub fn tab_size(&self) -> usize {
        self.tab_size
    }

    /// Set whether to use spaces for tabs
    pub fn set_use_spaces(&mut self, use_spaces: bool) {
        self.use_spaces = use_spaces;
    }

    /// Check if using spaces for tabs
    pub fn is_using_spaces(&self) -> bool {
        self.use_spaces
    }

    /// Get the file path
    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    /// Set the file path
    pub fn set_file_path(&mut self, path: Option<PathBuf>) {
        if let Some(ref p) = path {
            self.title = p
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled")
                .to_string();
        }
        self.file_path = path;
    }

    /// Get the title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Clone view state (new ID)
    pub fn clone_state(&self) -> Self {
        Self {
            id: ViewId::new_v4(),
            ..self.clone()
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_view_creation() {
        let view = View::new();
        assert_eq!(view.title, "Untitled");
        assert_eq!(view.cursor_position, (0, 0));
        assert_eq!(view.scroll_line, 0);
        assert!(!view.focused);
        assert!(!view.read_only);
    }

    #[test]
    fn test_view_with_file() {
        let path = PathBuf::from("/path/to/test.txt");
        let view = View::with_file(path.clone());
        assert_eq!(view.title, "test.txt");
        assert_eq!(view.file_path, Some(path));
    }

    #[test]
    fn test_cursor_operations() {
        let mut view = View::new();

        view.set_cursor(10, 5);
        assert_eq!(view.cursor(), (10, 5));
        assert_eq!(view.cursor_position, (10, 5));
    }

    #[test]
    fn test_scroll_operations() {
        let mut view = View::new();

        view.set_scroll(20, 10);
        assert_eq!(view.scroll(), (20, 10));
    }

    #[test]
    fn test_selection_operations() {
        let mut view = View::new();

        assert!(!view.has_selection());
        assert_eq!(view.get_selection(), None);

        view.set_selection((0, 0), (5, 10));
        assert!(view.has_selection());
        assert_eq!(view.get_selection(), Some(((0, 0), (5, 10))));

        view.clear_selection();
        assert!(!view.has_selection());
    }

    #[test]
    fn test_focus_operations() {
        let mut view = View::new();

        assert!(!view.is_focused());

        view.set_focused(true);
        assert!(view.is_focused());

        view.set_focused(false);
        assert!(!view.is_focused());
    }

    #[test]
    fn test_read_only_operations() {
        let mut view = View::new();

        assert!(!view.is_read_only());

        view.set_read_only(true);
        assert!(view.is_read_only());
    }

    #[test]
    fn test_dimensions() {
        let mut view = View::new();

        assert_eq!(view.dimensions(), (80, 24));

        view.set_dimensions(120, 40);
        assert_eq!(view.dimensions(), (120, 40));
    }

    #[test]
    fn test_word_wrap() {
        let mut view = View::new();

        assert!(!view.is_word_wrap_enabled());

        view.set_word_wrap(true);
        assert!(view.is_word_wrap_enabled());
    }

    #[test]
    fn test_line_numbers() {
        let mut view = View::new();

        assert!(view.are_line_numbers_shown());

        view.set_show_line_numbers(false);
        assert!(!view.are_line_numbers_shown());
    }

    #[test]
    fn test_tab_settings() {
        let mut view = View::new();

        assert_eq!(view.tab_size(), 4);
        assert!(view.is_using_spaces());

        view.set_tab_size(8);
        assert_eq!(view.tab_size(), 8);

        view.set_use_spaces(false);
        assert!(!view.is_using_spaces());

        // Test bounds
        view.set_tab_size(0);
        assert_eq!(view.tab_size(), 1); // Minimum

        view.set_tab_size(100);
        assert_eq!(view.tab_size(), 16); // Maximum
    }

    #[test]
    fn test_file_path_operations() {
        let mut view = View::new();

        assert_eq!(view.file_path(), None);

        let path = PathBuf::from("/test/file.txt");
        view.set_file_path(Some(path.clone()));
        assert_eq!(view.file_path(), Some(&path));
        assert_eq!(view.title(), "file.txt");

        view.set_file_path(None);
        assert_eq!(view.file_path(), None);
    }

    #[test]
    fn test_title_operations() {
        let mut view = View::new();

        assert_eq!(view.title(), "Untitled");

        view.set_title("My File".to_string());
        assert_eq!(view.title(), "My File");
    }

    #[test]
    fn test_clone_state() {
        let mut view = View::new();
        view.set_cursor(10, 5);
        view.set_focused(true);

        let cloned = view.clone_state();

        // New ID
        assert_ne!(view.id, cloned.id);

        // Same state
        assert_eq!(view.cursor_position, cloned.cursor_position);
        assert_eq!(view.focused, cloned.focused);
    }

    #[test]
    fn test_unique_ids() {
        let view1 = View::new();
        let view2 = View::new();

        assert_ne!(view1.id, view2.id);
    }
}
