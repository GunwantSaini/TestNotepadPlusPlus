//! Rust-native plugin API

use std::any::Any;
use std::collections::HashMap;

/// Plugin metadata and information
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Plugin name
    pub name: String,

    /// Plugin version (semver)
    pub version: String,

    /// Plugin author
    pub author: String,

    /// Plugin description
    pub description: String,

    /// Minimum editor version required
    pub min_editor_version: String,

    /// Plugin homepage URL
    pub homepage: Option<String>,
}

impl PluginInfo {
    /// Create new plugin info
    pub fn new(
        name: String,
        version: String,
        author: String,
        description: String,
    ) -> Self {
        Self {
            name,
            version,
            author,
            description,
            min_editor_version: "8.0.0".to_string(),
            homepage: None,
        }
    }

    /// Set homepage URL
    pub fn with_homepage(mut self, url: String) -> Self {
        self.homepage = Some(url);
        self
    }

    /// Set minimum editor version
    pub fn with_min_version(mut self, version: String) -> Self {
        self.min_editor_version = version;
        self
    }
}

/// Event types that plugins can handle
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginEvent {
    /// Application startup
    Startup,

    /// Application shutdown
    Shutdown,

    /// File opened
    FileOpened(String),

    /// File saved
    FileSaved(String),

    /// File closed
    FileClosed(String),

    /// Text changed
    TextChanged,

    /// Selection changed
    SelectionChanged,

    /// Menu item clicked
    MenuItemClicked(String),

    /// Custom event
    Custom(String),
}

/// Context provided to plugins during execution
pub struct PluginContext {
    /// Current file path (if any)
    pub current_file: Option<String>,

    /// Selected text
    pub selection: Option<String>,

    /// Cursor position (line, column)
    pub cursor_position: Option<(usize, usize)>,

    /// Editor text content
    pub text_content: Option<String>,

    /// Additional context data
    pub data: HashMap<String, Box<dyn Any>>,
}

impl PluginContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            current_file: None,
            selection: None,
            cursor_position: None,
            text_content: None,
            data: HashMap::new(),
        }
    }

    /// Set current file
    pub fn with_file(mut self, path: String) -> Self {
        self.current_file = Some(path);
        self
    }

    /// Set selection
    pub fn with_selection(mut self, text: String) -> Self {
        self.selection = Some(text);
        self
    }

    /// Set cursor position
    pub fn with_cursor(mut self, line: usize, col: usize) -> Self {
        self.cursor_position = Some((line, col));
        self
    }

    /// Set text content
    pub fn with_text(mut self, text: String) -> Self {
        self.text_content = Some(text);
        self
    }

    /// Add custom data
    pub fn add_data<T: Any + 'static>(&mut self, key: String, value: T) {
        self.data.insert(key, Box::new(value));
    }

    /// Get custom data
    pub fn get_data<T: Any + 'static>(&self, key: &str) -> Option<&T> {
        self.data.get(key)?.downcast_ref::<T>()
    }
}

impl Default for PluginContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of plugin execution
#[derive(Debug, Clone)]
pub enum PluginResult {
    /// Plugin executed successfully
    Success,

    /// Plugin executed with a message
    SuccessWithMessage(String),

    /// Plugin wants to modify text
    ModifyText(String),

    /// Plugin wants to insert text
    InsertText(String),

    /// Plugin wants to show a message
    ShowMessage(String),

    /// Plugin execution failed
    Error(String),
}

/// Main plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn info(&self) -> PluginInfo;

    /// Initialize the plugin
    ///
    /// Called when the plugin is loaded
    fn initialize(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Handle an event
    ///
    /// Returns true if the event was handled
    fn on_event(&mut self, event: &PluginEvent, context: &PluginContext) -> bool {
        let _ = (event, context);
        false
    }

    /// Execute the plugin's main action
    fn execute(&mut self, context: &PluginContext) -> PluginResult {
        let _ = context;
        PluginResult::Success
    }

    /// Shutdown the plugin
    ///
    /// Called when the plugin is unloaded
    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Get plugin configuration
    fn get_config(&self, key: &str) -> Option<String> {
        let _ = key;
        None
    }

    /// Set plugin configuration
    fn set_config(&mut self, key: String, value: String) -> Result<(), String> {
        let _ = (key, value);
        Ok(())
    }

    /// Check if plugin has a menu item
    fn has_menu_item(&self) -> bool {
        false
    }

    /// Get menu item text
    fn menu_item_text(&self) -> Option<String> {
        None
    }

    /// Check if plugin supports a file type
    fn supports_file_type(&self, extension: &str) -> bool {
        let _ = extension;
        true // Default: support all file types
    }
}

