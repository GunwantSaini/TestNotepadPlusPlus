//! Macro action types and definitions

use serde::{Deserialize, Serialize};

/// Represents a single recordable action in a macro
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MacroAction {
    /// Insert text at cursor
    InsertText { text: String },

    /// Delete text
    Delete {
        /// Number of characters to delete (negative for backspace)
        count: i32,
    },

    /// Move cursor
    MoveCursor {
        /// Line number (absolute)
        line: usize,
        /// Column number (absolute)
        column: usize,
    },

    /// Move cursor relative to current position
    MoveCursorRelative {
        /// Line delta (can be negative)
        line_delta: i32,
        /// Column delta (can be negative)
        column_delta: i32,
    },

    /// Select text
    Select {
        /// Start position (line, column)
        start: (usize, usize),
        /// End position (line, column)
        end: (usize, usize),
    },

    /// Select all text
    SelectAll,

    /// Cut selected text
    Cut,

    /// Copy selected text
    Copy,

    /// Paste from clipboard
    Paste,

    /// Undo last action
    Undo,

    /// Redo last undone action
    Redo,

    /// Find text
    Find {
        /// Text to find
        text: String,
        /// Case sensitive
        case_sensitive: bool,
        /// Whole word
        whole_word: bool,
        /// Use regex
        regex: bool,
    },

    /// Replace text
    Replace {
        /// Text to find
        find: String,
        /// Replacement text
        replace: String,
        /// Replace all occurrences
        all: bool,
    },

    /// Go to line
    GoToLine { line: usize },

    /// Insert new line
    NewLine,

    /// Insert tab
    Tab,

    /// Indent selection
    Indent,

    /// Unindent selection
    Unindent,

    /// Comment/uncomment lines
    ToggleComment,

    /// Convert to uppercase
    ToUpperCase,

    /// Convert to lowercase
    ToLowerCase,

    /// Duplicate line/selection
    Duplicate,

    /// Delete line
    DeleteLine,

    /// Sort lines ascending
    SortAscending,

    /// Sort lines descending
    SortDescending,

    /// Remove duplicate lines
    RemoveDuplicates,

    /// Trim trailing whitespace
    TrimTrailing,

    /// Custom action with arbitrary data
    Custom {
        /// Action name
        name: String,
        /// Action data (JSON)
        data: serde_json::Value,
    },

    /// Delay/pause
    Delay {
        /// Delay in milliseconds
        milliseconds: u64,
    },
}

