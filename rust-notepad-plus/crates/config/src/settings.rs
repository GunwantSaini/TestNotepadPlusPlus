//! Application settings with persistence

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Configuration directory path (not serialized)
    #[serde(skip)]
    pub config_dir: PathBuf,

    /// Editor settings
    #[serde(default)]
    pub editor: EditorSettings,

    /// UI settings
    #[serde(default)]
    pub ui: UiSettings,

    /// Session settings
    #[serde(default)]
    pub session: SessionSettings,

    /// File settings
    #[serde(default)]
    pub file: FileSettings,
}

/// Editor-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    /// Tab size in spaces
    #[serde(default = "default_tab_size")]
    pub tab_size: u32,

    /// Use spaces instead of tabs
    #[serde(default)]
    pub use_spaces: bool,

    /// Auto-indent new lines
    #[serde(default = "default_true")]
    pub auto_indent: bool,

    /// Show line numbers
    #[serde(default = "default_true")]
    pub show_line_numbers: bool,

    /// Show whitespace characters
    #[serde(default)]
    pub show_whitespace: bool,

    /// Enable word wrap
    #[serde(default)]
    pub word_wrap: bool,

    /// Font family
    #[serde(default = "default_font_family")]
    pub font_family: String,

    /// Font size
    #[serde(default = "default_font_size")]
    pub font_size: u32,
}

/// UI-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    /// Theme name
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Show toolbar
    #[serde(default = "default_true")]
    pub show_toolbar: bool,

    /// Show status bar
    #[serde(default = "default_true")]
    pub show_status_bar: bool,

    /// Show menu bar
    #[serde(default = "default_true")]
    pub show_menu_bar: bool,

    /// Window position (x, y)
    #[serde(default)]
    pub window_position: Option<(i32, i32)>,

    /// Window size (width, height)
    #[serde(default)]
    pub window_size: Option<(u32, u32)>,

    /// Window maximized state
    #[serde(default)]
    pub window_maximized: bool,
}

/// Session management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    /// Remember open files on exit
    #[serde(default = "default_true")]
    pub remember_session: bool,

    /// Session file path
    #[serde(default)]
    pub session_file: Option<PathBuf>,

    /// Allow multiple instances
    #[serde(default)]
    pub multi_instance: bool,

    /// Auto-save interval in seconds (0 = disabled)
    #[serde(default = "default_autosave_interval")]
    pub auto_save_interval: u32,
}

/// File handling settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSettings {
    /// Default encoding
    #[serde(default = "default_encoding")]
    pub default_encoding: String,

    /// Default line ending
    #[serde(default = "default_line_ending")]
    pub default_line_ending: String,

    /// Enable backup
    #[serde(default = "default_true")]
    pub backup_enabled: bool,

    /// Backup directory
    #[serde(default)]
    pub backup_dir: Option<PathBuf>,

    /// Maximum recent files
    #[serde(default = "default_max_recent_files")]
    pub max_recent_files: usize,
}

// Default value functions
fn default_tab_size() -> u32 {
    4
}

fn default_true() -> bool {
    true
}

fn default_font_family() -> String {
    "Consolas".to_string()
}

fn default_font_size() -> u32 {
    11
}

fn default_theme() -> String {
    "default".to_string()
}

fn default_autosave_interval() -> u32 {
    300 // 5 minutes
}

fn default_encoding() -> String {
    "UTF-8".to_string()
}

fn default_line_ending() -> String {
    "Windows".to_string()
}

fn default_max_recent_files() -> usize {
    10
}

// Default implementations
impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            tab_size: default_tab_size(),
            use_spaces: false,
            auto_indent: true,
            show_line_numbers: true,
            show_whitespace: false,
            word_wrap: false,
            font_family: default_font_family(),
            font_size: default_font_size(),
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            show_toolbar: true,
            show_status_bar: true,
            show_menu_bar: true,
            window_position: None,
            window_size: None,
            window_maximized: false,
        }
    }
}

impl Default for SessionSettings {
    fn default() -> Self {
        Self {
            remember_session: true,
            session_file: None,
            multi_instance: false,
            auto_save_interval: default_autosave_interval(),
        }
    }
}

impl Default for FileSettings {
    fn default() -> Self {
        Self {
            default_encoding: default_encoding(),
            default_line_ending: default_line_ending(),
            backup_enabled: true,
            backup_dir: None,
            max_recent_files: default_max_recent_files(),
        }
    }
}

