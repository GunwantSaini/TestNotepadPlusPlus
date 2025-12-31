//! Text editor component
//!
//! Provides a Rust-based text editing widget using Ropey for text management

pub mod text_buffer;
pub mod view;

#[cfg(windows)]
pub mod scintilla;

pub use text_buffer::TextBuffer;
pub use view::EditorView;

#[cfg(windows)]
pub use scintilla::ScintillaControl;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("Invalid position: {0}")]
    InvalidPosition(usize),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, EditorError>;
