//! Windows UI backend (Win32 API) for Notepad++ Rust Edition

// Windows-specific modules
pub mod main_window;
pub mod menu;
pub mod toolbar;
pub mod statusbar;
pub mod editor_control;
pub mod command_handler;
pub mod find_dialog;
pub mod replace_dialog;
pub mod goto_dialog;
pub mod accelerators;
pub mod file_dialogs;
pub mod window_updates;
pub mod global_state;

// Re-export portable modules from ui-core
pub use notepad_ui_core::encoding;
pub use notepad_ui_core::recent_files;
pub use notepad_ui_core::app_state;

// Re-export portable types
pub use notepad_ui_core::encoding::{Encoding, LineEnding};
pub use notepad_ui_core::recent_files::RecentFiles;
pub use notepad_ui_core::app_state::AppState;

// Re-export Windows-specific components
pub use main_window::MainWindow;
pub use accelerators::create_accelerators;
pub use global_state::{init_global_state, get_global_state, with_state, read_state};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("Window creation failed")]
    WindowCreationFailed,

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, UiError>;
