//! Scintilla editor control FFI bindings and wrapper
//!
//! Scintilla is a powerful text editing component used by Notepad++.
//! This module provides Rust bindings for embedding Scintilla in our UI.

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, WS_CHILD, WS_VISIBLE};

/// Scintilla window class name
pub const SCINTILLA_CLASS: &str = "Scintilla";

/// Common Scintilla messages
/// These are based on the Scintilla API documentation
#[repr(u32)]
#[allow(dead_code)]
pub enum SciMsg {
    // Document text retrieval and modification
    GetText = 2182,
    SetText = 2181,
    GetLength = 2183,
    ClearAll = 2004,

    // Selection and positioning
    GetCurrentPos = 2008,
    SetCurrentPos = 2141,
    GetAnchor = 2009,
    SetSel = 2160,
    SelectAll = 2013,

    // Line operations
    GetLineCount = 2154,
    GetLine = 2153,
    LineLength = 2350,
    GotoLine = 2024,

    // Undo/Redo
    Undo = 2176,
    Redo = 2011,
    CanUndo = 2174,
    CanRedo = 2016,
    EmptyUndoBuffer = 2175,

    // Cut/Copy/Paste
    Cut = 2177,
    Copy = 2178,
    Paste = 2179,
    Clear = 2180,

    // Search and replace
    SearchAnchor = 2366,
    SearchNext = 2367,
    SearchPrev = 2368,

    // Styling and appearance
    StyleSetFont = 2056,
    StyleSetSize = 2055,
    StyleSetFore = 2051,
    StyleSetBack = 2052,
    StyleSetBold = 2053,
    StyleSetItalic = 2054,

    // Margins
    SetMarginWidthN = 2242,
    SetMarginTypeN = 2240,

    // Folding
    SetFoldFlags = 2233,

    // Lexer (syntax highlighting)
    SetLexer = 4001,
    SetLexerLanguage = 4006,
    Colorize = 4003,

    // Configuration
    SetEOLMode = 2031,
    GetEOLMode = 2030,
    SetTabWidth = 2036,
    SetUseTabs = 2124,
    SetIndent = 2122,

    // Direct pointer access (advanced)
    GetDirectPointer = 2185,
}

/// EOL (End of Line) modes
#[repr(i32)]
#[allow(dead_code)]
pub enum EolMode {
    Crlf = 0,  // Windows (CR+LF)
    Cr = 1,    // Mac Classic (CR)
    Lf = 2,    // Unix/Linux/Mac (LF)
}

/// Scintilla control wrapper
pub struct ScintillaControl {
    hwnd: HWND,
}

impl ScintillaControl {
    /// Create a new Scintilla control
    ///
    /// Note: This requires the Scintilla DLL to be loaded and the window class registered.
    /// In a real application, you would load SciLexer.DLL first.
    pub fn new(parent: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<Self, String> {
        unsafe {
            // Convert class name to wide string
            let class_name: Vec<u16> = SCINTILLA_CLASS
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            let hwnd = CreateWindowExW(
                Default::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR::null(),
                WS_CHILD | WS_VISIBLE,
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
                return Err("Failed to create Scintilla control".to_string());
            }

            let control = Self { hwnd };

            // Initialize with basic settings
            control.init_default_settings();

            Ok(control)
        }
    }

    /// Initialize default Scintilla settings
    fn init_default_settings(&self) {
        // Set tab width to 4
        self.send_message(SciMsg::SetTabWidth, WPARAM(4), LPARAM(0));

        // Use spaces instead of tabs
        self.send_message(SciMsg::SetUseTabs, WPARAM(0), LPARAM(0));

        // Set indent to 4
        self.send_message(SciMsg::SetIndent, WPARAM(4), LPARAM(0));

        // Set EOL mode to CRLF (Windows)
        self.send_message(
            SciMsg::SetEOLMode,
            WPARAM(EolMode::Crlf as usize),
            LPARAM(0),
        );

        // Set line number margin width
        self.send_message(SciMsg::SetMarginWidthN, WPARAM(0), LPARAM(40));
    }

    /// Send a message to the Scintilla control
    pub fn send_message(&self, msg: SciMsg, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                msg as u32,
                wparam,
                lparam,
            )
        }
    }

    /// Get the window handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    /// Set the text content
    pub fn set_text(&self, text: &str) {
        let text_ptr = text.as_ptr() as isize;
        self.send_message(SciMsg::SetText, WPARAM(0), LPARAM(text_ptr));
    }

    /// Get the text content
    pub fn get_text(&self) -> String {
        let length = self.send_message(SciMsg::GetLength, WPARAM(0), LPARAM(0)).0 as usize;

        if length == 0 {
            return String::new();
        }

        let mut buffer = vec![0u8; length + 1];
        self.send_message(
            SciMsg::GetText,
            WPARAM(buffer.len()),
            LPARAM(buffer.as_mut_ptr() as isize),
        );

        String::from_utf8_lossy(&buffer[..length]).to_string()
    }

    /// Clear all text
    pub fn clear_all(&self) {
        self.send_message(SciMsg::ClearAll, WPARAM(0), LPARAM(0));
    }

    /// Select all text
    pub fn select_all(&self) {
        self.send_message(SciMsg::SelectAll, WPARAM(0), LPARAM(0));
    }

    /// Undo last action
    pub fn undo(&self) {
        self.send_message(SciMsg::Undo, WPARAM(0), LPARAM(0));
    }

    /// Redo last undone action
    pub fn redo(&self) {
        self.send_message(SciMsg::Redo, WPARAM(0), LPARAM(0));
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.send_message(SciMsg::CanUndo, WPARAM(0), LPARAM(0)).0 != 0
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.send_message(SciMsg::CanRedo, WPARAM(0), LPARAM(0)).0 != 0
    }

    /// Cut selected text
    pub fn cut(&self) {
        self.send_message(SciMsg::Cut, WPARAM(0), LPARAM(0));
    }

    /// Copy selected text
    pub fn copy(&self) {
        self.send_message(SciMsg::Copy, WPARAM(0), LPARAM(0));
    }

    /// Paste from clipboard
    pub fn paste(&self) {
        self.send_message(SciMsg::Paste, WPARAM(0), LPARAM(0));
    }

    /// Get current line number (0-based)
    pub fn get_current_line(&self) -> usize {
        let pos = self.send_message(SciMsg::GetCurrentPos, WPARAM(0), LPARAM(0)).0 as usize;
        // Would need LineFromPosition message - simplified for now
        pos / 80 // Rough estimate
    }

    /// Get line count
    pub fn get_line_count(&self) -> usize {
        self.send_message(SciMsg::GetLineCount, WPARAM(0), LPARAM(0)).0 as usize
    }

    /// Go to specific line
    pub fn goto_line(&self, line: usize) {
        self.send_message(SciMsg::GotoLine, WPARAM(line), LPARAM(0));
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
}

/// Note: In a real implementation, you would need to:
/// 1. Load SciLexer.DLL at startup using LoadLibrary
/// 2. The DLL self-registers the "Scintilla" window class
/// 3. Handle WM_NOTIFY messages from Scintilla for notifications
///
/// For now, this provides the FFI structure. The actual Scintilla DLL
/// would need to be included in the distribution.
