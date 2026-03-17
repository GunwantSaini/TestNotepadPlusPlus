//! Configuration management for Notepad++
//!
//! Handles loading and saving configuration files, session management, and recent files

use anyhow::Result;
use std::path::PathBuf;

pub mod parser;
pub mod recent_files;
pub mod session;
pub mod settings;
pub mod theme;

pub use recent_files::{RecentFileEntry, RecentFiles};
pub use session::{Session, SessionFile, ViewData};
pub use settings::{EditorSettings, FileSettings, SessionSettings, Settings, UiSettings};
pub use theme::Theme;

/// Load configuration from config directory
pub fn load_config(config_dir: PathBuf) -> Result<Settings> {
    log::info!("Loading configuration from: {:?}", config_dir);
    Settings::load(config_dir)
}

/// Save configuration
pub fn save_config(settings: &Settings) -> Result<()> {
    log::info!("Saving configuration");
    settings.save()
}

/// Load session file
pub fn load_session(session_file: PathBuf) -> Result<Session> {
    Session::load(session_file)
}

/// Save session file
pub fn save_session(session: &mut Session, session_file: PathBuf) -> Result<()> {
    session.save(session_file)
}

/// Load recent files list
pub fn load_recent_files(recent_file: PathBuf) -> Result<RecentFiles> {
    RecentFiles::load(recent_file)
}

/// Save recent files list
pub fn save_recent_files(recent: &RecentFiles, recent_file: PathBuf) -> Result<()> {
    recent.save(recent_file)
}
