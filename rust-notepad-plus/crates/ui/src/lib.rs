//! User interface components

pub mod main_window;
pub mod menu;
pub mod toolbar;
pub mod statusbar;
pub mod editor_control;
pub mod command_handler;
pub mod find_dialog;

pub use main_window::MainWindow;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("Window creation failed")]
    WindowCreationFailed,

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, UiError>;
