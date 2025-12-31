//! Simple editor control wrapper
//!
//! This module provides a basic text editor control using Win32 EDIT control.
//! It can be replaced with Scintilla integration when SciLexer.DLL is available.

use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Controls::{EM_CANUNDO, EM_GETLINECOUNT, EM_SETSEL, EM_UNDO};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, WM_COPY, WM_CUT, WM_PASTE, WM_SETTEXT, WINDOW_STYLE, WS_CHILD, WS_HSCROLL,
    WS_VISIBLE, WS_VSCROLL,
};

/// ES_MULTILINE edit control style (multiline editing)
const ES_MULTILINE: u32 = 0x0004;

/// Simple editor control using Win32 EDIT control
pub struct EditorControl {
    hwnd: HWND,
}

impl EditorControl {
    /// Create a new editor control
    pub fn new(parent: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<Self, String> {
        unsafe {
            let hwnd = CreateWindowExW(
                Default::default(),
                w!("EDIT"),
                w!(""),
                WS_CHILD | WS_VISIBLE | WS_VSCROLL | WS_HSCROLL | WINDOW_STYLE(ES_MULTILINE),
                x,
                y,
                width,
                height,
                parent,
                None,
                None,
                None,
            );

            if hwnd.0 == 0 {
                return Err("Failed to create editor control".to_string());
            }

            Ok(Self { hwnd })
        }
    }

    /// Get the window handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    /// Set text content
    pub fn set_text(&self, text: &str) {
        let text_wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                WM_SETTEXT,
                WPARAM(0),
                LPARAM(text_wide.as_ptr() as isize),
            );
        }
    }

    /// Undo last action
    pub fn undo(&self) {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                EM_UNDO,
                WPARAM(0),
                LPARAM(0),
            );
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                EM_CANUNDO,
                WPARAM(0),
                LPARAM(0),
            )
            .0
                != 0
        }
    }

    /// Cut selected text
    pub fn cut(&self) {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                WM_CUT,
                WPARAM(0),
                LPARAM(0),
            );
        }
    }

    /// Copy selected text
    pub fn copy(&self) {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                WM_COPY,
                WPARAM(0),
                LPARAM(0),
            );
        }
    }

    /// Paste from clipboard
    pub fn paste(&self) {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                WM_PASTE,
                WPARAM(0),
                LPARAM(0),
            );
        }
    }

    /// Get line count
    pub fn get_line_count(&self) -> usize {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                EM_GETLINECOUNT,
                WPARAM(0),
                LPARAM(0),
            )
            .0 as usize
        }
    }

    /// Resize the control
    pub fn resize(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
                self.hwnd,
                None,
                x,
                y,
                width,
                height,
                windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER,
            )
            .ok();
        }
    }

    /// Select all text
    pub fn select_all(&self) {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                EM_SETSEL,
                WPARAM(0),
                LPARAM(-1),
            );
        }
    }
}
