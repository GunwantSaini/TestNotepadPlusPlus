//! Text buffer using Ropey

use ropey::Rope;
use crate::Result;
use crate::command::{Command, InsertCommand, DeleteCommand, ReplaceCommand, UndoStack};

pub struct TextBuffer {
    rope: Rope,
    modified: bool,
    undo_stack: UndoStack,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            modified: false,
            undo_stack: UndoStack::new(),
        }
    }

    pub fn from_string(s: String) -> Self {
        Self {
            rope: Rope::from_str(&s),
            modified: false,
            undo_stack: UndoStack::new(),
        }
    }

    pub fn get_text(&self) -> String {
        self.rope.to_string()
    }

    pub fn insert(&mut self, pos: usize, text: &str) -> Result<()> {
        let command = InsertCommand::new(pos, text.to_string());
        command.execute(&mut self.rope)?;
        self.undo_stack.push(Box::new(command));
        self.modified = true;
        Ok(())
    }

    pub fn delete(&mut self, start: usize, end: usize) -> Result<()> {
        // Extract the text being deleted for undo
        let deleted_text = self.rope.slice(start..end).to_string();
        let command = DeleteCommand::new(start, end, deleted_text);
        command.execute(&mut self.rope)?;
        self.undo_stack.push(Box::new(command));
        self.modified = true;
        Ok(())
    }

    pub fn replace(&mut self, start: usize, end: usize, new_text: &str) -> Result<()> {
        let old_text = self.rope.slice(start..end).to_string();
        let command = ReplaceCommand::new(start, end, old_text, new_text.to_string());
        command.execute(&mut self.rope)?;
        self.undo_stack.push(Box::new(command));
        self.modified = true;
        Ok(())
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn clear_modified(&mut self) {
        self.modified = false;
    }

    pub fn len(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn get_line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.line_count() {
            Some(self.rope.line(line_idx).to_string())
        } else {
            None
        }
    }

    /// Undo the last operation
    pub fn undo(&mut self) -> Result<()> {
        if let Some(command) = self.undo_stack.undo() {
            command.undo(&mut self.rope)?;
            self.modified = true;
        }
        Ok(())
    }

    /// Redo the last undone operation
    pub fn redo(&mut self) -> Result<()> {
        if let Some(command) = self.undo_stack.redo() {
            command.execute(&mut self.rope)?;
            self.modified = true;
        }
        Ok(())
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.undo_stack.can_undo()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.undo_stack.can_redo()
    }

    /// Clear undo/redo history
    pub fn clear_undo_history(&mut self) {
        self.undo_stack.clear();
    }

    /// Get description of next undo operation
    pub fn peek_undo(&self) -> Option<String> {
        self.undo_stack.peek_undo()
    }

    /// Get description of next redo operation
    pub fn peek_redo(&self) -> Option<String> {
        self.undo_stack.peek_redo()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_buffer() {
        let mut buffer = TextBuffer::new();
        assert_eq!(buffer.len(), 0);

        buffer.insert(0, "Hello").unwrap();
        assert_eq!(buffer.get_text(), "Hello");
        assert!(buffer.is_modified());

        buffer.insert(5, " World").unwrap();
        assert_eq!(buffer.get_text(), "Hello World");
    }

    #[test]
    fn test_undo_insert() {
        let mut buffer = TextBuffer::new();

        // Insert text
        buffer.insert(0, "Hello").unwrap();
        assert_eq!(buffer.get_text(), "Hello");
        assert!(buffer.can_undo());
        assert!(!buffer.can_redo());

        // Undo
        buffer.undo().unwrap();
        assert_eq!(buffer.get_text(), "");
        assert!(!buffer.can_undo());
        assert!(buffer.can_redo());
    }

    #[test]
    fn test_redo_insert() {
        let mut buffer = TextBuffer::new();

        // Insert and undo
        buffer.insert(0, "Hello").unwrap();
        buffer.undo().unwrap();
        assert_eq!(buffer.get_text(), "");

        // Redo
        buffer.redo().unwrap();
        assert_eq!(buffer.get_text(), "Hello");
        assert!(buffer.can_undo());
        assert!(!buffer.can_redo());
    }

    #[test]
    fn test_undo_delete() {
        let mut buffer = TextBuffer::from_string("Hello World".to_string());
        buffer.clear_undo_history(); // Clear history from initial load

        // Delete text
        buffer.delete(5, 11).unwrap();
        assert_eq!(buffer.get_text(), "Hello");
        assert!(buffer.can_undo());

        // Undo delete
        buffer.undo().unwrap();
        assert_eq!(buffer.get_text(), "Hello World");
    }

    #[test]
    fn test_multiple_undo_redo() {
        let mut buffer = TextBuffer::new();

        // Multiple operations
        buffer.insert(0, "Hello").unwrap();
        buffer.insert(5, " World").unwrap();
        buffer.insert(11, "!").unwrap();
        assert_eq!(buffer.get_text(), "Hello World!");

        // Undo all
        buffer.undo().unwrap();
        assert_eq!(buffer.get_text(), "Hello World");

        buffer.undo().unwrap();
        assert_eq!(buffer.get_text(), "Hello");

        buffer.undo().unwrap();
        assert_eq!(buffer.get_text(), "");

        // Redo all
        buffer.redo().unwrap();
        assert_eq!(buffer.get_text(), "Hello");

        buffer.redo().unwrap();
        assert_eq!(buffer.get_text(), "Hello World");

        buffer.redo().unwrap();
        assert_eq!(buffer.get_text(), "Hello World!");
    }

    #[test]
    fn test_replace_operation() {
        let mut buffer = TextBuffer::from_string("Hello World".to_string());
        buffer.clear_undo_history();

        // Replace "World" with "Rust"
        buffer.replace(6, 11, "Rust").unwrap();
        assert_eq!(buffer.get_text(), "Hello Rust");

        // Undo replace
        buffer.undo().unwrap();
        assert_eq!(buffer.get_text(), "Hello World");

        // Redo replace
        buffer.redo().unwrap();
        assert_eq!(buffer.get_text(), "Hello Rust");
    }

    #[test]
    fn test_undo_clears_redo() {
        let mut buffer = TextBuffer::new();

        // Insert, undo, then insert again
        buffer.insert(0, "Hello").unwrap();
        buffer.undo().unwrap();
        assert!(buffer.can_redo());

        // New operation should clear redo stack
        buffer.insert(0, "Goodbye").unwrap();
        assert!(!buffer.can_redo());
    }

    #[test]
    fn test_peek_undo_redo() {
        let mut buffer = TextBuffer::new();

        buffer.insert(0, "Hello").unwrap();

        let undo_desc = buffer.peek_undo();
        assert!(undo_desc.is_some());
        assert!(undo_desc.unwrap().contains("Insert"));

        buffer.undo().unwrap();

        let redo_desc = buffer.peek_redo();
        assert!(redo_desc.is_some());
        assert!(redo_desc.unwrap().contains("Insert"));
    }

    #[test]
    fn test_clear_undo_history() {
        let mut buffer = TextBuffer::new();

        buffer.insert(0, "Hello").unwrap();
        assert!(buffer.can_undo());

        buffer.clear_undo_history();
        assert!(!buffer.can_undo());
        assert!(!buffer.can_redo());
    }

    #[test]
    fn test_complex_editing_workflow() {
        let mut buffer = TextBuffer::new();

        // Simulate realistic editing
        buffer.insert(0, "fn main() {\n").unwrap();
        buffer.insert(12, "    println!(\"Hello\");\n").unwrap();
        buffer.insert(35, "}\n").unwrap();

        let code = buffer.get_text();
        assert!(code.contains("fn main()"));
        assert!(code.contains("println!"));

        // Undo last brace
        buffer.undo().unwrap();
        assert!(!buffer.get_text().contains("}"));

        // Redo
        buffer.redo().unwrap();
        assert!(buffer.get_text().contains("}"));

        // Delete middle line
        let text = buffer.get_text();
        let start_idx = text.find("    println").unwrap();
        let end_idx = start_idx + 23;
        buffer.delete(start_idx, end_idx).unwrap();

        // Verify deletion
        let text = buffer.get_text();
        assert!(!text.contains("println"));

        // Undo deletion
        buffer.undo().unwrap();
        assert!(buffer.get_text().contains("println"));
    }

    #[test]
    fn test_unicode_undo() {
        let mut buffer = TextBuffer::new();

        // Insert unicode
        buffer.insert(0, "Hello 世界 🦀").unwrap();
        assert_eq!(buffer.get_text(), "Hello 世界 🦀");

        // Undo
        buffer.undo().unwrap();
        assert_eq!(buffer.get_text(), "");

        // Redo
        buffer.redo().unwrap();
        assert_eq!(buffer.get_text(), "Hello 世界 🦀");
    }

    #[test]
    fn test_large_text_undo() {
        let mut buffer = TextBuffer::new();

        // Insert large text
        let large_text = "A".repeat(10000);
        buffer.insert(0, &large_text).unwrap();
        assert_eq!(buffer.len(), 10000);

        // Undo
        buffer.undo().unwrap();
        assert_eq!(buffer.len(), 0);

        // Redo
        buffer.redo().unwrap();
        assert_eq!(buffer.len(), 10000);
    }
}
