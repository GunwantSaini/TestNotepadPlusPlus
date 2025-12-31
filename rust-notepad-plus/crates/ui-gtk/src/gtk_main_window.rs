//! GTK main window implementation

use crate::gtk_dialogs::{DialogResultWrapper, GtkDialogs, MessageTypeWrapper};
use crate::gtk_menu::GtkMenu;
use crate::gtk_statusbar::GtkStatusBar;
use crate::gtk_text_editor::GtkTextEditor;
use crate::{app_state_manager, encoding, AppState, Encoding, LineEnding};
use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box as GtkBox, Orientation, ScrolledWindow,
};
use log::{debug, info};
use notepad_core::NotepadApp;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// GTK-based main window
pub struct GtkMainWindow {
    app: Application,
    window: ApplicationWindow,
    editor: Rc<RefCell<GtkTextEditor>>,
    dialogs: Rc<GtkDialogs>,
    menu: Rc<GtkMenu>,
    statusbar: Rc<GtkStatusBar>,
}

impl GtkMainWindow {
    /// Create a new GTK main window
    pub fn new(_notepad_app: &NotepadApp) -> Result<Self, crate::UiError> {
        // Create GTK application
        let app = Application::builder()
            .application_id("com.notepadplusplus.rust")
            .build();

        // Build UI on activation
        let window = ApplicationWindow::builder()
            .application(&app)
            .title("Notepad++ (Rust Edition)")
            .default_width(800)
            .default_height(600)
            .build();

        // Create main vertical box
        let vbox = GtkBox::new(Orientation::Vertical, 0);

        // Create menu
        let menu = Rc::new(GtkMenu::new());
        menu.build_menu();
        vbox.append(menu.widget());

        // Create text editor with scrolling
        let editor = Rc::new(RefCell::new(GtkTextEditor::new()));
        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(editor.borrow().widget()));
        scrolled.set_vexpand(true);
        vbox.append(&scrolled);

        // Create status bar
        let statusbar = Rc::new(GtkStatusBar::new());
        vbox.append(statusbar.widget());

        // Set the main box as window content
        window.set_child(Some(&vbox));

        // Create dialogs provider
        let dialogs = Rc::new(GtkDialogs::new(window.clone().upcast()));

        // Set up actions
        let main_window = Self {
            app: app.clone(),
            window: window.clone(),
            editor: editor.clone(),
            dialogs: dialogs.clone(),
            menu: menu.clone(),
            statusbar: statusbar.clone(),
        };

        main_window.setup_actions();
        main_window.setup_text_change_handler();

