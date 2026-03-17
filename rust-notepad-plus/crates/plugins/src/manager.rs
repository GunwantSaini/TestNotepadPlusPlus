//! Plugin manager for loading and managing plugins

use crate::api::{Plugin, PluginContext, PluginEvent, PluginInfo, PluginResult};
use crate::{PluginError, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Manages loaded plugins
pub struct PluginManager {
    /// Directory where plugins are stored
    plugins_dir: PathBuf,

    /// Loaded plugins (name -> plugin)
    plugins: HashMap<String, Arc<RwLock<Box<dyn Plugin>>>>,

    /// Plugin load order
    load_order: Vec<String>,

    /// Enabled/disabled state
    enabled_plugins: HashMap<String, bool>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new(plugins_dir: PathBuf) -> Self {
        Self {
            plugins_dir,
            plugins: HashMap::new(),
            load_order: Vec::new(),
            enabled_plugins: HashMap::new(),
        }
    }

    /// Register a plugin
    ///
    /// This is used for statically-linked plugins
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<()> {
        let info = plugin.info();
        let name = info.name.clone();

        log::info!("Registering plugin: {}", name);

        self.plugins.insert(name.clone(), Arc::new(RwLock::new(plugin)));
        self.load_order.push(name.clone());
        self.enabled_plugins.insert(name, true);

        Ok(())
    }

    /// Initialize all plugins
    pub fn initialize_all(&mut self) -> Result<()> {
        for name in &self.load_order.clone() {
            if let Some(plugin) = self.plugins.get(name) {
                let mut plugin = plugin.write().map_err(|e| {
                    PluginError::LoadError(format!("Failed to lock plugin {}: {}", name, e))
                })?;

                if let Err(e) = plugin.initialize() {
                    log::error!("Failed to initialize plugin {}: {}", name, e);
                    self.enabled_plugins.insert(name.clone(), false);
                } else {
                    log::info!("Initialized plugin: {}", name);
                }
            }
        }

        Ok(())
    }

    /// Shutdown all plugins
    pub fn shutdown_all(&mut self) -> Result<()> {
        for name in self.load_order.iter().rev() {
            if let Some(plugin) = self.plugins.get(name) {
                let mut plugin = plugin.write().map_err(|e| {
                    PluginError::LoadError(format!("Failed to lock plugin {}: {}", name, e))
                })?;

                if let Err(e) = plugin.shutdown() {
                    log::error!("Failed to shutdown plugin {}: {}", name, e);
                } else {
                    log::info!("Shutdown plugin: {}", name);
                }
            }
        }

        Ok(())
    }

    /// Dispatch an event to all plugins
    pub fn dispatch_event(&self, event: &PluginEvent, context: &PluginContext) {
        for name in &self.load_order {
            if !self.is_enabled(name) {
                continue;
            }

            if let Some(plugin) = self.plugins.get(name) {
                if let Ok(mut plugin) = plugin.write() {
                    let handled = plugin.on_event(event, context);
                    if handled {
                        log::debug!("Plugin {} handled event: {:?}", name, event);
                    }
                }
            }
        }
    }

    /// Execute a specific plugin
    pub fn execute_plugin(&self, name: &str, context: &PluginContext) -> Result<PluginResult> {
        if !self.is_enabled(name) {
            return Err(PluginError::InvalidPlugin(format!(
                "Plugin {} is disabled",
                name
            )));
        }

        let plugin = self.plugins.get(name).ok_or_else(|| {
            PluginError::InvalidPlugin(format!("Plugin {} not found", name))
        })?;

        let mut plugin = plugin.write().map_err(|e| {
            PluginError::LoadError(format!("Failed to lock plugin {}: {}", name, e))
        })?;

        Ok(plugin.execute(context))
    }

    /// Get information about a plugin
    pub fn get_plugin_info(&self, name: &str) -> Option<PluginInfo> {
        let plugin = self.plugins.get(name)?;
        let plugin = plugin.read().ok()?;
        Some(plugin.info())
    }

    /// Get all plugin info
    pub fn get_all_plugin_info(&self) -> Vec<PluginInfo> {
        self.load_order
            .iter()
            .filter_map(|name| self.get_plugin_info(name))
            .collect()
    }

    /// Get plugins directory
    pub fn plugins_directory(&self) -> &PathBuf {
        &self.plugins_dir
    }

    /// Get list of loaded plugin names
    pub fn loaded_plugins(&self) -> Vec<String> {
        self.load_order.clone()
    }

    /// Check if a plugin is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled_plugins.get(name).copied().unwrap_or(false)
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, name: &str) -> Result<()> {
        if !self.plugins.contains_key(name) {
            return Err(PluginError::InvalidPlugin(format!(
                "Plugin {} not found",
                name
            )));
        }

        self.enabled_plugins.insert(name.to_string(), true);
        log::info!("Enabled plugin: {}", name);
        Ok(())
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, name: &str) -> Result<()> {
        if !self.plugins.contains_key(name) {
            return Err(PluginError::InvalidPlugin(format!(
                "Plugin {} not found",
                name
            )));
        }

        self.enabled_plugins.insert(name.to_string(), false);
        log::info!("Disabled plugin: {}", name);
        Ok(())
    }

    /// Get number of loaded plugins
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Check if a plugin is loaded
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Plugin;
    use std::env;

    struct MockPlugin {
        name: String,
        initialized: bool,
        event_count: usize,
    }

    impl MockPlugin {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                initialized: false,
                event_count: 0,
            }
        }
    }

    impl Plugin for MockPlugin {
        fn info(&self) -> PluginInfo {
            PluginInfo::new(
                self.name.clone(),
                "1.0.0".to_string(),
                "Test".to_string(),
                "Mock plugin".to_string(),
            )
        }

        fn initialize(&mut self) -> std::result::Result<(), String> {
            self.initialized = true;
            Ok(())
        }

        fn on_event(&mut self, _event: &PluginEvent, _context: &PluginContext) -> bool {
            self.event_count += 1;
            true
        }

        fn execute(&mut self, context: &PluginContext) -> PluginResult {
            if let Some(text) = &context.selection {
                PluginResult::ModifyText(text.to_uppercase())
            } else {
                PluginResult::Success
            }
        }
    }

    #[test]
    fn test_plugin_manager_creation() {
        let temp_dir = env::temp_dir();
        let manager = PluginManager::new(temp_dir.clone());

        assert_eq!(manager.plugins_directory(), &temp_dir);
        assert_eq!(manager.plugin_count(), 0);
    }

    #[test]
    fn test_register_plugin() {
        let temp_dir = env::temp_dir();
        let mut manager = PluginManager::new(temp_dir);

        let plugin = Box::new(MockPlugin::new("TestPlugin"));
        manager.register_plugin(plugin).unwrap();

        assert_eq!(manager.plugin_count(), 1);
        assert!(manager.has_plugin("TestPlugin"));
    }

    #[test]
    fn test_initialize_plugins() {
        let temp_dir = env::temp_dir();
        let mut manager = PluginManager::new(temp_dir);

        manager.register_plugin(Box::new(MockPlugin::new("Plugin1"))).unwrap();
        manager.register_plugin(Box::new(MockPlugin::new("Plugin2"))).unwrap();

        manager.initialize_all().unwrap();

        // Plugins should be initialized
        assert!(manager.is_enabled("Plugin1"));
        assert!(manager.is_enabled("Plugin2"));
    }

    #[test]
    fn test_dispatch_event() {
        let temp_dir = env::temp_dir();
        let mut manager = PluginManager::new(temp_dir);

        manager.register_plugin(Box::new(MockPlugin::new("TestPlugin"))).unwrap();
        manager.initialize_all().unwrap();

        let context = PluginContext::new();
        manager.dispatch_event(&PluginEvent::Startup, &context);

        // Event should have been dispatched
    }

    #[test]
    fn test_execute_plugin() {
        let temp_dir = env::temp_dir();
        let mut manager = PluginManager::new(temp_dir);

        manager.register_plugin(Box::new(MockPlugin::new("TestPlugin"))).unwrap();
        manager.initialize_all().unwrap();

        let context = PluginContext::new().with_selection("hello".to_string());
        let result = manager.execute_plugin("TestPlugin", &context).unwrap();

        match result {
            PluginResult::ModifyText(text) => assert_eq!(text, "HELLO"),
            _ => panic!("Expected ModifyText result"),
        }
    }

    #[test]
    fn test_enable_disable_plugin() {
        let temp_dir = env::temp_dir();
        let mut manager = PluginManager::new(temp_dir);

        manager.register_plugin(Box::new(MockPlugin::new("TestPlugin"))).unwrap();

        assert!(manager.is_enabled("TestPlugin"));

        manager.disable_plugin("TestPlugin").unwrap();
        assert!(!manager.is_enabled("TestPlugin"));

        manager.enable_plugin("TestPlugin").unwrap();
        assert!(manager.is_enabled("TestPlugin"));
    }

    #[test]
    fn test_get_plugin_info() {
        let temp_dir = env::temp_dir();
        let mut manager = PluginManager::new(temp_dir);

        manager.register_plugin(Box::new(MockPlugin::new("TestPlugin"))).unwrap();

        let info = manager.get_plugin_info("TestPlugin").unwrap();
        assert_eq!(info.name, "TestPlugin");
        assert_eq!(info.version, "1.0.0");
    }

    #[test]
    fn test_get_all_plugin_info() {
        let temp_dir = env::temp_dir();
        let mut manager = PluginManager::new(temp_dir);

        manager.register_plugin(Box::new(MockPlugin::new("Plugin1"))).unwrap();
        manager.register_plugin(Box::new(MockPlugin::new("Plugin2"))).unwrap();

        let all_info = manager.get_all_plugin_info();
        assert_eq!(all_info.len(), 2);
    }

    #[test]
    fn test_loaded_plugins() {
        let temp_dir = env::temp_dir();
        let mut manager = PluginManager::new(temp_dir);

        manager.register_plugin(Box::new(MockPlugin::new("Plugin1"))).unwrap();
        manager.register_plugin(Box::new(MockPlugin::new("Plugin2"))).unwrap();

        let loaded = manager.loaded_plugins();
        assert_eq!(loaded.len(), 2);
        assert!(loaded.contains(&"Plugin1".to_string()));
        assert!(loaded.contains(&"Plugin2".to_string()));
    }
}
