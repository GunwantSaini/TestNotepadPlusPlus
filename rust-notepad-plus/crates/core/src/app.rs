//! Main application state and logic
//!
//! This module contains the NotepadApp struct, which is the Rust equivalent
//! of the C++ Notepad_plus class. It manages the overall application state,
//! documents, views, and command routing.

use crate::{Buffer, BufferId, Command, CommandId, FileManager, Result, ViewId};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Main application state
///
/// Corresponds to the C++ Notepad_plus class (PowerEditor/src/Notepad_plus.h)
pub struct NotepadApp {
    /// File manager (singleton in C++)
    file_manager: Arc<FileManager>,

    /// Active buffers
    buffers: Arc<RwLock<HashMap<BufferId, Arc<Buffer>>>>,

    /// Main view buffer
    main_view_buffer: Arc<RwLock<Option<BufferId>>>,

    /// Sub view buffer (for split view)
    sub_view_buffer: Arc<RwLock<Option<BufferId>>>,

    /// Current active view
    active_view: Arc<RwLock<ViewId>>,

    /// Command handlers
    command_handlers: Arc<RwLock<HashMap<CommandId, Box<dyn Fn(&NotepadApp) -> Result<()> + Send + Sync>>>>,

    /// Application settings
    config: Arc<RwLock<AppConfig>>,
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Multi-instance mode
    pub multi_instance: bool,

    /// Session file path
    pub session_path: Option<PathBuf>,

    /// Remember session
    pub remember_session: bool,

    /// Backup settings
    pub backup_enabled: bool,
    pub backup_dir: Option<PathBuf>,

    /// Tab settings
    pub tab_size: u32,
    pub use_spaces: bool,

    /// Line ending
    pub default_eol: EolFormat,

    /// Encoding
    pub default_encoding: EncodingType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EolFormat {
    Windows,  // CRLF
    Unix,     // LF
    Mac,      // CR
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingType {
    Ansi,
    Utf8,
    Utf8Bom,
    Utf16Le,
    Utf16LeBom,
    Utf16Be,
    Utf16BeBom,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            multi_instance: false,
            session_path: None,
            remember_session: true,
            backup_enabled: true,
            backup_dir: None,
            tab_size: 4,
            use_spaces: false,
            default_eol: EolFormat::Windows,
            default_encoding: EncodingType::Utf8,
        }
    }
}

impl NotepadApp {
    /// Create a new Notepad++ application instance
    pub fn new() -> Result<Self> {
        log::info!("Initializing NotepadApp");

        Ok(Self {
            file_manager: Arc::new(FileManager::new()?),
            buffers: Arc::new(RwLock::new(HashMap::new())),
            main_view_buffer: Arc::new(RwLock::new(None)),
            sub_view_buffer: Arc::new(RwLock::new(None)),
            active_view: Arc::new(RwLock::new(ViewId::Main)),
            command_handlers: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(AppConfig::default())),
        })
    }

    /// Open a file and return its buffer ID
    pub fn open_file(&self, path: PathBuf) -> Result<BufferId> {
        log::info!("Opening file: {:?}", path);

        // Load file through file manager
        let buffer = self.file_manager.load_file(path)?;
        let buffer_id = buffer.id();

        // Add to buffers map
        self.buffers.write().insert(buffer_id, Arc::new(buffer));

        // Set as active buffer in main view if no buffer is active
        let mut main_view = self.main_view_buffer.write();
        if main_view.is_none() {
            *main_view = Some(buffer_id);
        }

        Ok(buffer_id)
    }

    /// Save a buffer to disk
    pub fn save_buffer(&self, buffer_id: BufferId) -> Result<()> {
        log::info!("Saving buffer: {:?}", buffer_id);

        let buffers = self.buffers.read();
        let buffer = buffers
            .get(&buffer_id)
            .ok_or(crate::NotepadError::InvalidBuffer(buffer_id))?;

        self.file_manager.save_buffer(buffer)
    }

    /// Close a buffer
    pub fn close_buffer(&self, buffer_id: BufferId) -> Result<()> {
        log::info!("Closing buffer: {:?}", buffer_id);

        // Remove from buffers map
        self.buffers.write().remove(&buffer_id);

        // Clear from views if active
        let mut main_view = self.main_view_buffer.write();
        if *main_view == Some(buffer_id) {
            *main_view = None;
        }

        let mut sub_view = self.sub_view_buffer.write();
        if *sub_view == Some(buffer_id) {
            *sub_view = None;
        }

        Ok(())
    }

    /// Get active buffer ID
    pub fn get_active_buffer(&self) -> Option<BufferId> {
        match *self.active_view.read() {
            ViewId::Main => *self.main_view_buffer.read(),
            ViewId::Sub => *self.sub_view_buffer.read(),
        }
    }

    /// Execute a command
    pub fn execute_command(&self, command_id: CommandId) -> Result<()> {
        log::debug!("Executing command: {:?}", command_id);

        let handlers = self.command_handlers.read();
        if let Some(handler) = handlers.get(&command_id) {
            handler(self)
        } else {
            Err(crate::NotepadError::Command(format!(
                "No handler for command: {:?}",
                command_id
            )))
        }
    }

    /// Get application configuration
    pub fn config(&self) -> AppConfig {
        self.config.read().clone()
    }

    /// Update application configuration
    pub fn update_config<F>(&self, f: F)
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.write();
        f(&mut config);
    }

    /// Get buffer by ID
    pub fn get_buffer(&self, buffer_id: BufferId) -> Option<Arc<Buffer>> {
        self.buffers.read().get(&buffer_id).cloned()
    }

    /// Get all open buffer IDs
    pub fn get_all_buffers(&self) -> Vec<BufferId> {
        self.buffers.read().keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = NotepadApp::new();
        assert!(app.is_ok());
    }

    #[test]
    fn test_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.tab_size, 4);
        assert_eq!(config.default_eol, EolFormat::Windows);
    }
}
