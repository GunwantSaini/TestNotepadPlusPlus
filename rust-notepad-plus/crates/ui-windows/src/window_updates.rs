//! Window title and status bar update helpers

use notepad_ui_core::app_state::AppState;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, MessageBoxW, SendMessageW, SetWindowTextW, MB_ICONQUESTION, MB_YESNOCANCEL,
    WM_SETTEXT,
};

/// Update the window title based on app state
pub unsafe fn update_window_title(hwnd: HWND, state: &AppState) {
    let title = state.get_window_title();
    let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

    SetWindowTextW(hwnd, PCWSTR(title_wide.as_ptr())).ok();
}

/// Update the status bar position section
pub unsafe fn update_status_bar_position(hwnd: HWND, state: &AppState) {
    // Find the status bar (it's a child window of class "msctls_statusbar32")
    let statusbar_hwnd = FindWindowExW(
        hwnd,
        None,
        windows::core::w!("msctls_statusbar32"),
        PCWSTR::null(),
    );

    if statusbar_hwnd.0 != 0 {
        let position_text = state.get_position_status();
        let text_wide: Vec<u16> = position_text.encode_utf16().chain(std::iter::once(0)).collect();

        // SB_SETTEXT with part 1 (position section)
        const SB_SETTEXT: u32 = 0x0401;
        SendMessageW(
            statusbar_hwnd,
            SB_SETTEXT,
            WPARAM(1), // Part 1 is the position section
            LPARAM(text_wide.as_ptr() as isize),
        );
    }
}

/// Update the status bar encoding/EOL section
pub unsafe fn update_status_bar_encoding(hwnd: HWND, state: &AppState) {
    // Find the status bar (it's a child window of class "msctls_statusbar32")
    let statusbar_hwnd = FindWindowExW(
        hwnd,
        None,
        windows::core::w!("msctls_statusbar32"),
        PCWSTR::null(),
    );

    if statusbar_hwnd.0 != 0 {
        let encoding_text = state.get_encoding_status();
        let text_wide: Vec<u16> = encoding_text.encode_utf16().chain(std::iter::once(0)).collect();

        // SB_SETTEXT with part 2 (encoding section)
        const SB_SETTEXT: u32 = 0x0401;
        SendMessageW(
            statusbar_hwnd,
            SB_SETTEXT,
            WPARAM(2), // Part 2 is the encoding section
            LPARAM(text_wide.as_ptr() as isize),
        );
    }
}

/// Get current cursor position from editor
pub unsafe fn get_editor_cursor_position(hwnd: HWND) -> (usize, usize, usize) {
    use windows::Win32::UI::Controls::EM_GETSEL;
    use windows::Win32::UI::WindowsAndMessaging::WM_GETTEXT;

    let editor_hwnd = FindWindowExW(hwnd, None, windows::core::w!("EDIT"), PCWSTR::null());

    if editor_hwnd.0 == 0 {
        return (1, 1, 1);
    }

    // Get current selection (cursor position)
    let mut start: u32 = 0;
    let mut end: u32 = 0;
    SendMessageW(
        editor_hwnd,
        EM_GETSEL,
        WPARAM(&mut start as *mut u32 as usize),
        LPARAM(&mut end as *mut u32 as isize),
    );

    // Get editor text to calculate line and column
    let mut buffer = vec![0u16; 65536];
    let len = SendMessageW(
        editor_hwnd,
        WM_GETTEXT,
        WPARAM(buffer.len()),
        LPARAM(buffer.as_mut_ptr() as isize),
    );

    if len.0 > 0 {
        let text = String::from_utf16_lossy(&buffer[..len.0 as usize]);

        // Calculate line and column from cursor position
        let cursor_pos = start as usize;
        let before_cursor = if cursor_pos < text.len() {
            &text[..cursor_pos]
        } else {
            &text
        };

        let line = before_cursor.lines().count();
        let last_newline_pos = before_cursor.rfind('\n').map(|p| p + 1).unwrap_or(0);
        let column = cursor_pos - last_newline_pos + 1;
        let total_lines = text.lines().count().max(1);

        (line, column, total_lines)
    } else {
        (1, 1, 1)
    }
}

/// Prompt user to save changes
/// Returns: 6 = IDYES, 7 = IDNO, 2 = IDCANCEL
pub unsafe fn prompt_save_changes(hwnd: HWND) -> i32 {
    let msg = "Do you want to save changes?";
    let msg_wide: Vec<u16> = msg.encode_utf16().chain(std::iter::once(0)).collect();
    let title_wide: Vec<u16> = "Notepad++".encode_utf16().chain(std::iter::once(0)).collect();

    MessageBoxW(
        hwnd,
        PCWSTR(msg_wide.as_ptr()),
        PCWSTR(title_wide.as_ptr()),
        MB_YESNOCANCEL | MB_ICONQUESTION,
    )
    .0
}
