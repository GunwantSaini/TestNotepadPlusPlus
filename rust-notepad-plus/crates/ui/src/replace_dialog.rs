//! Replace dialog implementation

use notepad_search::{SearchEngine, SearchOptions};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Controls::EM_SETSEL;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, EndDialog, FindWindowExW, GetDlgItem, GetParent,
    GetWindowTextW, SendMessageW, HMENU, WM_GETTEXT, WM_SETTEXT, WINDOW_EX_STYLE, WINDOW_STYLE,
    WM_CLOSE, WM_COMMAND, WM_INITDIALOG, WS_BORDER, WS_CAPTION, WS_CHILD, WS_POPUP, WS_SYSMENU,
    WS_TABSTOP, WS_VISIBLE,
};

// Constants for dialog
const WS_EX_CONTROLPARENT: u32 = 0x00010000;
const ES_AUTOHSCROLL: u32 = 0x0080;
const BM_GETCHECK: u32 = 0x00F0;
const BST_CHECKED: u32 = 0x0001;

// Control IDs
const IDC_FIND_EDIT: i32 = 2001;
const IDC_REPLACE_EDIT: i32 = 2002;
const IDC_FIND_NEXT: i32 = 2003;
const IDC_REPLACE: i32 = 2004;
const IDC_REPLACE_ALL: i32 = 2005;
const IDC_CANCEL: i32 = 2006;
const IDC_CASE_SENSITIVE: i32 = 2007;

/// Replace dialog result
#[derive(Debug, Clone)]
pub struct ReplaceResult {
    pub search_text: String,
    pub replace_text: String,
    pub case_sensitive: bool,
}

/// Create and show a Replace dialog
pub fn show_replace_dialog(_parent: HWND) -> Option<ReplaceResult> {
    // Placeholder for now
    None
}

/// Create a simple Replace dialog window (manual creation without resource file)
pub unsafe fn create_replace_dialog(parent: HWND) -> HWND {
    // Create dialog window (larger than Find dialog to fit replace controls)
    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE(WS_EX_CONTROLPARENT),
        w!("Static"),
        w!("Replace"),
        WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
        100,
        100,
        450,
        200,
        parent,
        None,
        None,
        None,
    );

    if hwnd.0 == 0 {
        return HWND(0);
    }

    // "Find what:" label
    CreateWindowExW(
        Default::default(),
        w!("STATIC"),
        w!("Find what:"),
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

    // Find text edit control
    CreateWindowExW(
        Default::default(),
        w!("EDIT"),
        w!(""),
        WS_CHILD | WS_VISIBLE | WS_BORDER | WS_TABSTOP | WINDOW_STYLE(ES_AUTOHSCROLL),
        10,
        30,
        330,
        25,
        hwnd,
        HMENU(IDC_FIND_EDIT as isize),
        None,
        None,
    );

    // "Replace with:" label
    CreateWindowExW(
        Default::default(),
        w!("STATIC"),
        w!("Replace with:"),
        WS_CHILD | WS_VISIBLE,
        10,
        65,
        100,
        20,
        hwnd,
        None,
        None,
        None,
    );

    // Replace text edit control
    CreateWindowExW(
        Default::default(),
        w!("EDIT"),
        w!(""),
        WS_CHILD | WS_VISIBLE | WS_BORDER | WS_TABSTOP | WINDOW_STYLE(ES_AUTOHSCROLL),
        10,
        85,
        330,
        25,
        hwnd,
        HMENU(IDC_REPLACE_EDIT as isize),
        None,
        None,
    );

    // "Find Next" button
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Find Next"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP,
        350,
        10,
        90,
        25,
        hwnd,
        HMENU(IDC_FIND_NEXT as isize),
        None,
        None,
    );

    // "Replace" button
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Replace"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP,
        350,
        45,
        90,
        25,
        hwnd,
        HMENU(IDC_REPLACE as isize),
        None,
        None,
    );

    // "Replace All" button
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Replace All"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP,
        350,
        80,
        90,
        25,
        hwnd,
        HMENU(IDC_REPLACE_ALL as isize),
        None,
        None,
    );

    // "Cancel" button
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Cancel"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP,
        350,
        115,
        90,
        25,
        hwnd,
        HMENU(IDC_CANCEL as isize),
        None,
        None,
    );

    // "Match case" checkbox
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Match case"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(0x0003), // BS_AUTOCHECKBOX
        10,
        120,
        150,
        20,
        hwnd,
        HMENU(IDC_CASE_SENSITIVE as isize),
        None,
        None,
    );

    hwnd
}

