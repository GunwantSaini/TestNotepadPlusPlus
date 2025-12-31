//! GTK dialog implementations (file dialogs, message boxes, etc.)

use gtk4::prelude::*;
use gtk4::{
    FileChooserAction, FileChooserDialog, FileFilter, MessageDialog, MessageType as GtkMessageType,
    ButtonsType, ResponseType, Window, Entry, Dialog, Box as GtkBox, Orientation, Label,
};
use std::path::PathBuf;
use log::debug;

/// GTK dialog provider
pub struct GtkDialogs {
    parent_window: Window,
}

impl GtkDialogs {
    /// Create a new dialog provider with a parent window
    pub fn new(parent_window: Window) -> Self {
        Self { parent_window }
    }

    /// Show an "Open File" dialog
    pub fn show_open_dialog(&self, filters: &[(String, Vec<String>)]) -> Option<PathBuf> {
        let dialog = FileChooserDialog::new(
            Some("Open File"),
            Some(&self.parent_window),
            FileChooserAction::Open,
            &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
        );

        // Add file filters
        for (name, patterns) in filters {
            let filter = FileFilter::new();
            filter.set_name(Some(name));
            for pattern in patterns {
                filter.add_pattern(pattern);
            }
            dialog.add_filter(&filter);
        }

        let response = dialog.run_future();
        let result = glib::MainContext::default().block_on(async {
            let resp = response.await;
            if resp == ResponseType::Accept {
                dialog.file().and_then(|f| f.path())
            } else {
                None
            }
        });

        dialog.close();
        debug!("Open dialog result: {:?}", result);
        result
    }

    /// Show a "Save File" dialog
    pub fn show_save_dialog(&self, default_name: &str, filters: &[(String, Vec<String>)]) -> Option<PathBuf> {
        let dialog = FileChooserDialog::new(
            Some("Save File"),
            Some(&self.parent_window),
            FileChooserAction::Save,
            &[("Cancel", ResponseType::Cancel), ("Save", ResponseType::Accept)],
        );

        dialog.set_current_name(default_name);

        // Add file filters
        for (name, patterns) in filters {
            let filter = FileFilter::new();
            filter.set_name(Some(name));
            for pattern in patterns {
                filter.add_pattern(pattern);
            }
            dialog.add_filter(&filter);
        }

        let response = dialog.run_future();
        let result = glib::MainContext::default().block_on(async {
            let resp = response.await;
            if resp == ResponseType::Accept {
                dialog.file().and_then(|f| f.path())
            } else {
                None
            }
        });

        dialog.close();
        debug!("Save dialog result: {:?}", result);
        result
    }

    /// Show an informational message
    pub fn show_message(&self, title: &str, message: &str, msg_type: MessageTypeWrapper) {
        let gtk_type = match msg_type {
            MessageTypeWrapper::Info => GtkMessageType::Info,
            MessageTypeWrapper::Warning => GtkMessageType::Warning,
            MessageTypeWrapper::Error => GtkMessageType::Error,
            MessageTypeWrapper::Question => GtkMessageType::Question,
        };

        let dialog = MessageDialog::new(
            Some(&self.parent_window),
            gtk4::DialogFlags::MODAL,
            gtk_type,
            ButtonsType::Ok,
            message,
        );

        dialog.set_title(Some(title));

        let response = dialog.run_future();
        glib::MainContext::default().block_on(async {
            response.await;
        });

        dialog.close();
    }

    /// Show a yes/no question dialog
    pub fn show_question(&self, title: &str, message: &str) -> DialogResultWrapper {
        let dialog = MessageDialog::new(
            Some(&self.parent_window),
            gtk4::DialogFlags::MODAL,
            GtkMessageType::Question,
            ButtonsType::YesNo,
            message,
        );

        dialog.set_title(Some(title));

        let response = dialog.run_future();
        let result = glib::MainContext::default().block_on(async {
            let resp = response.await;
            match resp {
                ResponseType::Yes => DialogResultWrapper::Yes,
                ResponseType::No => DialogResultWrapper::No,
                _ => DialogResultWrapper::Cancel,
            }
        });

        dialog.close();
        result
    }

    /// Show "Find" dialog
    pub fn show_find_dialog(&self) -> Option<FindDialogResult> {
        let dialog = Dialog::with_buttons(
            Some("Find"),
            Some(&self.parent_window),
            gtk4::DialogFlags::MODAL,
            &[("Cancel", ResponseType::Cancel), ("Find Next", ResponseType::Accept)],
        );

        let content = dialog.content_area();
        let vbox = GtkBox::new(Orientation::Vertical, 10);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);
        vbox.set_margin_top(10);
        vbox.set_margin_bottom(10);