        info!("GTK main window created");
        Ok(main_window)
    }

    /// Set up GTK actions for menu items
    fn setup_actions(&self) {
        let editor = self.editor.clone();
        let dialogs = self.dialogs.clone();
        let statusbar = self.statusbar.clone();
        let window = self.window.clone();

        // File -> New
        let new_action = gio::SimpleAction::new("new", None);
        let editor_clone = editor.clone();
        let dialogs_clone = dialogs.clone();
        new_action.connect_activate(move |_, _| {
            if app_state_manager::read_state(|state| state.is_dirty) {
                let result = dialogs_clone.show_question(
                    "Unsaved Changes",
                    "Do you want to save changes?",
                );
                if result == DialogResultWrapper::Yes {
                    // TODO: Save file
                }
            }
            editor_clone.borrow().set_text("");
            app_state_manager::with_state(|state| {
                state.current_file = None;
                state.is_dirty = false;
            });
            debug!("New file created");
        });
        self.app.add_action(&new_action);

        // File -> Open
        let open_action = gio::SimpleAction::new("open", None);
        let editor_clone = editor.clone();
        let dialogs_clone = dialogs.clone();
        let statusbar_clone = statusbar.clone();
        open_action.connect_activate(move |_, _| {
            let filters = vec![
                ("Text Files".to_string(), vec!["*.txt".to_string()]),
                ("All Files".to_string(), vec!["*".to_string()]),
            ];
            if let Some(path) = dialogs_clone.show_open_dialog(&filters) {
                match Self::load_file(&path, &editor_clone, &statusbar_clone) {
                    Ok(_) => {
                        app_state_manager::with_state(|state| {
                            state.current_file = Some(path.clone());
                            state.is_dirty = false;
                            state.recent_files.add(path);
                        });
                        info!("File opened successfully");
                    }
                    Err(e) => {
                        dialogs_clone.show_message(
                            "Error",
                            &format!("Failed to open file: {}", e),
                            MessageTypeWrapper::Error,
                        );
                    }
                }
            }
        });
        self.app.add_action(&open_action);

        // File -> Save
        let save_action = gio::SimpleAction::new("save", None);
        let editor_clone = editor.clone();
        let dialogs_clone = dialogs.clone();
        save_action.connect_activate(move |_, _| {
            let current_file = app_state_manager::read_state(|state| state.current_file.clone());
            if let Some(path) = current_file {
                match Self::save_file(&path, &editor_clone) {
                    Ok(_) => {
                        app_state_manager::with_state(|state| state.is_dirty = false);
                        info!("File saved successfully");
                    }
                    Err(e) => {
                        dialogs_clone.show_message(
                            "Error",
                            &format!("Failed to save file: {}", e),
                            MessageTypeWrapper::Error,
                        );
                    }
                }
            } else {
                // No current file, show save dialog
                let filters = vec![
                    ("Text Files".to_string(), vec!["*.txt".to_string()]),
                    ("All Files".to_string(), vec!["*".to_string()]),
                ];
                if let Some(path) = dialogs_clone.show_save_dialog("untitled.txt", &filters) {
                    match Self::save_file(&path, &editor_clone) {
                        Ok(_) => {
                            app_state_manager::with_state(|state| {
                                state.current_file = Some(path);
                                state.is_dirty = false;
                            });
                            info!("File saved successfully");
                        }
                        Err(e) => {
                            dialogs_clone.show_message(
                                "Error",
                                &format!("Failed to save file: {}", e),
                                MessageTypeWrapper::Error,
                            );
                        }
                    }
                }
            }
        });
        self.app.add_action(&save_action);

        // File -> Save As
        let save_as_action = gio::SimpleAction::new("save-as", None);
        let editor_clone = editor.clone();
        let dialogs_clone = dialogs.clone();
        save_as_action.connect_activate(move |_, _| {
            let filters = vec![
                ("Text Files".to_string(), vec!["*.txt".to_string()]),
                ("All Files".to_string(), vec!["*".to_string()]),
            ];
            if let Some(path) = dialogs_clone.show_save_dialog("untitled.txt", &filters) {
                match Self::save_file(&path, &editor_clone) {
                    Ok(_) => {
                        app_state_manager::with_state(|state| {
                            state.current_file = Some(path);
                            state.is_dirty = false;
                        });
                        info!("File saved as new file");
                    }
                    Err(e) => {
                        dialogs_clone.show_message(
                            "Error",
                            &format!("Failed to save file: {}", e),
                            MessageTypeWrapper::Error,
                        );
                    }
                }
            }
        });
        self.app.add_action(&save_as_action);

        // File -> Quit
        let quit_action = gio::SimpleAction::new("quit", None);
        let app_clone = self.app.clone();
        quit_action.connect_activate(move |_, _| {
            app_clone.quit();
        });
        self.app.add_action(&quit_action);

        // Edit -> Undo
        let undo_action = gio::SimpleAction::new("undo", None);
        let editor_clone = editor.clone();
        undo_action.connect_activate(move |_, _| {
            editor_clone.borrow().undo();
        });
        self.app.add_action(&undo_action);

        // Edit -> Redo
        let redo_action = gio::SimpleAction::new("redo", None);
        let editor_clone = editor.clone();
        redo_action.connect_activate(move |_, _| {
            editor_clone.borrow().redo();
        });
        self.app.add_action(&redo_action);

        // Edit -> Cut
        let cut_action = gio::SimpleAction::new("cut", None);
        let editor_clone = editor.clone();
        cut_action.connect_activate(move |_, _| {
            editor_clone.borrow().cut();
        });
        self.app.add_action(&cut_action);

        // Edit -> Copy
        let copy_action = gio::SimpleAction::new("copy", None);
        let editor_clone = editor.clone();
        copy_action.connect_activate(move |_, _| {
            editor_clone.borrow().copy();
        });
        self.app.add_action(&copy_action);

        // Edit -> Paste
        let paste_action = gio::SimpleAction::new("paste", None);
        let editor_clone = editor.clone();
        paste_action.connect_activate(move |_, _| {
            editor_clone.borrow().paste();
        });
        self.app.add_action(&paste_action);

        // Edit -> Select All
        let select_all_action = gio::SimpleAction::new("select-all", None);
        let editor_clone = editor.clone();
        select_all_action.connect_activate(move |_, _| {
            editor_clone.borrow().select_all();
        });
        self.app.add_action(&select_all_action);

        // Search -> Find
        let find_action = gio::SimpleAction::new("find", None);
        let dialogs_clone = dialogs.clone();
        find_action.connect_activate(move |_, _| {
            if let Some(result) = dialogs_clone.show_find_dialog() {
                debug!("Find: {}", result.search_text);
                // TODO: Implement search
            }
        });
        self.app.add_action(&find_action);

        // Search -> Replace
        let replace_action = gio::SimpleAction::new("replace", None);
        let dialogs_clone = dialogs.clone();
        replace_action.connect_activate(move |_, _| {
            if let Some(result) = dialogs_clone.show_replace_dialog() {
                debug!("Replace: {} -> {}", result.search_text, result.replace_text);
                // TODO: Implement replace
            }
        });
        self.app.add_action(&replace_action);

        // Search -> Go To Line
        let goto_action = gio::SimpleAction::new("goto", None);
        let dialogs_clone = dialogs.clone();
        let editor_clone = editor.clone();
        goto_action.connect_activate(move |_, _| {
            let line_count = editor_clone.borrow().get_line_count();
            if let Some(line) = dialogs_clone.show_goto_dialog(line_count) {
                debug!("Go to line: {}", line);
                // TODO: Implement go to line
            }
        });
        self.app.add_action(&goto_action);

        // View -> Word Wrap
        let word_wrap_action = gio::SimpleAction::new("word-wrap", None);
        let editor_clone = editor.clone();
        word_wrap_action.connect_activate(move |_, _| {
            let new_state = !app_state_manager::read_state(|state| state.word_wrap_enabled);
            editor_clone.borrow().set_word_wrap(new_state);
            app_state_manager::with_state(|state| state.word_wrap_enabled = new_state);
            debug!("Word wrap toggled: {}", new_state);
        });
        self.app.add_action(&word_wrap_action);

        // Help -> About
        let about_action = gio::SimpleAction::new("about", None);
        let dialogs_clone = dialogs.clone();
        about_action.connect_activate(move |_, _| {
            dialogs_clone.show_message(
                "About Notepad++",
                "Notepad++ Rust Edition\nVersion 8.0.0\n\nCross-platform text editor",
                MessageTypeWrapper::Info,
            );
        });
        self.app.add_action(&about_action);

        info!("GTK actions configured");
    }

    /// Set up text change handler to track modifications
    fn setup_text_change_handler(&self) {
        let editor = self.editor.borrow();
        let statusbar = self.statusbar.clone();

        // Connect to buffer changed signal
        let buffer = editor.widget().buffer();
        buffer.connect_changed(move |buf| {
            // Update modified state
            if buf.is_modified() {
                app_state_manager::with_state(|state| state.is_dirty = true);
            }

            // Update cursor position in status bar
            let cursor_pos = buf.cursor_position();
            let iter = buf.iter_at_offset(cursor_pos);
            let line = iter.line() as usize;
            let column = iter.line_offset() as usize;

            let (encoding, line_ending) = app_state_manager::read_state(|state| {
                (state.current_encoding.to_string(), state.current_line_ending.to_string())
            });

            statusbar.update_all(line, column, &encoding, &line_ending);
        });
    }

    /// Load a file into the editor
    fn load_file(
        path: &PathBuf,
        editor: &Rc<RefCell<GtkTextEditor>>,
        statusbar: &Rc<GtkStatusBar>,
    ) -> anyhow::Result<()> {
        let content = std::fs::read(path)?;
        let (text, encoding, line_ending) = encoding::detect_encoding_and_convert(&content)?;

        editor.borrow().set_text(&text);
        editor.borrow().set_modified(false);

        app_state_manager::with_state(|state| {
            state.current_encoding = encoding;
            state.current_line_ending = line_ending;
        });

        statusbar.set_encoding(&encoding.to_string());
        statusbar.set_line_ending(&line_ending.to_string());

        Ok(())
    }

    /// Save the editor content to a file
    fn save_file(path: &PathBuf, editor: &Rc<RefCell<GtkTextEditor>>) -> anyhow::Result<()> {
        let text = editor.borrow().get_text();
        let (encoding, line_ending) = app_state_manager::read_state(|state| {
            (state.current_encoding, state.current_line_ending)
        });

        let bytes = encoding::convert_to_encoding(&text, encoding, line_ending)?;
        std::fs::write(path, bytes)?;

        editor.borrow().set_modified(false);

        Ok(())
    }

    /// Show the window
    pub fn show(&self) {
        self.window.present();
    }

    /// Run the GTK application event loop
    pub fn run(&self) -> i32 {
        self.app.run()
    }
}
