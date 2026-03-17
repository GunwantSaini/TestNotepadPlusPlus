//! Split view system for managing multiple editor views

pub mod split;
pub mod sync;
pub mod view;

pub use split::{SplitContainer, SplitDirection, SplitViewManager};
pub use sync::{SyncGroup, SyncGroupId, SyncMode, ViewSynchronizer};
pub use view::{View, ViewId};

use thiserror::Error;

/// Error types for view operations
#[derive(Error, Debug)]
pub enum ViewError {
    #[error("View error: {0}")]
    ViewError(String),

    #[error("Split error: {0}")]
    SplitError(String),

    #[error("Sync error: {0}")]
    SyncError(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

pub type Result<T> = std::result::Result<T, ViewError>;
