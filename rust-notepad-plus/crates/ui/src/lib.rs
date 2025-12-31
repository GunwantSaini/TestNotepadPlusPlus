//! User interface components

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
pub mod app_state;
pub mod window_updates;
pub mod global_state;
pub mod recent_files;
pub mod encoding;

pub use main_window::MainWindow;
pub use accelerators::create_accelerators;
pub use app_state::{AppState, SharedAppState, create_shared_state};
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
