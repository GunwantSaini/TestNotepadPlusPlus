//! Go To Line dialog implementation

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Controls::EM_SETSEL;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, EndDialog, FindWindowExW, GetDlgItem, GetParent,
    GetWindowTextW, SendMessageW, HMENU, MB_ICONERROR, MB_OK, WM_GETTEXT, WINDOW_EX_STYLE,
    WINDOW_STYLE, WM_CLOSE, WM_COMMAND, WM_INITDIALOG, WS_BORDER, WS_CAPTION, WS_CHILD, WS_POPUP,
    WS_SYSMENU, WS_TABSTOP, WS_VISIBLE,
};

// Constants for dialog
const WS_EX_CONTROLPARENT: u32 = 0x00010000;
const ES_AUTOHSCROLL: u32 = 0x0080;
const ES_NUMBER: u32 = 0x2000;

// Control IDs
const IDC_LINE_EDIT: i32 = 3001;
const IDC_GO: i32 = 3002;
const IDC_CANCEL: i32 = 3003;

/// Go To Line dialog result
#[derive(Debug, Clone)]
pub struct GoToLineResult {
    pub line_number: usize,
}

/// Create a simple Go To Line dialog window
pub unsafe fn create_goto_dialog(parent: HWND) -> HWND {
    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE(WS_EX_CONTROLPARENT),
        w!("Static"),
        w!("Go To Line"),
        WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
        100,
        100,
        350,
        150,
        parent,
        None,
        None,
        None,
    );

    if hwnd.0 == 0 {
        return HWND(0);
    }

    // "Line number:" label
    CreateWindowExW(
        Default::default(),
        w!("STATIC"),
        w!("Line number:"),
        WS_CHILD | WS_VISIBLE,
        10,
        10,
        100,
        20,
        hwnd,
        None,
        None,
        None,
    );

    // Line number edit control (numbers only)
    CreateWindowExW(
        Default::default(),
        w!("EDIT"),
        w!("1"),
        WS_CHILD | WS_VISIBLE | WS_BORDER | WS_TABSTOP | WINDOW_STYLE(ES_AUTOHSCROLL | ES_NUMBER),
        10,
        35,
        230,
        25,
        hwnd,
        HMENU(IDC_LINE_EDIT as isize),
        None,
        None,
    );

    // "Go" button
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Go"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP,
        250,
        10,
        90,
        25,
        hwnd,
        HMENU(IDC_GO as isize),
        None,
        None,
    );

    // "Cancel" button
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Cancel"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP,
        250,
        45,
        90,
        25,
        hwnd,
        HMENU(IDC_CANCEL as isize),
        None,
        None,
    );

    hwnd
}

/// Dialog procedure for Go To Line dialog
pub unsafe extern "system" fn goto_dialog_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_INITDIALOG => {
            log::debug!("Go To Line dialog initialized");
            LRESULT(1)
        }
        WM_COMMAND => {
            let control_id = (wparam.0 & 0xFFFF) as i32;
            match control_id {
                IDC_GO => {
                    perform_goto(hwnd);
                    LRESULT(0)
                }
                IDC_CANCEL => {
                    let _ = EndDialog(hwnd, 0);
                    LRESULT(0)
                }
                _ => LRESULT(0),
            }
        }
        WM_CLOSE => {
            let _ = EndDialog(hwnd, 0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// Perform go to line operation
unsafe fn perform_goto(hwnd: HWND) {
    let edit_hwnd = GetDlgItem(hwnd, IDC_LINE_EDIT);
    let mut buffer = vec![0u16; 32];
    let len = GetWindowTextW(edit_hwnd, &mut buffer);

    if len > 0 {
        let line_str = String::from_utf16_lossy(&buffer[..len as usize]);

        if let Ok(line_number) = line_str.parse::<usize>() {
            if line_number == 0 {
                show_error(hwnd, "Invalid Line Number", "Line number must be greater than 0");
                return;
            }

            log::info!("Going to line: {}", line_number);

            let parent_hwnd = GetParent(hwnd);
            let editor_hwnd = FindWindowExW(parent_hwnd, None, w!("EDIT"), PCWSTR::null());

            if editor_hwnd.0 != 0 {
                // Get editor text to find line positions
                let mut editor_buffer = vec![0u16; 65536];
                let editor_len = SendMessageW(
                    editor_hwnd,
                    WM_GETTEXT,
                    WPARAM(editor_buffer.len()),
                    LPARAM(editor_buffer.as_mut_ptr() as isize),
                );

                if editor_len.0 > 0 {
                    let editor_text = String::from_utf16_lossy(&editor_buffer[..editor_len.0 as usize]);

                    // Count lines to find the position
                    let lines: Vec<&str> = editor_text.lines().collect();

                    if line_number > lines.len() {
                        show_error(
                            hwnd,
                            "Line Out of Range",
                            &format!("Line {} does not exist. The file has only {} line(s).",
                                    line_number, lines.len())
                        );
                        return;
                    }

                    // Calculate character position for the line
                    let mut char_pos = 0;
                    for i in 0..(line_number - 1) {
                        char_pos += lines[i].len() + 1; // +1 for newline
                    }

                    // Set cursor position to the beginning of the line
                    SendMessageW(
                        editor_hwnd,
                        EM_SETSEL,
                        WPARAM(char_pos),
                        LPARAM(char_pos as isize),
                    );

                    log::info!("Cursor moved to line {} (position {})", line_number, char_pos);

                    // Close the dialog
                    let _ = EndDialog(hwnd, 1);
                }
            }
        } else {
            show_error(hwnd, "Invalid Input", "Please enter a valid line number");
        }
    }
}

/// Show an error message box
unsafe fn show_error(hwnd: HWND, title: &str, message: &str) {
    let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    let message_wide: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();

    windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
        hwnd,
        PCWSTR(message_wide.as_ptr()),
        PCWSTR(title_wide.as_ptr()),
        MB_OK | MB_ICONERROR,
    );
}
