//! Command pattern for undo/redo functionality
//!
//! This module implements the Command pattern to enable undo/redo operations
//! for all text editing actions.

use crate::Result;
use ropey::Rope;

/// Trait representing an undoable/redoable command
pub trait Command: Send + Sync {
    /// Execute the command on the given rope
    fn execute(&self, rope: &mut Rope) -> Result<()>;

    /// Undo the command on the given rope
    fn undo(&self, rope: &mut Rope) -> Result<()>;

    /// Get a description of this command for debugging
    fn description(&self) -> String;

    /// Clone this command into a Box
    fn box_clone(&self) -> Box<dyn Command>;
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

/// Command to insert text at a position
#[derive(Debug, Clone)]
pub struct InsertCommand {
    /// The position where text was inserted
    position: usize,
    /// The text that was inserted
    text: String,
}

impl InsertCommand {
    /// Create a new insert command
    pub fn new(position: usize, text: String) -> Self {
        Self { position, text }
    }
}

impl Command for InsertCommand {
    fn execute(&self, rope: &mut Rope) -> Result<()> {
        rope.insert(self.position, &self.text);
        Ok(())
    }

    fn undo(&self, rope: &mut Rope) -> Result<()> {
        // Use char count, not byte length, since Rope uses char indices
        let end = self.position + self.text.chars().count();
        rope.remove(self.position..end);
        Ok(())
    }

