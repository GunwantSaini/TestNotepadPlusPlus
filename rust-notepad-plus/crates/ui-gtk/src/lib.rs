//! GTK4 UI backend for Notepad++ Rust Edition (Linux)

pub mod gtk_text_editor;
pub mod gtk_dialogs;
pub mod gtk_main_window;
pub mod gtk_menu;
pub mod gtk_statusbar;
pub mod app_state_manager;

// Re-export portable modules from ui-core
pub use notepad_ui_core::encoding;
pub use notepad_ui_core::recent_files;
pub use notepad_ui_core::app_state;

// Re-export portable types
pub use notepad_ui_core::encoding::{Encoding, LineEnding};
pub use notepad_ui_core::recent_files::RecentFiles;
pub use notepad_ui_core::app_state::AppState;

// Re-export GTK-specific components
pub use gtk_main_window::GtkMainWindow;
pub use app_state_manager::{init_global_state, with_state, read_state};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("Window creation failed")]
    WindowCreationFailed,

    #[error("GTK initialization failed: {0}")]
    GtkInitFailed(String),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, UiError>;
