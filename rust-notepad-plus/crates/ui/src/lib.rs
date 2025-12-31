//! User interface components

pub mod main_window;
pub mod menu;
pub mod toolbar;
pub mod statusbar;

pub use main_window::MainWindow;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("Window creation failed")]
    WindowCreationFailed,

    #[error("Windows API error: {0}")]
    WindowsApi(#[from] windows::core::Error),
}

pub type Result<T> = std::result::Result<T, UiError>;