        let label = Label::new(Some("Find what:"));
        vbox.append(&label);

        let entry = Entry::new();
        entry.set_activates_default(true);
        vbox.append(&entry);

        content.append(&vbox);

        dialog.set_default_response(ResponseType::Accept);

        let response = dialog.run_future();
        let entry_clone = entry.clone();
        let result = glib::MainContext::default().block_on(async {
            let resp = response.await;
            if resp == ResponseType::Accept {
                let text = entry_clone.text().to_string();
                if !text.is_empty() {
                    Some(FindDialogResult {
                        search_text: text,
                        case_sensitive: false,
                        whole_word: false,
                        use_regex: false,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        });

        dialog.close();
        result
    }

    /// Show "Replace" dialog
    pub fn show_replace_dialog(&self) -> Option<ReplaceDialogResult> {
        let dialog = Dialog::with_buttons(
            Some("Replace"),
            Some(&self.parent_window),
            gtk4::DialogFlags::MODAL,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Replace", ResponseType::Accept),
                ("Replace All", ResponseType::Apply),
            ],
        );

        let content = dialog.content_area();
        let vbox = GtkBox::new(Orientation::Vertical, 10);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);
        vbox.set_margin_top(10);
        vbox.set_margin_bottom(10);

        let find_label = Label::new(Some("Find what:"));
        vbox.append(&find_label);

        let find_entry = Entry::new();
        vbox.append(&find_entry);

        let replace_label = Label::new(Some("Replace with:"));
        vbox.append(&replace_label);

        let replace_entry = Entry::new();
        replace_entry.set_activates_default(true);
        vbox.append(&replace_entry);

        content.append(&vbox);

        dialog.set_default_response(ResponseType::Accept);

        let response = dialog.run_future();
        let find_clone = find_entry.clone();
        let replace_clone = replace_entry.clone();
        let result = glib::MainContext::default().block_on(async {
            let resp = response.await;
            if resp == ResponseType::Accept || resp == ResponseType::Apply {
                let find_text = find_clone.text().to_string();
                let replace_text = replace_clone.text().to_string();
                if !find_text.is_empty() {
                    Some(ReplaceDialogResult {
                        search_text: find_text,
                        replace_text,
                        case_sensitive: false,
                        whole_word: false,
                        use_regex: false,
                        replace_all: resp == ResponseType::Apply,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        });

        dialog.close();
        result
    }

    /// Show "Go To Line" dialog
    pub fn show_goto_dialog(&self, max_line: usize) -> Option<usize> {
        let dialog = Dialog::with_buttons(
            Some("Go To Line"),
            Some(&self.parent_window),
            gtk4::DialogFlags::MODAL,
            &[("Cancel", ResponseType::Cancel), ("Go", ResponseType::Accept)],
        );

        let content = dialog.content_area();
        let vbox = GtkBox::new(Orientation::Vertical, 10);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);
        vbox.set_margin_top(10);
        vbox.set_margin_bottom(10);

        let label = Label::new(Some(&format!("Line number (1-{}):", max_line)));
        vbox.append(&label);

        let entry = Entry::new();
        entry.set_activates_default(true);
        vbox.append(&entry);

        content.append(&vbox);

        dialog.set_default_response(ResponseType::Accept);

        let response = dialog.run_future();
        let entry_clone = entry.clone();
        let result = glib::MainContext::default().block_on(async {
            let resp = response.await;
            if resp == ResponseType::Accept {
                let text = entry_clone.text().to_string();
                text.trim().parse::<usize>().ok().filter(|&line| line > 0 && line <= max_line)
            } else {
                None
            }
        });

        dialog.close();
        result
    }
}

/// Wrapper types for platform-independent dialog results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageTypeWrapper {
    Info,
    Warning,
    Error,
    Question,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogResultWrapper {
    Yes,
    No,
    Cancel,
}

#[derive(Debug, Clone)]
pub struct FindDialogResult {
    pub search_text: String,
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub use_regex: bool,
}

#[derive(Debug, Clone)]
pub struct ReplaceDialogResult {
    pub search_text: String,
    pub replace_text: String,
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub use_regex: bool,
    pub replace_all: bool,
}
