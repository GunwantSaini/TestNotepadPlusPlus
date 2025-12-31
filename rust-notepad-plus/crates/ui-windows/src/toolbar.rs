//! Toolbar implementation with common editing buttons

use notepad_core::CommandId;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Controls::{
    TBBUTTON, TBSTATE_ENABLED, TBSTYLE_FLAT, TBSTYLE_TOOLTIPS, TB_ADDBUTTONS,
    TB_BUTTONSTRUCTSIZE, TOOLBARCLASSNAMEW,
};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, WS_CHILD, WS_VISIBLE};

const TOOLBAR_HEIGHT: i32 = 32;

/// Toolbar control wrapper
pub struct Toolbar {
    hwnd: HWND,
}

impl Toolbar {
    /// Create a new toolbar
    pub fn new(parent: HWND, _h_instance: isize) -> Result<Self, windows::core::Error> {
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;

            // Create toolbar window
            let hwnd = CreateWindowExW(
                Default::default(),
                TOOLBARCLASSNAMEW,
                PCWSTR::null(),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(TBSTYLE_FLAT | TBSTYLE_TOOLTIPS),
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

            // Set button structure size
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                hwnd,
                TB_BUTTONSTRUCTSIZE,
                WPARAM(std::mem::size_of::<TBBUTTON>()),
                LPARAM(0),
            );

            let toolbar = Self { hwnd };
            toolbar.add_standard_buttons()?;

            Ok(toolbar)
        }
    }

    /// Add standard toolbar buttons
    fn add_standard_buttons(&self) -> Result<(), windows::core::Error> {
        unsafe {
            // Define toolbar buttons with text labels (no bitmap images for now)
            let buttons = vec![
                create_button(0, CommandId::FileNew, TBSTATE_ENABLED),
                create_button(1, CommandId::FileOpen, TBSTATE_ENABLED),
                create_button(2, CommandId::FileSave, TBSTATE_ENABLED),
                separator(),
                create_button(3, CommandId::EditCut, TBSTATE_ENABLED),
                create_button(4, CommandId::EditCopy, TBSTATE_ENABLED),
                create_button(5, CommandId::EditPaste, TBSTATE_ENABLED),
                separator(),
                create_button(6, CommandId::EditUndo, TBSTATE_ENABLED),
                create_button(7, CommandId::EditRedo, TBSTATE_ENABLED),
                separator(),
                create_button(8, CommandId::SearchFind, TBSTATE_ENABLED),
                create_button(9, CommandId::SearchReplace, TBSTATE_ENABLED),
            ];

            // Add buttons to toolbar
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                self.hwnd,
                TB_ADDBUTTONS,
                WPARAM(buttons.len()),
                LPARAM(buttons.as_ptr() as isize),
            );

            Ok(())
        }
    }

    /// Get toolbar window handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    /// Get toolbar height
    pub fn height(&self) -> i32 {
        TOOLBAR_HEIGHT
    }

    /// Resize toolbar to fit parent width
    pub fn resize(&self, width: i32) {
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
                self.hwnd,
                None,
                0,
                0,
                width,
                TOOLBAR_HEIGHT,
                windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER,
            )
            .ok();
        }
    }
}

/// Create a toolbar button
fn create_button(bitmap_index: i32, command_id: CommandId, state: u32) -> TBBUTTON {
    TBBUTTON {
        iBitmap: bitmap_index,
        idCommand: command_id.to_menu_id() as i32,
        fsState: state as u8, // Cast to u8 for TBBUTTON structure
        fsStyle: 0,           // BTNS_BUTTON
        bReserved: [0; 6],
        dwData: 0,
        iString: 0,
    }
}

/// Create a separator button
fn separator() -> TBBUTTON {
    TBBUTTON {
        iBitmap: 8, // I_IMAGENONE
        idCommand: 0,
        fsState: 0,
        fsStyle: 1, // BTNS_SEP
        bReserved: [0; 6],
        dwData: 0,
        iString: 0,
    }
}