/// Macro to help create plugins
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::api::Plugin {
            let constructor: fn() -> $plugin_type = $constructor;
            let object = constructor();
            let boxed: Box<dyn $crate::api::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }

        #[no_mangle]
        pub extern "C" fn _plugin_destroy(ptr: *mut dyn $crate::api::Plugin) {
            if !ptr.is_null() {
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        name: String,
        event_count: usize,
    }

    impl TestPlugin {
        fn new() -> Self {
            Self {
                name: "Test Plugin".to_string(),
                event_count: 0,
            }
        }
    }

    impl Plugin for TestPlugin {
        fn info(&self) -> PluginInfo {
            PluginInfo::new(
                self.name.clone(),
                "1.0.0".to_string(),
                "Test Author".to_string(),
                "A test plugin".to_string(),
            )
        }

        fn initialize(&mut self) -> Result<(), String> {
            self.event_count = 0;
            Ok(())
        }

        fn on_event(&mut self, event: &PluginEvent, _context: &PluginContext) -> bool {
            self.event_count += 1;
            matches!(event, PluginEvent::Startup)
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
    fn test_plugin_info() {
        let info = PluginInfo::new(
            "MyPlugin".to_string(),
            "1.0.0".to_string(),
            "Author".to_string(),
            "Description".to_string(),
        );

        assert_eq!(info.name, "MyPlugin");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.author, "Author");
    }

    #[test]
    fn test_plugin_info_builder() {
        let info = PluginInfo::new(
            "MyPlugin".to_string(),
            "1.0.0".to_string(),
            "Author".to_string(),
            "Description".to_string(),
        )
        .with_homepage("https://example.com".to_string())
        .with_min_version("8.1.0".to_string());

        assert_eq!(info.homepage, Some("https://example.com".to_string()));
        assert_eq!(info.min_editor_version, "8.1.0");
    }

    #[test]
    fn test_plugin_context() {
        let mut context = PluginContext::new()
            .with_file("/test/file.txt".to_string())
            .with_selection("selected text".to_string())
            .with_cursor(10, 5);

        assert_eq!(context.current_file, Some("/test/file.txt".to_string()));
        assert_eq!(context.selection, Some("selected text".to_string()));
        assert_eq!(context.cursor_position, Some((10, 5)));

        context.add_data("key".to_string(), 42i32);
        assert_eq!(context.get_data::<i32>("key"), Some(&42));
    }

    #[test]
    fn test_plugin_implementation() {
        let mut plugin = TestPlugin::new();

        // Test info
        let info = plugin.info();
        assert_eq!(info.name, "Test Plugin");

        // Test initialize
        assert!(plugin.initialize().is_ok());
        assert_eq!(plugin.event_count, 0);

        // Test event handling
        let context = PluginContext::new();
        let handled = plugin.on_event(&PluginEvent::Startup, &context);
        assert!(handled);
        assert_eq!(plugin.event_count, 1);

        // Test execute
        let context = PluginContext::new().with_selection("hello".to_string());
        let result = plugin.execute(&context);
        match result {
            PluginResult::ModifyText(text) => assert_eq!(text, "HELLO"),
            _ => panic!("Expected ModifyText result"),
        }
    }

    #[test]
    fn test_plugin_events() {
        let event1 = PluginEvent::Startup;
        let event2 = PluginEvent::FileOpened("test.txt".to_string());
        let event3 = PluginEvent::Custom("custom".to_string());

        assert_ne!(event1, event2);
        assert_ne!(event2, event3);
    }

    #[test]
    fn test_plugin_result_types() {
        let result1 = PluginResult::Success;
        let result2 = PluginResult::ModifyText("modified".to_string());
        let result3 = PluginResult::Error("error".to_string());

        match result1 {
            PluginResult::Success => (),
            _ => panic!("Expected Success"),
        }

        match result2 {
            PluginResult::ModifyText(text) => assert_eq!(text, "modified"),
            _ => panic!("Expected ModifyText"),
        }

        match result3 {
            PluginResult::Error(msg) => assert_eq!(msg, "error"),
            _ => panic!("Expected Error"),
        }
    }
}