impl Settings {
    /// Create new settings with defaults
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            config_dir,
            editor: EditorSettings::default(),
            ui: UiSettings::default(),
            session: SessionSettings::default(),
            file: FileSettings::default(),
        }
    }

    /// Load settings from config directory
    pub fn load(config_dir: PathBuf) -> Result<Self> {
        let config_file = config_dir.join("config.toml");

        if config_file.exists() {
            log::info!("Loading configuration from: {:?}", config_file);
            let contents = fs::read_to_string(&config_file)
                .context("Failed to read config file")?;

            let mut settings: Settings = toml::from_str(&contents)
                .context("Failed to parse config file")?;

            settings.config_dir = config_dir;
            Ok(settings)
        } else {
            log::info!("No config file found, using defaults");
            Ok(Self::new(config_dir))
        }
    }

    /// Save settings to config directory
    pub fn save(&self) -> Result<()> {
        // Ensure config directory exists
        fs::create_dir_all(&self.config_dir)
            .context("Failed to create config directory")?;

        let config_file = self.config_dir.join("config.toml");
        log::info!("Saving configuration to: {:?}", config_file);

        let toml_string = toml::to_string_pretty(self)
            .context("Failed to serialize settings")?;

        fs::write(&config_file, toml_string)
            .context("Failed to write config file")?;

        Ok(())
    }

    /// Get config file path
    pub fn config_file_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// Reset to defaults
    pub fn reset_to_defaults(&mut self) {
        self.editor = EditorSettings::default();
        self.ui = UiSettings::default();
        self.session = SessionSettings::default();
        self.file = FileSettings::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_settings() {
        let settings = Settings::new(PathBuf::from("/tmp/test"));

        assert_eq!(settings.editor.tab_size, 4);
        assert!(!settings.editor.use_spaces);
        assert!(settings.editor.auto_indent);
        assert!(settings.editor.show_line_numbers);
        assert_eq!(settings.editor.font_family, "Consolas");

        assert_eq!(settings.ui.theme, "default");
        assert!(settings.ui.show_toolbar);

        assert!(settings.session.remember_session);
        assert!(!settings.session.multi_instance);

        assert_eq!(settings.file.default_encoding, "UTF-8");
        assert!(settings.file.backup_enabled);
        assert_eq!(settings.file.max_recent_files, 10);
    }

    #[test]
    fn test_save_and_load() -> Result<()> {
        let temp_dir = env::temp_dir().join("notepad-test-config");
        fs::create_dir_all(&temp_dir)?;

        // Create and save settings
        let mut settings = Settings::new(temp_dir.clone());
        settings.editor.tab_size = 8;
        settings.editor.use_spaces = true;
        settings.ui.theme = "dark".to_string();
        settings.save()?;

        // Load settings
        let loaded = Settings::load(temp_dir.clone())?;

        assert_eq!(loaded.editor.tab_size, 8);
        assert!(loaded.editor.use_spaces);
        assert_eq!(loaded.ui.theme, "dark");

        // Cleanup
        fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    #[test]
    fn test_load_nonexistent() -> Result<()> {
        let temp_dir = env::temp_dir().join("notepad-test-nonexistent");

        // Should return defaults without error
        let settings = Settings::load(temp_dir)?;

        assert_eq!(settings.editor.tab_size, 4);
        assert!(!settings.editor.use_spaces);

        Ok(())
    }

    #[test]
    fn test_toml_serialization() -> Result<()> {
        let settings = Settings::new(PathBuf::from("/tmp/test"));

        let toml_string = toml::to_string_pretty(&settings)?;

        // Verify TOML structure
        assert!(toml_string.contains("[editor]"));
        assert!(toml_string.contains("[ui]"));
        assert!(toml_string.contains("[session]"));
        assert!(toml_string.contains("[file]"));
        assert!(toml_string.contains("tab_size = 4"));

        Ok(())
    }

    #[test]
    fn test_reset_to_defaults() {
        let temp_dir = PathBuf::from("/tmp/test");
        let mut settings = Settings::new(temp_dir);

        // Modify settings
        settings.editor.tab_size = 8;
        settings.ui.theme = "dark".to_string();

        // Reset
        settings.reset_to_defaults();

        assert_eq!(settings.editor.tab_size, 4);
        assert_eq!(settings.ui.theme, "default");
    }
}
