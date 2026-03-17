//! Example plugins to demonstrate the plugin API

use crate::api::{Plugin, PluginContext, PluginInfo, PluginResult};

/// Example plugin that converts text to uppercase
pub struct UppercasePlugin {
    name: String,
}

impl UppercasePlugin {
    pub fn new() -> Self {
        Self {
            name: "Uppercase Plugin".to_string(),
        }
    }
}

impl Default for UppercasePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for UppercasePlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo::new(
            self.name.clone(),
            "1.0.0".to_string(),
            "Notepad++ Rust Team".to_string(),
            "Converts selected text to uppercase".to_string(),
        )
        .with_homepage("https://github.com/notepad-plus-rust/plugins".to_string())
    }

    fn execute(&mut self, context: &PluginContext) -> PluginResult {
        if let Some(selection) = &context.selection {
            PluginResult::ModifyText(selection.to_uppercase())
        } else if let Some(text) = &context.text_content {
            PluginResult::ModifyText(text.to_uppercase())
        } else {
            PluginResult::Error("No text to convert".to_string())
        }
    }

    fn has_menu_item(&self) -> bool {
        true
    }

    fn menu_item_text(&self) -> Option<String> {
        Some("Convert to UPPERCASE".to_string())
    }
}

/// Example plugin that converts text to lowercase
pub struct LowercasePlugin {
    name: String,
}

impl LowercasePlugin {
    pub fn new() -> Self {
        Self {
            name: "Lowercase Plugin".to_string(),
        }
    }
}

impl Default for LowercasePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for LowercasePlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo::new(
            self.name.clone(),
            "1.0.0".to_string(),
            "Notepad++ Rust Team".to_string(),
            "Converts selected text to lowercase".to_string(),
        )
    }

    fn execute(&mut self, context: &PluginContext) -> PluginResult {
        if let Some(selection) = &context.selection {
            PluginResult::ModifyText(selection.to_lowercase())
        } else if let Some(text) = &context.text_content {
            PluginResult::ModifyText(text.to_lowercase())
        } else {
            PluginResult::Error("No text to convert".to_string())
        }
    }

    fn has_menu_item(&self) -> bool {
        true
    }

    fn menu_item_text(&self) -> Option<String> {
        Some("Convert to lowercase".to_string())
    }
}

/// Example plugin that counts words in selection
pub struct WordCountPlugin {
    name: String,
}

impl WordCountPlugin {
    pub fn new() -> Self {
        Self {
            name: "Word Count Plugin".to_string(),
        }
    }
}

impl Default for WordCountPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for WordCountPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo::new(
            self.name.clone(),
            "1.0.0".to_string(),
            "Notepad++ Rust Team".to_string(),
            "Counts words in selected text".to_string(),
        )
    }

    fn execute(&mut self, context: &PluginContext) -> PluginResult {
        let text = if let Some(selection) = &context.selection {
            selection
        } else if let Some(content) = &context.text_content {
            content
        } else {
            return PluginResult::Error("No text to count".to_string());
        };

        let word_count = text.split_whitespace().count();
        let char_count = text.chars().count();
        let line_count = text.lines().count();

        let message = format!(
            "Statistics:\nWords: {}\nCharacters: {}\nLines: {}",
            word_count, char_count, line_count
        );

        PluginResult::ShowMessage(message)
    }

    fn has_menu_item(&self) -> bool {
        true
    }

    fn menu_item_text(&self) -> Option<String> {
        Some("Word Count".to_string())
    }
}

/// Example plugin that reverses text
pub struct ReverseTextPlugin {
    name: String,
}

impl ReverseTextPlugin {
    pub fn new() -> Self {
        Self {
            name: "Reverse Text Plugin".to_string(),
        }
    }
}

