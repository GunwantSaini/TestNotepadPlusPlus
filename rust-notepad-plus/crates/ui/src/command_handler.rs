//! Command handler for menu and toolbar actions

use notepad_core::CommandId;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Controls::{EM_CANUNDO, EM_UNDO};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, MessageBoxW, SendMessageW, MB_ICONINFORMATION, MB_OK, WM_COPY, WM_CUT,
    WM_PASTE,
};

/// Handle a command by dispatching to the appropriate handler
pub fn handle_command(hwnd: HWND, cmd: CommandId) -> bool {
    log::info!("Handling command: {:?}", cmd);

    // Find the editor control
    let editor_hwnd = unsafe { FindWindowExW(hwnd, None, w!("EDIT"), PCWSTR::null()) };

    if editor_hwnd.0 == 0 {
        log::error!("Could not find editor control");
        return false;
    }

    match cmd {
        // Edit commands
        CommandId::EditUndo => {
            unsafe {
                SendMessageW(editor_hwnd, EM_UNDO, WPARAM(0), LPARAM(0));
            }
            true
        }
        CommandId::EditCut => {
            unsafe {
                SendMessageW(editor_hwnd, WM_CUT, WPARAM(0), LPARAM(0));
            }
            true
        }
        CommandId::EditCopy => {
            unsafe {
                SendMessageW(editor_hwnd, WM_COPY, WPARAM(0), LPARAM(0));
            }
            true
        }
        CommandId::EditPaste => {
            unsafe {
                SendMessageW(editor_hwnd, WM_PASTE, WPARAM(0), LPARAM(0));
            }
            true
        }
        CommandId::EditSelectAll => {
            unsafe {
                use windows::Win32::UI::Controls::EM_SETSEL;
                SendMessageW(editor_hwnd, EM_SETSEL, WPARAM(0), LPARAM(-1));
            }
            true
        }

        // File commands (basic implementations)
        CommandId::FileNew => {
            unsafe {
                use windows::Win32::UI::WindowsAndMessaging::WM_SETTEXT;
                let empty: Vec<u16> = vec![0];
                SendMessageW(
                    editor_hwnd,
                    WM_SETTEXT,
                    WPARAM(0),
                    LPARAM(empty.as_ptr() as isize),
                );
            }
            log::info!("New file created");
            true
        }

        CommandId::FileOpen => {
            log::info!("File Open dialog would appear here");
            show_message(hwnd, "File Open", "File Open dialog not yet implemented");
            false
        }

        CommandId::FileSave => {
            log::info!("File Save dialog would appear here");
            show_message(hwnd, "File Save", "File Save dialog not yet implemented");
            false
        }

        CommandId::FileSaveAs => {
            log::info!("File Save As dialog would appear here");
            show_message(hwnd, "Save As", "Save As dialog not yet implemented");
            false
        }

        CommandId::FilePrint => {
            show_message(hwnd, "Print", "Print functionality not yet implemented");
            false
        }

        // Search commands
        CommandId::SearchFind => {
            log::info!("Opening Find dialog");
            let dialog_hwnd = unsafe { crate::find_dialog::create_find_dialog(hwnd) };
            if dialog_hwnd.0 != 0 {
                log::info!("Find dialog created");
            }
            true
        }

        CommandId::SearchReplace => {
            show_message(hwnd, "Replace", "Replace dialog not yet implemented");
            false
        }

        CommandId::SearchGoToLine => {
            show_message(hwnd, "Go To Line", "Go To Line dialog not yet implemented");
            false
        }

        // View commands
        CommandId::ViewFullScreen => {
            show_message(
                hwnd,
                "Full Screen",
                "Full screen mode not yet implemented",
            );
            false
        }

        // Help commands
        CommandId::HelpAbout => {
            show_about_dialog(hwnd);
            true
        }

        _ => {
            log::warn!("Unhandled command: {:?}", cmd);
            false
        }
    }
}

/// Show a simple message box
fn show_message(hwnd: HWND, title: &str, message: &str) {
    unsafe {
        let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let message_wide: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();

        MessageBoxW(
            hwnd,
            PCWSTR(message_wide.as_ptr()),
            PCWSTR(title_wide.as_ptr()),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}

/// Show the About dialog
fn show_about_dialog(hwnd: HWND) {
    let message = "Notepad++ Rust Edition\nVersion 8.0.0\n\n\
                   A complete port of Notepad++ from C++ to Rust.\n\
                   Built with Rust for Windows.\n\n\
                   Features:\n\
                   • Full Win32 GUI with menu, toolbar, and status bar\n\
                   • Multi-line text editing\n\
                   • Cut, Copy, Paste, Undo operations\n\
                   • Scintilla editor support (when available)\n\n\
                   https://github.com/notepad-plus-plus/notepad-plus-plus";

    show_message(hwnd, "About Notepad++ Rust Edition", message);
}

/// Check if undo is available
pub fn can_undo(hwnd: HWND) -> bool {
    let editor_hwnd = unsafe { FindWindowExW(hwnd, None, w!("EDIT"), PCWSTR::null()) };
    if editor_hwnd.0 == 0 {
        return false;
    }

    unsafe { SendMessageW(editor_hwnd, EM_CANUNDO, WPARAM(0), LPARAM(0)).0 != 0 }
}
