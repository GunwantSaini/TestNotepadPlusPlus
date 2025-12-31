//! Application settings

use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Settings {
    pub config_dir: PathBuf,
    pub remember_session: bool,
    pub multi_instance: bool,
    pub tab_size: u32,
    pub use_spaces: bool,
    pub show_line_numbers: bool,
    pub show_whitespace: bool,
    pub word_wrap: bool,
}

impl Settings {
    pub fn load(config_dir: PathBuf) -> Result<Self> {
        // TODO: Parse config.xml
        Ok(Self {
            config_dir,
            remember_session: true,
            multi_instance: false,
            tab_size: 4,
            use_spaces: false,
            show_line_numbers: true,
            show_whitespace: false,
            word_wrap: false,
        })
    }

    pub fn save(&self) -> Result<()> {
        // TODO: Write config.xml
        Ok(())
    }
}
