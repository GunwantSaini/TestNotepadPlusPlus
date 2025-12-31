//! Configuration management for Notepad++
//!
//! Handles loading and saving XML-based configuration files

use anyhow::Result;
use std::path::PathBuf;

pub mod parser;
pub mod settings;
pub mod theme;

pub use settings::Settings;
pub use theme::Theme;

/// Load configuration from XML files
pub fn load_config(config_dir: PathBuf) -> Result<Settings> {
    log::info!("Loading configuration from: {:?}", config_dir);
    Settings::load(config_dir)
}

/// Save configuration to XML files
pub fn save_config(settings: &Settings) -> Result<()> {
    log::info!("Saving configuration");
    settings.save()
}
