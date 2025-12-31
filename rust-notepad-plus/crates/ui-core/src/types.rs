//! Common types for platform-independent UI

use std::path::PathBuf;
use thiserror::Error;

/// File filter for open/save dialogs
#[derive(Debug, Clone)]
pub struct FileFilter {
    pub name: String,
    pub patterns: Vec<String>,
}

impl FileFilter {
    pub fn new(name: impl Into<String>, patterns: Vec<String>) -> Self {
        Self {
            name: name.into(),
            patterns,
        }
    }

    /// Create a filter for text files
    pub fn text_files() -> Self {
        Self::new(
            "Text Files",
            vec![
                "*.txt".into(),
                "*.log".into(),
                "*.md".into(),
                "*.rs".into(),
                "*.cpp".into(),
                "*.h".into(),
                "*.c".into(),
                "*.js".into(),
                "*.py".into(),
                "*.java".into(),
            ],
        )
    }

    /// Create a filter for all files
    pub fn all_files() -> Self {
        Self::new("All Files", vec!["*.*".into()])
    }
}

/// Message type for dialogs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Info,
    Warning,
    Error,
    Question,
}

/// Dialog result for Yes/No/Cancel questions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogResult {
    Yes,
    No,
    Cancel,
}

/// Find dialog result
#[derive(Debug, Clone)]
pub struct FindDialogResult {
    pub search_text: String,
    pub case_sensitive: bool,
    pub wrap_around: bool,
    pub search_direction: SearchDirection,
}

/// Replace dialog result
#[derive(Debug, Clone)]
pub struct ReplaceDialogResult {
    pub search_text: String,
    pub replace_text: String,
    pub case_sensitive: bool,
    pub replace_all: bool,
}

/// Search direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirection {
    Forward,
    Backward,
}

/// Menu group (e.g., File, Edit, etc.)
#[derive(Debug, Clone)]
pub struct MenuGroup {
    pub label: String,
    pub items: Vec<MenuItem>,
}

/// Menu item
#[derive(Debug, Clone)]
pub enum MenuItem {
    Command {
        label: String,
        id: notepad_core::CommandId,
        shortcut: Option<String>,
        enabled: bool,
    },
    Separator,
    Submenu {
        label: String,
        items: Vec<MenuItem>,
    },
}

/// Menu item state
#[derive(Debug, Clone, Copy)]
pub struct MenuItemState {
    pub enabled: bool,
    pub checked: bool,
    pub visible: bool,
}

impl Default for MenuItemState {
    fn default() -> Self {
        Self {
            enabled: true,
            checked: false,
            visible: true,
        }
    }
}

/// UI error types
#[derive(Error, Debug)]
pub enum UiError {
    #[error("Platform initialization failed: {0}")]
    PlatformInit(String),

    #[error("Window creation failed: {0}")]
    WindowCreation(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Dialog error: {0}")]
    DialogError(String),

    #[error("Menu error: {0}")]
    MenuError(String),

    #[error("Other error: {0}")]
    Other(String),
}
