//! Find and Replace dialog implementation

use notepad_search::{SearchEngine, SearchOptions};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Controls::EM_SETSEL;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, EndDialog, FindWindowExW, GetDlgItem, GetParent,
    GetWindowTextW, SendMessageW, HMENU, WM_GETTEXT, WINDOW_EX_STYLE, WINDOW_STYLE, WM_CLOSE,
    WM_COMMAND, WM_INITDIALOG, WS_BORDER, WS_CAPTION, WS_CHILD, WS_POPUP, WS_SYSMENU, WS_TABSTOP,
    WS_VISIBLE,
};

// Constants for dialog
const WS_EX_CONTROLPARENT: u32 = 0x00010000;
const ES_AUTOHSCROLL: u32 = 0x0080;
const BM_GETCHECK: u32 = 0x00F0;
const BST_CHECKED: u32 = 0x0001;

// Control IDs
const IDC_FIND_EDIT: i32 = 1001;
const IDC_FIND_NEXT: i32 = 1002;
const IDC_CANCEL: i32 = 1003;
const IDC_CASE_SENSITIVE: i32 = 1004;

/// Find dialog result
#[derive(Debug, Clone)]
pub struct FindResult {
    pub search_text: String,
    pub case_sensitive: bool,
}

/// Create and show a Find dialog
pub fn show_find_dialog(_parent: HWND) -> Option<FindResult> {
    // For now, we'll use a simple implementation
    // In a full implementation, this would create a dialog from a resource
    // or dynamically create dialog controls

    log::info!("Find dialog requested (simplified implementation)");

    // This is a placeholder - a real implementation would:
    // 1. Create a dialog window
    // 2. Add edit controls for search text
    // 3. Add checkboxes for options (case sensitive, whole word, regex)
    // 4. Add buttons (Find Next, Find Previous, Cancel)
    // 5. Handle the dialog message loop
    // 6. Return the search parameters when user clicks Find

    // For now, return None to indicate dialog was cancelled
    None
}

/// Create a simple Find dialog window (manual creation without resource file)
pub unsafe fn create_find_dialog(parent: HWND) -> HWND {
    // Create dialog window
    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE(WS_EX_CONTROLPARENT),
        w!("Static"), // Using Static as a simple dialog base
        w!("Find"),
        WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
        100,
        100,
        400,
        150,
        parent,
        None,
        None,
        None,
    );

    if hwnd.0 == 0 {
        return HWND(0);
    }

    // Create search text edit control
    CreateWindowExW(
        Default::default(),
        w!("EDIT"),
        w!(""),
        WS_CHILD | WS_VISIBLE | WS_BORDER | WS_TABSTOP | WINDOW_STYLE(ES_AUTOHSCROLL),
        10,
        25,
        300,
        25,
        hwnd,
        HMENU(IDC_FIND_EDIT as isize),
        None,
        None,
    );

    // Create "Find what:" label
    CreateWindowExW(
        Default::default(),
        w!("STATIC"),
        w!("Find what:"),
        WS_CHILD | WS_VISIBLE,
        10,
        5,
        100,
        20,
        hwnd,
        None,
        None,
        None,
    );

    // Create "Find Next" button
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Find Next"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP,
        320,
        10,
        70,
        25,
        hwnd,
        HMENU(IDC_FIND_NEXT as isize),
        None,
        None,
    );

    // Create "Cancel" button
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Cancel"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP,
        320,
        45,
        70,
        25,
        hwnd,
        HMENU(IDC_CANCEL as isize),
        None,
        None,
    );

    // Create "Match case" checkbox
    CreateWindowExW(
        Default::default(),
        w!("BUTTON"),
        w!("Match case"),
        WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(0x0003), // BS_AUTOCHECKBOX
        10,
        60,
        150,
        20,
        hwnd,
        HMENU(IDC_CASE_SENSITIVE as isize),
        None,
        None,
    );

    hwnd
}

/// Dialog procedure for Find dialog
pub unsafe extern "system" fn find_dialog_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_INITDIALOG => {
            // Initialize dialog
            log::debug!("Find dialog initialized");
            LRESULT(1)
        }
        WM_COMMAND => {
            let control_id = (wparam.0 & 0xFFFF) as i32;
            match control_id {
                IDC_FIND_NEXT => {
                    // Get search text from the find dialog edit control
                    let edit_hwnd = GetDlgItem(hwnd, IDC_FIND_EDIT);
                    let mut search_buffer = vec![0u16; 256];
                    let search_len = GetWindowTextW(edit_hwnd, &mut search_buffer);

                    if search_len > 0 {
                        let search_text = String::from_utf16_lossy(&search_buffer[..search_len as usize]);
                        log::info!("Searching for: {}", search_text);

                        // Get the parent window (main window)
                        let parent_hwnd = GetParent(hwnd);

                        // Find the editor control in the parent window
                        let editor_hwnd = FindWindowExW(parent_hwnd, None, w!("EDIT"), PCWSTR::null());

                        if editor_hwnd.0 != 0 {
                            // Get editor text
                            let mut editor_buffer = vec![0u16; 65536]; // 64KB buffer
                            let editor_len = SendMessageW(
                                editor_hwnd,
                                WM_GETTEXT,
                                WPARAM(editor_buffer.len()),
                                LPARAM(editor_buffer.as_mut_ptr() as isize),
                            );

                            if editor_len.0 > 0 {
                                let editor_text = String::from_utf16_lossy(&editor_buffer[..editor_len.0 as usize]);

                                // Get current selection to start search from there
                                let current_pos = SendMessageW(
                                    editor_hwnd,
                                    EM_SETSEL,
                                    WPARAM(usize::MAX),
                                    LPARAM(0),
                                ).0 as usize;

                                // Check case sensitive checkbox
                                let case_checkbox = GetDlgItem(hwnd, IDC_CASE_SENSITIVE);
                                let is_case_sensitive = if case_checkbox.0 != 0 {
                                    SendMessageW(case_checkbox, BM_GETCHECK, WPARAM(0), LPARAM(0)).0 == BST_CHECKED as isize
                                } else {
                                    false
                                };

                                // Create search engine
                                let options = SearchOptions {
                                    case_sensitive: is_case_sensitive,
                                    ..Default::default()
                                };
                                let engine = SearchEngine::new(options);

                                // Perform search starting from current position
                                let start_pos = if current_pos < editor_text.len() {
                                    current_pos + 1
                                } else {
                                    0
                                };

                                match engine.find(&editor_text, &search_text, start_pos) {
                                    Ok(Some(result)) => {
                                        // Select the found text
                                        SendMessageW(
                                            editor_hwnd,
                                            EM_SETSEL,
                                            WPARAM(result.start),
                                            LPARAM(result.end as isize),
                                        );
                                        log::info!("Found at position {}..{}", result.start, result.end);
                                    }
                                    Ok(None) => {
                                        // Not found - try wrapping around from beginning
                                        match engine.find(&editor_text, &search_text, 0) {
                                            Ok(Some(result)) => {
                                                SendMessageW(
                                                    editor_hwnd,
                                                    EM_SETSEL,
                                                    WPARAM(result.start),
                                                    LPARAM(result.end as isize),
                                                );
                                                log::info!("Found at position {}..{} (wrapped)", result.start, result.end);
                                            }
                                            Ok(None) => {
                                                let msg_text: Vec<u16> = format!("Cannot find \"{}\"", search_text)
                                                    .encode_utf16()
                                                    .chain(std::iter::once(0))
                                                    .collect();
                                                windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                                                    hwnd,
                                                    PCWSTR(msg_text.as_ptr()),
                                                    w!("Find"),
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
