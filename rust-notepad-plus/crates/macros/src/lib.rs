//! Macro recording and playback system for Notepad++

pub mod action;
pub mod macro_def;
pub mod manager;
pub mod playback;
pub mod recorder;

pub use action::MacroAction;
pub use macro_def::{Macro, MacroStatistics};
pub use manager::MacroManager;
pub use playback::{ActionHandler, MacroPlayback, PlaybackOptions, PlaybackResult};
pub use recorder::{create_shared_recorder, MacroRecorder, RecorderState, SharedRecorder};

use thiserror::Error;

/// Error types for macro operations
#[derive(Error, Debug)]
pub enum MacroError {
    #[error("Recording error: {0}")]
    RecordingError(String),

    #[error("Playback error: {0}")]
    PlaybackError(String),

    #[error("Invalid macro: {0}")]
    InvalidMacro(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, MacroError>;
