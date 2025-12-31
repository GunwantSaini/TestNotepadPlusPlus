//! GTK status bar implementation

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Separator};
use log::debug;

/// GTK status bar
pub struct GtkStatusBar {
    container: GtkBox,
    line_label: Label,
    column_label: Label,
    encoding_label: Label,
    line_ending_label: Label,
}

impl GtkStatusBar {
    /// Create a new GTK status bar
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 5);
        container.set_margin_start(5);
        container.set_margin_end(5);
        container.set_margin_top(2);
        container.set_margin_bottom(2);

        // Line and column indicator
        let line_label = Label::new(Some("Ln: 1"));
        line_label.set_width_chars(10);
        container.append(&line_label);

        let sep1 = Separator::new(Orientation::Vertical);
        container.append(&sep1);

        let column_label = Label::new(Some("Col: 1"));
        column_label.set_width_chars(10);
        container.append(&column_label);

        let sep2 = Separator::new(Orientation::Vertical);
        container.append(&sep2);

        // Encoding indicator
        let encoding_label = Label::new(Some("UTF-8"));
        encoding_label.set_width_chars(15);
        container.append(&encoding_label);

        let sep3 = Separator::new(Orientation::Vertical);
        container.append(&sep3);

        // Line ending indicator
        let line_ending_label = Label::new(Some("CRLF"));
        line_ending_label.set_width_chars(8);
        container.append(&line_ending_label);

        Self {
            container,
            line_label,
            column_label,
            encoding_label,
            line_ending_label,
        }
    }

    /// Get the GTK container widget
    pub fn widget(&self) -> &GtkBox {
        &self.container
    }

    /// Update cursor position
    pub fn set_cursor_position(&self, line: usize, column: usize) {
        self.line_label.set_text(&format!("Ln: {}", line + 1));
        self.column_label.set_text(&format!("Col: {}", column + 1));
        debug!("Cursor position: {}:{}", line + 1, column + 1);
    }

    /// Update encoding indicator
    pub fn set_encoding(&self, encoding: &str) {
        self.encoding_label.set_text(encoding);
        debug!("Encoding: {}", encoding);
    }

    /// Update line ending indicator
    pub fn set_line_ending(&self, line_ending: &str) {
        self.line_ending_label.set_text(line_ending);
        debug!("Line ending: {}", line_ending);
    }

    /// Update all status information
    pub fn update_all(&self, line: usize, column: usize, encoding: &str, line_ending: &str) {
        self.set_cursor_position(line, column);
        self.set_encoding(encoding);
        self.set_line_ending(line_ending);
    }
}

impl Default for GtkStatusBar {
    fn default() -> Self {
        Self::new()
    }
}
