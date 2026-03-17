//! Macro definition and metadata

use crate::action::MacroAction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A complete macro definition with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    /// Unique identifier
    pub id: String,

    /// Macro name
    pub name: String,

    /// Macro description
    pub description: String,

    /// Actions in the macro
    pub actions: Vec<MacroAction>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,

    /// Author/creator
    pub author: Option<String>,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Keyboard shortcut (if any)
    pub shortcut: Option<String>,

    /// Number of times executed
    pub execution_count: usize,
}

impl Macro {
    /// Create a new macro
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            actions: Vec::new(),
            created_at: now,
            modified_at: now,
            author: None,
            tags: Vec::new(),
            shortcut: None,
            execution_count: 0,
        }
    }

    /// Create a new macro with actions
    pub fn with_actions(name: String, description: String, actions: Vec<MacroAction>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            actions,
            created_at: now,
            modified_at: now,
            author: None,
            tags: Vec::new(),
            shortcut: None,
            execution_count: 0,
        }
    }

    /// Add an action to the macro
    pub fn add_action(&mut self, action: MacroAction) {
        self.actions.push(action);
        self.modified_at = Utc::now();
    }

    /// Insert an action at a specific position
    pub fn insert_action(&mut self, index: usize, action: MacroAction) {
        if index <= self.actions.len() {
            self.actions.insert(index, action);
            self.modified_at = Utc::now();
        }
    }

    /// Remove an action at a specific position
    pub fn remove_action(&mut self, index: usize) -> Option<MacroAction> {
        if index < self.actions.len() {
            self.modified_at = Utc::now();
            Some(self.actions.remove(index))
        } else {
            None
        }
    }

    /// Replace an action at a specific position
    pub fn replace_action(&mut self, index: usize, action: MacroAction) -> Option<MacroAction> {
        if index < self.actions.len() {
            self.modified_at = Utc::now();
            Some(std::mem::replace(&mut self.actions[index], action))
        } else {
            None
        }
    }

    /// Clear all actions
    pub fn clear_actions(&mut self) {
        self.actions.clear();
        self.modified_at = Utc::now();
    }

    /// Get the number of actions
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Check if the macro is empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Remove a tag
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if macro has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Set the keyboard shortcut
    pub fn set_shortcut(&mut self, shortcut: Option<String>) {
        self.shortcut = shortcut;
        self.modified_at = Utc::now();
    }

    /// Increment execution count
    pub fn increment_execution_count(&mut self) {
        self.execution_count += 1;
    }

    /// Get statistics about the macro
    pub fn statistics(&self) -> MacroStatistics {
        let total_actions = self.actions.len();
        let modifying_actions = self
            .actions
            .iter()
            .filter(|a| a.is_modifying())
            .count();
        let navigation_actions = self
            .actions
            .iter()
            .filter(|a| a.is_navigation())
            .count();
        let selection_actions = self
            .actions
            .iter()
            .filter(|a| a.is_selection())
            .count();

        MacroStatistics {
            total_actions,
            modifying_actions,
            navigation_actions,
            selection_actions,
            execution_count: self.execution_count,
        }
    }

    /// Validate the macro
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Macro name cannot be empty".to_string());
        }

        if self.actions.is_empty() {
            return Err("Macro must have at least one action".to_string());
        }

        Ok(())
    }

    /// Create a copy with a new name
    pub fn duplicate(&self, new_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: new_name,
            description: self.description.clone(),
            actions: self.actions.clone(),
            created_at: now,
            modified_at: now,
            author: self.author.clone(),
            tags: self.tags.clone(),
            shortcut: None, // Don't duplicate shortcut to avoid conflicts
            execution_count: 0,
        }
    }
}

/// Statistics about a macro
#[derive(Debug, Clone)]
pub struct MacroStatistics {
    /// Total number of actions
    pub total_actions: usize,

    /// Number of actions that modify the document
    pub modifying_actions: usize,

    /// Number of navigation actions
    pub navigation_actions: usize,

    /// Number of selection actions
    pub selection_actions: usize,

    /// Number of times the macro has been executed
    pub execution_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::MacroAction;

    #[test]
    fn test_macro_creation() {
        let macro_def = Macro::new("Test Macro".to_string(), "A test macro".to_string());
        assert_eq!(macro_def.name, "Test Macro");
        assert_eq!(macro_def.description, "A test macro");
        assert!(macro_def.is_empty());
        assert_eq!(macro_def.execution_count, 0);
    }

    #[test]
    fn test_add_action() {
        let mut macro_def = Macro::new("Test".to_string(), "Test".to_string());
        macro_def.add_action(MacroAction::InsertText {
            text: "Hello".to_string(),
        });
        assert_eq!(macro_def.action_count(), 1);
        assert!(!macro_def.is_empty());
    }

