//! Text editor component
//!
//! Provides a Rust-based text editing widget using Ropey for text management

pub mod text_buffer;
pub mod view;

pub use text_buffer::TextBuffer;
pub use view::EditorView;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("Invalid position: {0}")]
    InvalidPosition(usize),

    #[error("Windows API error: {0}")]
    WindowsApi(#[from] windows::core::Error),
}

pub type Result<T> = std::result::Result<T, EditorError>;
