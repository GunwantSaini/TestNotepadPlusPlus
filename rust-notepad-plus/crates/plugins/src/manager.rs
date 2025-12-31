//! Plugin manager

use crate::{PluginInfo, PluginLoader, Result};
use std::path::PathBuf;

pub struct PluginManager {
    loader: PluginLoader,
    plugins: Vec<PluginInfo>,
}

impl PluginManager {
    pub fn new(plugins_dir: PathBuf) -> Result<Self> {
        let loader = PluginLoader::new(plugins_dir);
        Ok(Self {
            loader,
            plugins: Vec::new(),
        })
    }

    pub fn load_plugins(&mut self) -> Result<()> {
        self.plugins = self.loader.load_all()?;
        log::info!("Loaded {} plugins", self.plugins.len());
        Ok(())
    }

    pub fn get_plugins(&self) -> &[PluginInfo] {
        &self.plugins
    }
}