/// Dialog procedure for Replace dialog
pub unsafe extern "system" fn replace_dialog_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_INITDIALOG => {
            log::debug!("Replace dialog initialized");
            LRESULT(1)
        }
        WM_COMMAND => {
            let control_id = (wparam.0 & 0xFFFF) as i32;
            match control_id {
                IDC_FIND_NEXT => {
                    perform_find(hwnd);
                    LRESULT(0)
                }
                IDC_REPLACE => {
                    perform_replace(hwnd, false);
                    LRESULT(0)
                }
                IDC_REPLACE_ALL => {
                    perform_replace(hwnd, true);
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

/// Perform find operation
unsafe fn perform_find(hwnd: HWND) {
    let edit_hwnd = GetDlgItem(hwnd, IDC_FIND_EDIT);
    let mut search_buffer = vec![0u16; 256];
    let search_len = GetWindowTextW(edit_hwnd, &mut search_buffer);

    if search_len > 0 {
        let search_text = String::from_utf16_lossy(&search_buffer[..search_len as usize]);
        log::info!("Searching for: {}", search_text);

        let parent_hwnd = GetParent(hwnd);
        let editor_hwnd = FindWindowExW(parent_hwnd, None, w!("EDIT"), PCWSTR::null());

        if editor_hwnd.0 != 0 {
            let mut editor_buffer = vec![0u16; 65536];
            let editor_len = SendMessageW(
                editor_hwnd,
                WM_GETTEXT,
                WPARAM(editor_buffer.len()),
                LPARAM(editor_buffer.as_mut_ptr() as isize),
            );

            if editor_len.0 > 0 {
                let editor_text = String::from_utf16_lossy(&editor_buffer[..editor_len.0 as usize]);

                let current_pos = SendMessageW(editor_hwnd, EM_SETSEL, WPARAM(usize::MAX), LPARAM(0)).0
                    as usize;

                let case_checkbox = GetDlgItem(hwnd, IDC_CASE_SENSITIVE);
                let is_case_sensitive = if case_checkbox.0 != 0 {
                    SendMessageW(case_checkbox, BM_GETCHECK, WPARAM(0), LPARAM(0)).0
                        == BST_CHECKED as isize
                } else {
                    false
                };

                let options = SearchOptions {
                    case_sensitive: is_case_sensitive,
                    ..Default::default()
                };
                let engine = SearchEngine::new(options);

                let start_pos = if current_pos < editor_text.len() {
                    current_pos + 1
                } else {
                    0
                };

                match engine.find(&editor_text, &search_text, start_pos) {
                    Ok(Some(result)) => {
                        SendMessageW(
                            editor_hwnd,
                            EM_SETSEL,
                            WPARAM(result.start),
                            LPARAM(result.end as isize),
                        );
                        log::info!("Found at position {}..{}", result.start, result.end);
                    }
                    Ok(None) => {
                        match engine.find(&editor_text, &search_text, 0) {
                            Ok(Some(result)) => {
                                SendMessageW(
                                    editor_hwnd,
                                    EM_SETSEL,
                                    WPARAM(result.start),
                                    LPARAM(result.end as isize),
                                );
                                log::info!(
                                    "Found at position {}..{} (wrapped)",
                                    result.start,
                                    result.end
                                );
                            }
                            Ok(None) => {
                                let msg_text: Vec<u16> = format!("Cannot find \"{}\"", search_text)
                                    .encode_utf16()
                                    .chain(std::iter::once(0))
                                    .collect();
                                windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                                    hwnd,
                                    PCWSTR(msg_text.as_ptr()),
                                    w!("Replace"),
                                    windows::Win32::UI::WindowsAndMessaging::MB_OK,
                                );
                            }
                            Err(e) => {
                                log::error!("Search error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Search error: {}", e);
                    }
                }
            }
        }
    }
}

/// Perform replace operation
unsafe fn perform_replace(hwnd: HWND, replace_all: bool) {
    let find_edit = GetDlgItem(hwnd, IDC_FIND_EDIT);
    let replace_edit = GetDlgItem(hwnd, IDC_REPLACE_EDIT);

    let mut find_buffer = vec![0u16; 256];
    let mut replace_buffer = vec![0u16; 256];

    let find_len = GetWindowTextW(find_edit, &mut find_buffer);
    let replace_len = GetWindowTextW(replace_edit, &mut replace_buffer);

    if find_len > 0 {
        let search_text = String::from_utf16_lossy(&find_buffer[..find_len as usize]);
        let replace_text = String::from_utf16_lossy(&replace_buffer[..replace_len as usize]);

        let parent_hwnd = GetParent(hwnd);
        let editor_hwnd = FindWindowExW(parent_hwnd, None, w!("EDIT"), PCWSTR::null());

        if editor_hwnd.0 != 0 {
            let mut editor_buffer = vec![0u16; 65536];
            let editor_len = SendMessageW(
                editor_hwnd,
                WM_GETTEXT,
                WPARAM(editor_buffer.len()),
                LPARAM(editor_buffer.as_mut_ptr() as isize),
            );

            if editor_len.0 > 0 {
                let editor_text = String::from_utf16_lossy(&editor_buffer[..editor_len.0 as usize]);

                let case_checkbox = GetDlgItem(hwnd, IDC_CASE_SENSITIVE);
                let is_case_sensitive = if case_checkbox.0 != 0 {
                    SendMessageW(case_checkbox, BM_GETCHECK, WPARAM(0), LPARAM(0)).0
                        == BST_CHECKED as isize
                } else {
                    false
                };

                let options = SearchOptions {
                    case_sensitive: is_case_sensitive,
                    ..Default::default()
                };
                let engine = SearchEngine::new(options);

                if replace_all {
                    // Replace all occurrences
                    match engine.replace(&editor_text, &search_text, &replace_text) {
                        Ok(new_text) => {
                            let text_wide: Vec<u16> =
                                new_text.encode_utf16().chain(std::iter::once(0)).collect();
                            SendMessageW(
                                editor_hwnd,
                                WM_SETTEXT,
                                WPARAM(0),
                                LPARAM(text_wide.as_ptr() as isize),
                            );

                            let count = engine.find_all(&editor_text, &search_text).unwrap_or(vec![]).len();
                            let msg_text: Vec<u16> = format!("Replaced {} occurrence(s)", count)
                                .encode_utf16()
                                .chain(std::iter::once(0))
                                .collect();
                            windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                                hwnd,
                                PCWSTR(msg_text.as_ptr()),
                                w!("Replace All"),
                                windows::Win32::UI::WindowsAndMessaging::MB_OK
                                    | windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION,
                            );
                        }
                        Err(e) => {
                            log::error!("Replace error: {}", e);
                        }
                    }
                } else {
                    // Replace single occurrence (current selection or find next)
                    match engine.replace_first(&editor_text, &search_text, &replace_text) {
                        Ok(new_text) => {
                            let text_wide: Vec<u16> =
                                new_text.encode_utf16().chain(std::iter::once(0)).collect();
                            SendMessageW(
                                editor_hwnd,
                                WM_SETTEXT,
                                WPARAM(0),
                                LPARAM(text_wide.as_ptr() as isize),
                            );
                            log::info!("Replaced first occurrence");

                            // Find next occurrence after replacement
                            perform_find(hwnd);
                        }
                        Err(e) => {
                            log::error!("Replace error: {}", e);
                        }
                    }
                }
            }
        }
    }
}
