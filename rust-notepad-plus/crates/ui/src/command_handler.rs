//! Command handler for menu and toolbar actions

use crate::file_dialogs::{show_open_dialog, show_save_dialog};
use notepad_core::CommandId;
use std::fs;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Controls::{EM_CANUNDO, EM_UNDO};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, MessageBoxW, SendMessageW, MB_ICONERROR, MB_ICONINFORMATION, MB_OK, WM_COPY,
    WM_CUT, WM_PASTE, WM_SETTEXT,
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
            // Check if current document has unsaved changes
            let is_dirty = crate::global_state::read_state(|s| s.is_dirty);
            if is_dirty {
                let result = unsafe { crate::window_updates::prompt_save_changes(hwnd) };
                if result == 6 {
                    // IDYES - save first
                    if !handle_command(hwnd, CommandId::FileSave) {
                        return false; // Save was cancelled or failed
                    }
                } else if result == 2 {
                    // IDCANCEL - don't create new file
                    return false;
                }
                // IDNO (7) - proceed without saving
            }

            // Clear editor
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

            // Reset state
            crate::global_state::with_state(|state| {
                state.set_current_file(None);
                state.set_dirty(false);
                state.update_cursor_position(1, 1, 1);
            });

            // Update window title and status bar
            crate::global_state::read_state(|state| {
                unsafe {
                    crate::window_updates::update_window_title(hwnd, state);
                    crate::window_updates::update_status_bar_position(hwnd, state);
                }
            });

            log::info!("New file created");
            true
        }

        CommandId::FileOpen => {
            // Check if current document has unsaved changes
            let is_dirty = crate::global_state::read_state(|s| s.is_dirty);
            if is_dirty {
                let result = unsafe { crate::window_updates::prompt_save_changes(hwnd) };
                if result == 6 {
                    // IDYES - save first
                    if !handle_command(hwnd, CommandId::FileSave) {
                        return false; // Save was cancelled or failed
                    }
                } else if result == 2 {
                    // IDCANCEL - don't open file
                    return false;
                }
                // IDNO (7) - proceed without saving
            }

            log::info!("Opening File Open dialog");
            if let Some(file_path) = show_open_dialog(hwnd) {
                log::info!("Opening file: {:?}", file_path);
                // Read file with encoding detection
                match crate::encoding::Encoding::read_file_with_detection(&file_path) {
                    Ok((contents, detected_encoding)) => {
                        // Detect line ending
                        let detected_line_ending = crate::encoding::LineEnding::detect(&contents);

                        log::info!("Detected encoding: {:?}, line ending: {:?}", detected_encoding, detected_line_ending);

                        // Load the file contents into the editor
                        unsafe {
                            let text_wide: Vec<u16> =
                                contents.encode_utf16().chain(std::iter::once(0)).collect();
                            SendMessageW(
                                editor_hwnd,
                                WM_SETTEXT,
                                WPARAM(0),
                                LPARAM(text_wide.as_ptr() as isize),
                            );
                        }

                        // Update state with encoding and line ending info
                        crate::global_state::with_state(|state| {
                            state.set_current_file(Some(file_path.clone()));
                            state.set_dirty(false);
                            state.current_encoding = detected_encoding;
                            state.current_line_ending = detected_line_ending;
                        });

                        // Update UI
                        crate::global_state::read_state(|state| {
                            unsafe {
                                crate::window_updates::update_window_title(hwnd, state);
                                crate::window_updates::update_status_bar_position(hwnd, state);
                                crate::window_updates::update_status_bar_encoding(hwnd, state);

                                // Update recent files menu
                                let files_vec: Vec<_> = state.get_recent_files().get_files().iter().cloned().collect();
                                crate::menu::update_recent_files_menu(hwnd, &files_vec);
                            }
                        });

                        log::info!("File loaded successfully: {:?}", file_path);
                        true
                    }
                    Err(e) => {
                        log::error!("Failed to read file: {}", e);
                        show_error(hwnd, "File Open Error", &format!("Failed to open file: {}", e));
                        false
                    }
                }
            } else {
                log::info!("File Open dialog cancelled");
                false
            }
        }

        CommandId::FileSave => {
            log::info!("Opening File Save dialog");
            if let Some(file_path) = show_save_dialog(hwnd, Some("untitled.txt")) {
                log::info!("Saving file: {:?}", file_path);
                // Get encoding and line ending from state
                let (encoding, line_ending) = crate::global_state::read_state(|state| {
                    (state.current_encoding, state.current_line_ending)
                });

                // Get text from editor
                let save_result = unsafe {
                    use windows::Win32::UI::WindowsAndMessaging::WM_GETTEXT;
                    let mut buffer = vec![0u16; 65536]; // 64KB buffer
                    let len = SendMessageW(
                        editor_hwnd,
                        WM_GETTEXT,
                        WPARAM(buffer.len()),
                        LPARAM(buffer.as_mut_ptr() as isize),
                    );

                    if len.0 > 0 {
                        let text = String::from_utf16_lossy(&buffer[..len.0 as usize]);

                        // Convert line endings
                        let text_with_line_endings = crate::encoding::LineEnding::convert(&text, line_ending);

                        // Write file with encoding
                        match crate::encoding::Encoding::write_file_with_encoding(&file_path, &text_with_line_endings, encoding) {
                            Ok(_) => {
                                log::info!("File saved successfully: {:?} (encoding: {:?}, line ending: {:?})", file_path, encoding, line_ending);
                                show_message(
                                    hwnd,
                                    "File Saved",
                                    &format!("File saved: {}", file_path.display()),
                                );
                                true
                            }
                            Err(e) => {
                                log::error!("Failed to write file: {}", e);
                                show_error(
                                    hwnd,
                                    "File Save Error",
                                    &format!("Failed to save file: {}", e),
                                );
                                false
                            }
                        }
                    } else {
                        log::warn!("No text to save");
                        false
                    }
                };

                if save_result {
                    // Update state after successful save
                    crate::global_state::with_state(|state| {
                        state.set_current_file(Some(file_path.clone()));
                        state.set_dirty(false);
                    });

                    // Update UI
                    crate::global_state::read_state(|state| {
                        unsafe {
                            crate::window_updates::update_window_title(hwnd, state);

                            // Update recent files menu
                            let files_vec: Vec<_> = state.get_recent_files().get_files().iter().cloned().collect();
                            crate::menu::update_recent_files_menu(hwnd, &files_vec);
                        }
                    });
                }

                save_result
            } else {
                log::info!("File Save dialog cancelled");
                false
            }
        }

        CommandId::FileSaveAs => {
            log::info!("Opening Save As dialog");
            if let Some(file_path) = show_save_dialog(hwnd, None) {
                log::info!("Saving file as: {:?}", file_path);
                // Get encoding and line ending from state
                let (encoding, line_ending) = crate::global_state::read_state(|state| {
                    (state.current_encoding, state.current_line_ending)
                });

                // Get text from editor
                let save_result = unsafe {
                    use windows::Win32::UI::WindowsAndMessaging::WM_GETTEXT;
                    let mut buffer = vec![0u16; 65536]; // 64KB buffer
                    let len = SendMessageW(
                        editor_hwnd,
                        WM_GETTEXT,
                        WPARAM(buffer.len()),
                        LPARAM(buffer.as_mut_ptr() as isize),
                    );

                    if len.0 > 0 {
                        let text = String::from_utf16_lossy(&buffer[..len.0 as usize]);

                        // Convert line endings
                        let text_with_line_endings = crate::encoding::LineEnding::convert(&text, line_ending);

                        // Write file with encoding
                        match crate::encoding::Encoding::write_file_with_encoding(&file_path, &text_with_line_endings, encoding) {
                            Ok(_) => {
                                log::info!("File saved successfully: {:?} (encoding: {:?}, line ending: {:?})", file_path, encoding, line_ending);
                                show_message(
                                    hwnd,
                                    "File Saved",
                                    &format!("File saved: {}", file_path.display()),
                                );
                                true
                            }
                            Err(e) => {
                                log::error!("Failed to write file: {}", e);
                                show_error(
                                    hwnd,
                                    "File Save Error",
                                    &format!("Failed to save file: {}", e),
                                );
                                false
                            }
                        }
                    } else {
                        log::warn!("No text to save");
                        false
                    }
                };

                if save_result {
                    // Update state after successful save
                    crate::global_state::with_state(|state| {
                        state.set_current_file(Some(file_path.clone()));
                        state.set_dirty(false);
                    });

                    // Update UI
                    crate::global_state::read_state(|state| {
                        unsafe {
                            crate::window_updates::update_window_title(hwnd, state);

                            // Update recent files menu
                            let files_vec: Vec<_> = state.get_recent_files().get_files().iter().cloned().collect();
                            crate::menu::update_recent_files_menu(hwnd, &files_vec);
                        }
                    });
                }

                save_result
            } else {
                log::info!("Save As dialog cancelled");
                false
            }
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
            log::info!("Opening Replace dialog");
            let dialog_hwnd = unsafe { crate::replace_dialog::create_replace_dialog(hwnd) };
            if dialog_hwnd.0 != 0 {
                log::info!("Replace dialog created");
            }
            true
        }

        CommandId::SearchGoToLine => {
            log::info!("Opening Go To Line dialog");
            let dialog_hwnd = unsafe { crate::goto_dialog::create_goto_dialog(hwnd) };
            if dialog_hwnd.0 != 0 {
                log::info!("Go To Line dialog created");
            }
            true
        }

        // View commands
        CommandId::ViewWordWrap => {
            log::info!("Toggling word wrap");

            // Toggle word wrap in state
            crate::global_state::with_state(|state| {
                state.toggle_word_wrap();
                let enabled = state.word_wrap_enabled;

                // Apply word wrap to editor
                unsafe {
                    use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_STYLE};
                    use windows::Win32::UI::WindowsAndMessaging::WS_HSCROLL;

                    let current_style = GetWindowLongPtrW(editor_hwnd, GWL_STYLE);

                    if enabled {
                        // Remove horizontal scroll to enable word wrap
                        let new_style = current_style & !(WS_HSCROLL.0 as isize);
                        SetWindowLongPtrW(editor_hwnd, GWL_STYLE, new_style);
                    } else {
                        // Add horizontal scroll to disable word wrap
                        let new_style = current_style | (WS_HSCROLL.0 as isize);
                        SetWindowLongPtrW(editor_hwnd, GWL_STYLE, new_style);
                    }

                    // Force redraw
                    use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_FRAMECHANGED};
                    SetWindowPos(editor_hwnd, None, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED).ok();
                }

                log::info!("Word wrap {}", if enabled { "enabled" } else { "disabled" });
            });

            true
        }

        CommandId::ViewShowWhitespace => {
            show_message(
                hwnd,
                "Show Whitespace",
                "Show whitespace feature not yet implemented",
            );
            false
        }

        CommandId::ViewShowEol => {
            show_message(
                hwnd,
                "Show End of Line",
                "Show end of line feature not yet implemented",
            );
            false
        }

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

        // Handle recent file commands
        CommandId::Custom(id) if (45000..=45009).contains(&id) => {
            log::info!("Opening recent file with ID: {}", id);
            if let Some(file_path) = crate::menu::open_recent_file(hwnd, id) {
                log::info!("Opening recent file: {:?}", file_path);
                // Reuse the FileOpen logic but with a specific file path
                match std::fs::read_to_string(&file_path) {
                    Ok(contents) => {
                        unsafe {
                            let text_wide: Vec<u16> = contents.encode_utf16().chain(std::iter::once(0)).collect();
                            SendMessageW(
                                editor_hwnd,
                                WM_SETTEXT,
                                WPARAM(0),
                                LPARAM(text_wide.as_ptr() as isize),
                            );
                        }

                        // Update state
                        crate::global_state::with_state(|state| {
                            state.set_current_file(Some(file_path.clone()));
                            state.set_dirty(false);
                        });

                        // Update UI
                        crate::global_state::read_state(|state| {
                            unsafe {
                                crate::window_updates::update_window_title(hwnd, state);
                                crate::window_updates::update_status_bar_position(hwnd, state);

                                // Update recent files menu
                                let files_vec: Vec<_> = state.get_recent_files().get_files().iter().cloned().collect();
                                crate::menu::update_recent_files_menu(hwnd, &files_vec);
                            }
                        });

                        log::info!("Recent file loaded successfully: {:?}", file_path);
                        true
                    }
                    Err(e) => {
                        log::error!("Failed to read recent file: {}", e);
                        show_error(hwnd, "File Open Error", &format!("Failed to open file: {}\n\nThe file may have been moved or deleted.", e));

                        // Remove from recent files list
                        crate::global_state::with_state(|state| {
                            state.recent_files.remove_file(&file_path);
                        });

                        // Update menu
                        crate::global_state::read_state(|state| {
                            unsafe {
                                let files_vec: Vec<_> = state.get_recent_files().get_files().iter().cloned().collect();
                                crate::menu::update_recent_files_menu(hwnd, &files_vec);
                            }
                        });

                        false
                    }
                }
            } else {
                log::warn!("Recent file not found for ID: {}", id);
                false
            }
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

/// Show an error message box
fn show_error(hwnd: HWND, title: &str, message: &str) {
    unsafe {
        let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let message_wide: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();

        MessageBoxW(
            hwnd,
            PCWSTR(message_wide.as_ptr()),
            PCWSTR(title_wide.as_ptr()),
            MB_OK | MB_ICONERROR,
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
