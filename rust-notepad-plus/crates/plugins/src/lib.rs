//! Plugin system for loading and managing plugins

pub mod api;
pub mod examples;
pub mod interface;
pub mod loader;
pub mod manager;

pub use api::{Plugin, PluginContext, PluginEvent, PluginInfo, PluginResult};
pub use examples::*;
pub use interface::*;
pub use loader::PluginLoader;
pub use manager::PluginManager;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Failed to load plugin: {0}")]
    LoadError(String),

    #[error("Invalid plugin: {0}")]
    InvalidPlugin(String),

    #[error("Plugin function not found: {0}")]
    FunctionNotFound(String),
}

pub type Result<T> = std::result::Result<T, PluginError>;
