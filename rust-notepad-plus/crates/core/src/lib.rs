//! Core application logic for Notepad++ Rust
//!
//! This crate contains the main application state, buffer management,
//! and command routing logic.

pub mod app;
pub mod buffer;
pub mod commands;
pub mod file_manager;
pub mod view;

// Re-exports
pub use app::NotepadApp;
pub use buffer::{Buffer, BufferId};
pub use commands::{Command, CommandId};
pub use file_manager::FileManager;
pub use view::ViewId;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotepadError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid buffer ID: {0:?}")]
    InvalidBuffer(BufferId),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Encoding error: {0}")]
    Encoding(String),

    #[error("Command error: {0}")]
    Command(String),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, NotepadError>;
