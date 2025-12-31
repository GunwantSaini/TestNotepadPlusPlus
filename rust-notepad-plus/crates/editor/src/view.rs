//! Editor view widget

use crate::{TextBuffer, Result};
use windows::Win32::Foundation::HWND;

pub struct EditorView {
    hwnd: HWND,
    buffer: TextBuffer,
}

impl EditorView {
    pub fn new(parent: HWND) -> Result<Self> {
        // TODO: Create Win32 window for editor
        Ok(Self {
            hwnd: HWND(0),
            buffer: TextBuffer::new(),
        })
    }

    pub fn set_text(&mut self, text: String) {
        self.buffer = TextBuffer::from_string(text);
    }

    pub fn get_text(&self) -> String {
        self.buffer.get_text()
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
}
