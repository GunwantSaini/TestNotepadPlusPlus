//! GTK menu implementation

use gtk4::prelude::*;
use gtk4::{gio, Menu, MenuBar, MenuItem};
use log::debug;
use std::path::PathBuf;

/// GTK menu builder
pub struct GtkMenu {
    menubar: MenuBar,
}

impl GtkMenu {
    /// Create a new GTK menu builder
    pub fn new() -> Self {
        let menubar = MenuBar::new();
        Self { menubar }
    }

    /// Get the GTK MenuBar widget
    pub fn widget(&self) -> &MenuBar {
        &self.menubar
    }

    /// Build the complete menu bar
    pub fn build_menu(&self) {
        // File menu
        let file_menu = gio::Menu::new();
        file_menu.append(Some("New"), Some("app.new"));
        file_menu.append(Some("Open..."), Some("app.open"));
        file_menu.append(Some("Save"), Some("app.save"));
        file_menu.append(Some("Save As..."), Some("app.save-as"));

        let recent_section = gio::Menu::new();
        file_menu.append_section(Some("Recent Files"), &recent_section);

        file_menu.append(Some("Exit"), Some("app.quit"));

        // Edit menu
        let edit_menu = gio::Menu::new();
        edit_menu.append(Some("Undo"), Some("app.undo"));
        edit_menu.append(Some("Redo"), Some("app.redo"));
        edit_menu.append(Some("Cut"), Some("app.cut"));
        edit_menu.append(Some("Copy"), Some("app.copy"));
        edit_menu.append(Some("Paste"), Some("app.paste"));
        edit_menu.append(Some("Select All"), Some("app.select-all"));

        // Search menu
        let search_menu = gio::Menu::new();
        search_menu.append(Some("Find..."), Some("app.find"));
        search_menu.append(Some("Replace..."), Some("app.replace"));
        search_menu.append(Some("Go To Line..."), Some("app.goto"));

        // View menu
        let view_menu = gio::Menu::new();
        view_menu.append(Some("Word Wrap"), Some("app.word-wrap"));

        // Encoding menu
        let encoding_menu = gio::Menu::new();
        encoding_menu.append(Some("UTF-8"), Some("app.encoding-utf8"));
        encoding_menu.append(Some("UTF-8 BOM"), Some("app.encoding-utf8-bom"));
        encoding_menu.append(Some("UTF-16 LE"), Some("app.encoding-utf16-le"));
        encoding_menu.append(Some("UTF-16 BE"), Some("app.encoding-utf16-be"));
        encoding_menu.append(Some("ANSI"), Some("app.encoding-ansi"));

        // Line endings menu
        let line_ending_menu = gio::Menu::new();
        line_ending_menu.append(Some("Windows (CRLF)"), Some("app.line-ending-crlf"));
        line_ending_menu.append(Some("Unix (LF)"), Some("app.line-ending-lf"));
        line_ending_menu.append(Some("Mac (CR)"), Some("app.line-ending-cr"));

        // Help menu
        let help_menu = gio::Menu::new();
        help_menu.append(Some("About"), Some("app.about"));

        // Add to menubar
        let main_menu = gio::Menu::new();
        main_menu.append_submenu(Some("File"), &file_menu);
        main_menu.append_submenu(Some("Edit"), &edit_menu);
        main_menu.append_submenu(Some("Search"), &search_menu);
        main_menu.append_submenu(Some("View"), &view_menu);
        main_menu.append_submenu(Some("Encoding"), &encoding_menu);
        main_menu.append_submenu(Some("Line Endings"), &line_ending_menu);
        main_menu.append_submenu(Some("Help"), &help_menu);

        self.menubar.set_menu_model(Some(&main_menu));

        debug!("GTK menu built");
    }

    /// Update recent files list
    pub fn update_recent_files(&self, _files: &[PathBuf]) {
        // GTK menus are updated through actions in GTK4
        debug!("Recent files update requested");
    }
}

impl Default for GtkMenu {
    fn default() -> Self {
        Self::new()
    }
}