impl Default for ReverseTextPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ReverseTextPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo::new(
            self.name.clone(),
            "1.0.0".to_string(),
            "Notepad++ Rust Team".to_string(),
            "Reverses selected text".to_string(),
        )
    }

    fn execute(&mut self, context: &PluginContext) -> PluginResult {
        if let Some(selection) = &context.selection {
            let reversed: String = selection.chars().rev().collect();
            PluginResult::ModifyText(reversed)
        } else {
            PluginResult::Error("No text to reverse".to_string())
        }
    }

    fn has_menu_item(&self) -> bool {
        true
    }

    fn menu_item_text(&self) -> Option<String> {
        Some("Reverse Text".to_string())
    }
}

/// Example plugin that removes duplicate lines
pub struct RemoveDuplicatesPlugin {
    name: String,
}

impl RemoveDuplicatesPlugin {
    pub fn new() -> Self {
        Self {
            name: "Remove Duplicates Plugin".to_string(),
        }
    }
}

impl Default for RemoveDuplicatesPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveDuplicatesPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo::new(
            self.name.clone(),
            "1.0.0".to_string(),
            "Notepad++ Rust Team".to_string(),
            "Removes duplicate lines from selection".to_string(),
        )
    }

    fn execute(&mut self, context: &PluginContext) -> PluginResult {
        let text = if let Some(selection) = &context.selection {
            selection
        } else if let Some(content) = &context.text_content {
            content
        } else {
            return PluginResult::Error("No text to process".to_string());
        };

        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();

        for line in text.lines() {
            if seen.insert(line) {
                result.push(line);
            }
        }

        PluginResult::ModifyText(result.join("\n"))
    }

    fn has_menu_item(&self) -> bool {
        true
    }

    fn menu_item_text(&self) -> Option<String> {
        Some("Remove Duplicate Lines".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase_plugin() {
        let mut plugin = UppercasePlugin::new();

        let context = PluginContext::new().with_selection("hello world".to_string());
        let result = plugin.execute(&context);

        match result {
            PluginResult::ModifyText(text) => assert_eq!(text, "HELLO WORLD"),
            _ => panic!("Expected ModifyText"),
        }
    }

    #[test]
    fn test_lowercase_plugin() {
        let mut plugin = LowercasePlugin::new();

        let context = PluginContext::new().with_selection("HELLO WORLD".to_string());
        let result = plugin.execute(&context);

        match result {
            PluginResult::ModifyText(text) => assert_eq!(text, "hello world"),
            _ => panic!("Expected ModifyText"),
        }
    }

    #[test]
    fn test_word_count_plugin() {
        let mut plugin = WordCountPlugin::new();

        let context = PluginContext::new().with_selection("hello world\ntest".to_string());
        let result = plugin.execute(&context);

        match result {
            PluginResult::ShowMessage(msg) => {
                assert!(msg.contains("Words: 3"));
                assert!(msg.contains("Lines: 2"));
            }
            _ => panic!("Expected ShowMessage"),
        }
    }

    #[test]
    fn test_reverse_text_plugin() {
        let mut plugin = ReverseTextPlugin::new();

        let context = PluginContext::new().with_selection("hello".to_string());
        let result = plugin.execute(&context);

        match result {
            PluginResult::ModifyText(text) => assert_eq!(text, "olleh"),
            _ => panic!("Expected ModifyText"),
        }
    }

    #[test]
    fn test_remove_duplicates_plugin() {
        let mut plugin = RemoveDuplicatesPlugin::new();

        let text = "apple\nbanana\napple\ncherry\nbanana";
        let context = PluginContext::new().with_selection(text.to_string());
        let result = plugin.execute(&context);

        match result {
            PluginResult::ModifyText(text) => {
                assert_eq!(text, "apple\nbanana\ncherry");
            }
            _ => panic!("Expected ModifyText"),
        }
    }

    #[test]
    fn test_plugin_menu_items() {
        let uppercase = UppercasePlugin::new();
        assert!(uppercase.has_menu_item());
        assert_eq!(uppercase.menu_item_text(), Some("Convert to UPPERCASE".to_string()));

        let lowercase = LowercasePlugin::new();
        assert!(lowercase.has_menu_item());

        let word_count = WordCountPlugin::new();
        assert!(word_count.has_menu_item());
    }
}