    #[test]
    fn test_insert_remove_action() {
        let mut macro_def = Macro::new("Test".to_string(), "Test".to_string());
        macro_def.add_action(MacroAction::Copy);
        macro_def.add_action(MacroAction::Paste);

        macro_def.insert_action(1, MacroAction::Cut);
        assert_eq!(macro_def.action_count(), 3);

        let removed = macro_def.remove_action(1);
        assert!(matches!(removed, Some(MacroAction::Cut)));
        assert_eq!(macro_def.action_count(), 2);
    }

    #[test]
    fn test_replace_action() {
        let mut macro_def = Macro::new("Test".to_string(), "Test".to_string());
        macro_def.add_action(MacroAction::Copy);

        let old = macro_def.replace_action(0, MacroAction::Cut);
        assert!(matches!(old, Some(MacroAction::Copy)));
        assert!(matches!(macro_def.actions[0], MacroAction::Cut));
    }

    #[test]
    fn test_clear_actions() {
        let mut macro_def = Macro::new("Test".to_string(), "Test".to_string());
        macro_def.add_action(MacroAction::Copy);
        macro_def.add_action(MacroAction::Paste);

        macro_def.clear_actions();
        assert!(macro_def.is_empty());
    }

    #[test]
    fn test_tags() {
        let mut macro_def = Macro::new("Test".to_string(), "Test".to_string());

        macro_def.add_tag("formatting".to_string());
        macro_def.add_tag("text".to_string());
        assert_eq!(macro_def.tags.len(), 2);
        assert!(macro_def.has_tag("formatting"));

        // Adding duplicate tag should not increase count
        macro_def.add_tag("formatting".to_string());
        assert_eq!(macro_def.tags.len(), 2);

        assert!(macro_def.remove_tag("formatting"));
        assert_eq!(macro_def.tags.len(), 1);
        assert!(!macro_def.has_tag("formatting"));
    }

    #[test]
    fn test_shortcut() {
        let mut macro_def = Macro::new("Test".to_string(), "Test".to_string());
        assert!(macro_def.shortcut.is_none());

        macro_def.set_shortcut(Some("Ctrl+Shift+M".to_string()));
        assert_eq!(macro_def.shortcut, Some("Ctrl+Shift+M".to_string()));

        macro_def.set_shortcut(None);
        assert!(macro_def.shortcut.is_none());
    }

    #[test]
    fn test_execution_count() {
        let mut macro_def = Macro::new("Test".to_string(), "Test".to_string());
        assert_eq!(macro_def.execution_count, 0);

        macro_def.increment_execution_count();
        assert_eq!(macro_def.execution_count, 1);

        macro_def.increment_execution_count();
        assert_eq!(macro_def.execution_count, 2);
    }

    #[test]
    fn test_statistics() {
        let mut macro_def = Macro::new("Test".to_string(), "Test".to_string());
        macro_def.add_action(MacroAction::InsertText {
            text: "Hello".to_string(),
        }); // Modifying
        macro_def.add_action(MacroAction::MoveCursor { line: 1, column: 0 }); // Navigation
        macro_def.add_action(MacroAction::SelectAll); // Selection
        macro_def.add_action(MacroAction::Copy); // Selection

        let stats = macro_def.statistics();
        assert_eq!(stats.total_actions, 4);
        assert_eq!(stats.modifying_actions, 1);
        assert_eq!(stats.navigation_actions, 1);
        assert_eq!(stats.selection_actions, 2);
    }

    #[test]
    fn test_validation() {
        let mut macro_def = Macro::new("".to_string(), "Test".to_string());
        assert!(macro_def.validate().is_err());

        macro_def.name = "Test".to_string();
        assert!(macro_def.validate().is_err()); // No actions

        macro_def.add_action(MacroAction::Copy);
        assert!(macro_def.validate().is_ok());
    }

    #[test]
    fn test_duplicate() {
        let mut macro_def = Macro::new("Original".to_string(), "Test".to_string());
        macro_def.add_action(MacroAction::Copy);
        macro_def.set_shortcut(Some("Ctrl+M".to_string()));
        macro_def.increment_execution_count();

        let duplicate = macro_def.duplicate("Copy".to_string());
        assert_eq!(duplicate.name, "Copy");
        assert_eq!(duplicate.actions.len(), 1);
        assert_ne!(duplicate.id, macro_def.id);
        assert!(duplicate.shortcut.is_none()); // Shortcut not duplicated
        assert_eq!(duplicate.execution_count, 0); // Count reset
    }

    #[test]
    fn test_serialization() {
        let macro_def = Macro::with_actions(
            "Test".to_string(),
            "Test macro".to_string(),
            vec![
                MacroAction::InsertText {
                    text: "Hello".to_string(),
                },
                MacroAction::Copy,
            ],
        );

        let json = serde_json::to_string(&macro_def).unwrap();
        let deserialized: Macro = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, macro_def.name);
        assert_eq!(deserialized.actions.len(), macro_def.actions.len());
    }
}
