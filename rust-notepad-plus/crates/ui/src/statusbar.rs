//! Status bar implementation for displaying file and editor information

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Controls::{STATUSCLASSNAMEW, SB_SETPARTS, SB_SETTEXT};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, WS_CHILD, WS_VISIBLE};

const STATUSBAR_HEIGHT: i32 = 24;

/// Status bar part indices
#[derive(Debug, Clone, Copy)]
pub enum StatusPart {
    FileInfo = 0,    // File path and modification status
    Position = 1,    // Line and column position
    Encoding = 2,    // File encoding
    LineEnding = 3,  // Line ending type (CRLF/LF)
    FileType = 4,    // File type/language
}

/// Status bar control wrapper
pub struct StatusBar {
    hwnd: HWND,
}

impl StatusBar {
    /// Create a new status bar
    pub fn new(parent: HWND, _h_instance: isize) -> Result<Self, windows::core::Error> {
        unsafe {
            // Create status bar window
            let hwnd = CreateWindowExW(
                Default::default(),
                STATUSCLASSNAMEW,
                PCWSTR::null(),
                WS_CHILD | WS_VISIBLE,
                0,
                0,
                0,
                0,
                parent,
                None,
                None,
                None,
            );

            if hwnd.0 == 0 {
                return Err(windows::core::Error::from_win32());
            }

            let statusbar = Self { hwnd };
            statusbar.set_parts()?;

            Ok(statusbar)
        }
    }

    /// Set up status bar parts (sections)
    fn set_parts(&self) -> Result<(), windows::core::Error> {
        unsafe {
            // Define part widths (from right to left)
            // -1 means the last part extends to fill remaining space
            let parts: [i32; 5] = [
                -1,  // File info (extends to fill)
                600, // Position
                500, // Encoding
                400, // Line ending
                300, // File type
            ];

            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                SB_SETPARTS,
                WPARAM(parts.len()),
                LPARAM(parts.as_ptr() as isize),
            );

            // Set initial text
            self.set_text(StatusPart::FileInfo, "Ready")?;
            self.set_text(StatusPart::Position, "Ln: 1 Col: 1")?;
            self.set_text(StatusPart::Encoding, "UTF-8")?;
            self.set_text(StatusPart::LineEnding, "CRLF")?;
            self.set_text(StatusPart::FileType, "Normal")?;

            Ok(())
        }
    }

    /// Set text for a status bar part
    pub fn set_text(&self, part: StatusPart, text: &str) -> Result<(), windows::core::Error> {
        unsafe {
            let text_wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();

            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                SB_SETTEXT,
                WPARAM(part as usize),
                LPARAM(text_wide.as_ptr() as isize),
            );

            Ok(())
        }
    }

    /// Update position information (line and column)
    pub fn update_position(&self, line: usize, column: usize) -> Result<(), windows::core::Error> {
        let text = format!("Ln: {} Col: {}", line, column);
        self.set_text(StatusPart::Position, &text)
    }

    /// Update file information
    pub fn update_file_info(&self, path: &str, modified: bool) -> Result<(), windows::core::Error> {
        let status = if modified { " [Modified]" } else { "" };
        let text = format!("{}{}", path, status);
        self.set_text(StatusPart::FileInfo, &text)
    }

    /// Update encoding information
    pub fn update_encoding(&self, encoding: &str) -> Result<(), windows::core::Error> {
        self.set_text(StatusPart::Encoding, encoding)
    }

    /// Update line ending information
    pub fn update_line_ending(&self, line_ending: &str) -> Result<(), windows::core::Error> {
        self.set_text(StatusPart::LineEnding, line_ending)
    }

    /// Update file type/language
    pub fn update_file_type(&self, file_type: &str) -> Result<(), windows::core::Error> {
        self.set_text(StatusPart::FileType, file_type)
    }

    /// Get status bar window handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    /// Get status bar height
    pub fn height(&self) -> i32 {
        STATUSBAR_HEIGHT
    }

    /// Resize status bar (it automatically adjusts to parent width)
    pub fn resize(&self) {
        unsafe {
            // Send WM_SIZE to make statusbar recalculate its size
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                windows::Win32::UI::WindowsAndMessaging::WM_SIZE,
                WPARAM(0),
                LPARAM(0),
            );
        }
    }
}