    fn description(&self) -> String {
        format!("Insert '{}' at position {}",
            if self.text.len() > 20 {
                format!("{}...", &self.text[..20])
            } else {
                self.text.clone()
            },
            self.position
        )
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

/// Command to delete text from a range
#[derive(Debug, Clone)]
pub struct DeleteCommand {
    /// The start position of the deletion
    start: usize,
    /// The end position of the deletion
    end: usize,
    /// The deleted text (stored for undo)
    deleted_text: String,
}

impl DeleteCommand {
    /// Create a new delete command
    ///
    /// # Arguments
    ///
    /// * `start` - Start position of deletion
    /// * `end` - End position of deletion
    /// * `deleted_text` - The text being deleted (needed for undo)
    pub fn new(start: usize, end: usize, deleted_text: String) -> Self {
        Self {
            start,
            end,
            deleted_text,
        }
    }
}

impl Command for DeleteCommand {
    fn execute(&self, rope: &mut Rope) -> Result<()> {
        rope.remove(self.start..self.end);
        Ok(())
    }

    fn undo(&self, rope: &mut Rope) -> Result<()> {
        rope.insert(self.start, &self.deleted_text);
        Ok(())
    }

    fn description(&self) -> String {
        format!("Delete range {}..{} ('{}')",
            self.start,
            self.end,
            if self.deleted_text.len() > 20 {
                format!("{}...", &self.deleted_text[..20])
            } else {
                self.deleted_text.clone()
            }
        )
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

/// Command to replace text in a range
#[derive(Debug, Clone)]
pub struct ReplaceCommand {
    /// Start position of replacement
    start: usize,
    /// End position of replacement
    end: usize,
    /// The old text (for undo)
    old_text: String,
    /// The new text
    new_text: String,
}

impl ReplaceCommand {
    /// Create a new replace command
    pub fn new(start: usize, end: usize, old_text: String, new_text: String) -> Self {
        Self {
            start,
            end,
            old_text,
            new_text,
        }
    }
}

impl Command for ReplaceCommand {
    fn execute(&self, rope: &mut Rope) -> Result<()> {
        rope.remove(self.start..self.end);
        rope.insert(self.start, &self.new_text);
        Ok(())
    }

    fn undo(&self, rope: &mut Rope) -> Result<()> {
        // Use char count, not byte length, since Rope uses char indices
        let new_end = self.start + self.new_text.chars().count();
        rope.remove(self.start..new_end);
        rope.insert(self.start, &self.old_text);
        Ok(())
    }

    fn description(&self) -> String {
        format!("Replace '{}' with '{}' at {}..{}",
            if self.old_text.len() > 20 {
                format!("{}...", &self.old_text[..20])
            } else {
                self.old_text.clone()
            },
            if self.new_text.len() > 20 {
                format!("{}...", &self.new_text[..20])
            } else {
                self.new_text.clone()
            },
            self.start,
            self.end
        )
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

/// Manages a stack of commands for undo/redo functionality
pub struct UndoStack {
    /// Stack of executed commands (for undo)
    undo_stack: Vec<Box<dyn Command>>,
    /// Stack of undone commands (for redo)
    redo_stack: Vec<Box<dyn Command>>,
    /// Maximum number of undo levels (0 = unlimited)
    max_undo_levels: usize,
}

impl UndoStack {
    /// Create a new undo stack with unlimited undo levels
    pub fn new() -> Self {
        Self::with_limit(0)
    }

    /// Create a new undo stack with a maximum number of undo levels
    ///
    /// # Arguments
    ///
    /// * `max_levels` - Maximum undo levels (0 for unlimited)
    pub fn with_limit(max_levels: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_undo_levels: max_levels,
        }
    }

    /// Push a command onto the undo stack
    ///
    /// This clears the redo stack and adds the command to the undo stack.
    pub fn push(&mut self, command: Box<dyn Command>) {
        // Clear redo stack when a new command is executed
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(command);

        // Enforce max undo levels
        if self.max_undo_levels > 0 && self.undo_stack.len() > self.max_undo_levels {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last command
    ///
    /// Returns the command that was undone, or None if nothing to undo
    pub fn undo(&mut self) -> Option<Box<dyn Command>> {
        if let Some(command) = self.undo_stack.pop() {
            let cloned = command.box_clone();
            self.redo_stack.push(command);
            Some(cloned)
        } else {
            None
        }
    }

    /// Redo the last undone command
    ///
    /// Returns the command that was redone, or None if nothing to redo
    pub fn redo(&mut self) -> Option<Box<dyn Command>> {
        if let Some(command) = self.redo_stack.pop() {
            let cloned = command.box_clone();
            self.undo_stack.push(command);
            Some(cloned)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all undo/redo history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of commands in the undo stack
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of commands in the redo stack
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Get description of the next command to undo
    pub fn peek_undo(&self) -> Option<String> {
        self.undo_stack.last().map(|cmd| cmd.description())
    }

    /// Get description of the next command to redo
    pub fn peek_redo(&self) -> Option<String> {
        self.redo_stack.last().map(|cmd| cmd.description())
    }
}

impl Default for UndoStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_command() {
        let mut rope = Rope::new();
        let cmd = InsertCommand::new(0, "Hello".to_string());

        // Execute
        cmd.execute(&mut rope).unwrap();
        assert_eq!(rope.to_string(), "Hello");

        // Undo
        cmd.undo(&mut rope).unwrap();
        assert_eq!(rope.to_string(), "");
    }

    #[test]
    fn test_delete_command() {
        let mut rope = Rope::from_str("Hello World");
        let cmd = DeleteCommand::new(5, 11, " World".to_string());

        // Execute
        cmd.execute(&mut rope).unwrap();
        assert_eq!(rope.to_string(), "Hello");

        // Undo
        cmd.undo(&mut rope).unwrap();
        assert_eq!(rope.to_string(), "Hello World");
    }

    #[test]
    fn test_replace_command() {
        let mut rope = Rope::from_str("Hello World");
        let cmd = ReplaceCommand::new(6, 11, "World".to_string(), "Rust".to_string());

        // Execute
        cmd.execute(&mut rope).unwrap();
        assert_eq!(rope.to_string(), "Hello Rust");

        // Undo
        cmd.undo(&mut rope).unwrap();
        assert_eq!(rope.to_string(), "Hello World");
    }

    #[test]
    fn test_undo_stack_basic() {
        let mut stack = UndoStack::new();

        // Initially empty
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());

        // Push a command
        let cmd = Box::new(InsertCommand::new(0, "Hello".to_string()));
        stack.push(cmd);

        assert!(stack.can_undo());
        assert!(!stack.can_redo());
        assert_eq!(stack.undo_count(), 1);
    }

    #[test]
    fn test_undo_redo() {
        let mut stack = UndoStack::new();
        let mut rope = Rope::new();

        // Execute and push command
        let cmd1 = InsertCommand::new(0, "Hello".to_string());
        cmd1.execute(&mut rope).unwrap();
        stack.push(Box::new(cmd1));
        assert_eq!(rope.to_string(), "Hello");

        // Undo
        if let Some(cmd) = stack.undo() {
            cmd.undo(&mut rope).unwrap();
        }
        assert_eq!(rope.to_string(), "");
        assert!(stack.can_redo());

        // Redo
        if let Some(cmd) = stack.redo() {
            cmd.execute(&mut rope).unwrap();
        }
        assert_eq!(rope.to_string(), "Hello");
    }

    #[test]
    fn test_multiple_undo_redo() {
        let mut stack = UndoStack::new();
        let mut rope = Rope::new();

        // Execute multiple commands
        let cmd1 = InsertCommand::new(0, "Hello".to_string());
        cmd1.execute(&mut rope).unwrap();
        stack.push(Box::new(cmd1));

        let cmd2 = InsertCommand::new(5, " World".to_string());
        cmd2.execute(&mut rope).unwrap();
        stack.push(Box::new(cmd2));

        assert_eq!(rope.to_string(), "Hello World");

        // Undo both
        if let Some(cmd) = stack.undo() {
            cmd.undo(&mut rope).unwrap();
        }
        assert_eq!(rope.to_string(), "Hello");

        if let Some(cmd) = stack.undo() {
            cmd.undo(&mut rope).unwrap();
        }
        assert_eq!(rope.to_string(), "");

        // Redo both
        if let Some(cmd) = stack.redo() {
            cmd.execute(&mut rope).unwrap();
        }
        assert_eq!(rope.to_string(), "Hello");

        if let Some(cmd) = stack.redo() {
            cmd.execute(&mut rope).unwrap();
        }
        assert_eq!(rope.to_string(), "Hello World");
    }

    #[test]
    fn test_max_undo_levels() {
        let mut stack = UndoStack::with_limit(2);

        // Push 3 commands
        stack.push(Box::new(InsertCommand::new(0, "1".to_string())));
        stack.push(Box::new(InsertCommand::new(1, "2".to_string())));
        stack.push(Box::new(InsertCommand::new(2, "3".to_string())));

        // Should only have 2 commands (oldest removed)
        assert_eq!(stack.undo_count(), 2);
    }

    #[test]
    fn test_clear_redo_on_new_command() {
        let mut stack = UndoStack::new();
        let mut rope = Rope::new();

        // Execute commands
        let cmd1 = InsertCommand::new(0, "Hello".to_string());
        cmd1.execute(&mut rope).unwrap();
        stack.push(Box::new(cmd1));

        // Undo
        if let Some(cmd) = stack.undo() {
            cmd.undo(&mut rope).unwrap();
        }
        assert!(stack.can_redo());

        // Execute new command - should clear redo stack
        let cmd2 = InsertCommand::new(0, "Goodbye".to_string());
        cmd2.execute(&mut rope).unwrap();
        stack.push(Box::new(cmd2));

        assert!(!stack.can_redo());
        assert_eq!(stack.redo_count(), 0);
    }

    #[test]
    fn test_peek_descriptions() {
        let mut stack = UndoStack::new();

        stack.push(Box::new(InsertCommand::new(0, "Hello".to_string())));

        let desc = stack.peek_undo();
        assert!(desc.is_some());
        let desc_str = desc.unwrap();
        assert!(desc_str.contains("Insert"));
        assert!(desc_str.contains("Hello"));
    }
}
