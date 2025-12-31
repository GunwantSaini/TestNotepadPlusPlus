//! Editor view widget

use crate::TextBuffer;

pub struct EditorView {
    buffer: TextBuffer,
}

impl EditorView {
    pub fn new() -> Self {
        Self {
            buffer: TextBuffer::new(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.buffer = TextBuffer::from_string(text);
    }

    pub fn get_text(&self) -> String {
        self.buffer.get_text()
    }

    pub fn buffer(&self) -> &TextBuffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.buffer
    }
}
