//! Text buffer using Ropey

use ropey::Rope;
use crate::Result;

pub struct TextBuffer {
    rope: Rope,
    modified: bool,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            modified: false,
        }
    }

    pub fn from_string(s: String) -> Self {
        Self {
            rope: Rope::from_str(&s),
            modified: false,
        }
    }

    pub fn get_text(&self) -> String {
        self.rope.to_string()
    }

    pub fn insert(&mut self, pos: usize, text: &str) -> Result<()> {
        self.rope.insert(pos, text);
        self.modified = true;
        Ok(())
    }

    pub fn delete(&mut self, start: usize, end: usize) -> Result<()> {
        self.rope.remove(start..end);
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
}