impl MacroAction {
    /// Check if this action modifies the document
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            MacroAction::InsertText { .. }
                | MacroAction::Delete { .. }
                | MacroAction::Cut
                | MacroAction::Paste
                | MacroAction::Undo
                | MacroAction::Redo
                | MacroAction::Replace { .. }
                | MacroAction::NewLine
                | MacroAction::Tab
                | MacroAction::Indent
                | MacroAction::Unindent
                | MacroAction::ToggleComment
                | MacroAction::ToUpperCase
                | MacroAction::ToLowerCase
                | MacroAction::Duplicate
                | MacroAction::DeleteLine
                | MacroAction::SortAscending
                | MacroAction::SortDescending
                | MacroAction::RemoveDuplicates
                | MacroAction::TrimTrailing
        )
    }

    /// Check if this action changes cursor position
    pub fn is_navigation(&self) -> bool {
        matches!(
            self,
            MacroAction::MoveCursor { .. }
                | MacroAction::MoveCursorRelative { .. }
                | MacroAction::GoToLine { .. }
        )
    }

    /// Check if this action involves selection
    pub fn is_selection(&self) -> bool {
        matches!(
            self,
            MacroAction::Select { .. }
                | MacroAction::SelectAll
                | MacroAction::Cut
                | MacroAction::Copy
        )
    }

    /// Get a human-readable description of this action
    pub fn description(&self) -> String {
        match self {
            MacroAction::InsertText { text } => {
                if text.len() > 20 {
                    format!("Insert \"{}...\"", &text[..20])
                } else {
                    format!("Insert \"{}\"", text)
                }
            }
            MacroAction::Delete { count } => {
                if *count < 0 {
                    format!("Backspace {} characters", count.abs())
                } else {
                    format!("Delete {} characters", count)
                }
            }
            MacroAction::MoveCursor { line, column } => {
                format!("Move cursor to {}:{}", line, column)
            }
            MacroAction::MoveCursorRelative {
                line_delta,
                column_delta,
            } => {
                format!("Move cursor ({:+}, {:+})", line_delta, column_delta)
            }
            MacroAction::Select { start, end } => {
                format!("Select from {:?} to {:?}", start, end)
            }
            MacroAction::SelectAll => "Select All".to_string(),
            MacroAction::Cut => "Cut".to_string(),
            MacroAction::Copy => "Copy".to_string(),
            MacroAction::Paste => "Paste".to_string(),
            MacroAction::Undo => "Undo".to_string(),
            MacroAction::Redo => "Redo".to_string(),
            MacroAction::Find {
                text,
                case_sensitive,
                whole_word,
                regex,
            } => {
                let mut flags = Vec::new();
                if *case_sensitive {
                    flags.push("case");
                }
                if *whole_word {
                    flags.push("word");
                }
                if *regex {
                    flags.push("regex");
                }
                let flags_str = if flags.is_empty() {
                    String::new()
                } else {
                    format!(" ({})", flags.join(", "))
                };
                format!("Find \"{}\"{}",  text, flags_str)
            }
            MacroAction::Replace { find, replace, all } => {
                if *all {
                    format!("Replace all \"{}\" with \"{}\"", find, replace)
                } else {
                    format!("Replace \"{}\" with \"{}\"", find, replace)
                }
            }
            MacroAction::GoToLine { line } => format!("Go to line {}", line),
            MacroAction::NewLine => "New Line".to_string(),
            MacroAction::Tab => "Tab".to_string(),
            MacroAction::Indent => "Indent".to_string(),
            MacroAction::Unindent => "Unindent".to_string(),
            MacroAction::ToggleComment => "Toggle Comment".to_string(),
            MacroAction::ToUpperCase => "To Uppercase".to_string(),
            MacroAction::ToLowerCase => "To Lowercase".to_string(),
            MacroAction::Duplicate => "Duplicate".to_string(),
            MacroAction::DeleteLine => "Delete Line".to_string(),
            MacroAction::SortAscending => "Sort Ascending".to_string(),
            MacroAction::SortDescending => "Sort Descending".to_string(),
            MacroAction::RemoveDuplicates => "Remove Duplicates".to_string(),
            MacroAction::TrimTrailing => "Trim Trailing Whitespace".to_string(),
            MacroAction::Custom { name, .. } => format!("Custom: {}", name),
            MacroAction::Delay { milliseconds } => format!("Delay {}ms", milliseconds),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_is_modifying() {
        assert!(MacroAction::InsertText {
            text: "test".to_string()
        }
        .is_modifying());
        assert!(MacroAction::Delete { count: 1 }.is_modifying());
        assert!(MacroAction::Cut.is_modifying());
        assert!(!MacroAction::Copy.is_modifying());
        assert!(!MacroAction::MoveCursor { line: 1, column: 1 }.is_modifying());
    }

    #[test]
    fn test_action_is_navigation() {
        assert!(MacroAction::MoveCursor { line: 1, column: 1 }.is_navigation());
        assert!(MacroAction::MoveCursorRelative {
            line_delta: 1,
            column_delta: 0
        }
        .is_navigation());
        assert!(MacroAction::GoToLine { line: 10 }.is_navigation());
        assert!(!MacroAction::Copy.is_navigation());
    }

    #[test]
    fn test_action_is_selection() {
        assert!(MacroAction::SelectAll.is_selection());
        assert!(MacroAction::Select {
            start: (0, 0),
            end: (1, 0)
        }
        .is_selection());
        assert!(MacroAction::Cut.is_selection());
        assert!(!MacroAction::Paste.is_selection());
    }

    #[test]
    fn test_action_description() {
        let action = MacroAction::InsertText {
            text: "Hello".to_string(),
        };
        assert_eq!(action.description(), "Insert \"Hello\"");

        let action = MacroAction::Delete { count: 5 };
        assert_eq!(action.description(), "Delete 5 characters");

        let action = MacroAction::Find {
            text: "test".to_string(),
            case_sensitive: true,
            whole_word: false,
            regex: false,
        };
        assert_eq!(action.description(), "Find \"test\" (case)");
    }

    #[test]
    fn test_action_serialization() {
        let action = MacroAction::InsertText {
            text: "Hello".to_string(),
        };
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: MacroAction = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }

    #[test]
    fn test_custom_action() {
        let data = serde_json::json!({"key": "value"});
        let action = MacroAction::Custom {
            name: "MyAction".to_string(),
            data,
        };
        assert_eq!(action.description(), "Custom: MyAction");
    }
}
