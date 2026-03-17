//! Plugin DLL loader

use crate::{PluginInfo, Result};
use std::path::PathBuf;

pub struct PluginLoader {
    #[allow(dead_code)]
    plugins_dir: PathBuf,
}

impl PluginLoader {
    pub fn new(plugins_dir: PathBuf) -> Self {
        Self { plugins_dir }
    }

    pub fn load_all(&self) -> Result<Vec<PluginInfo>> {
        // TODO: Scan plugins directory and load all DLLs
        Ok(Vec::new())
    }
}
