//! Menu management for the application

use notepad_core::CommandId;
use std::path::PathBuf;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreateMenu, CreatePopupMenu, SetMenu, GetMenu, GetSubMenu, DeleteMenu, InsertMenuW,
    HMENU, MF_BYPOSITION, MF_POPUP, MF_SEPARATOR, MF_STRING,
};

/// Create the main menu bar
pub fn create_main_menu(hwnd: HWND) -> Result<HMENU, windows::core::Error> {
    unsafe {
        let menu_bar = CreateMenu()?;

        // File menu
        let file_menu = CreatePopupMenu()?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FileNew.to_menu_id() as usize, w!("&New\tCtrl+N"))?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FileOpen.to_menu_id() as usize, w!("&Open...\tCtrl+O"))?;
        AppendMenuW(file_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FileSave.to_menu_id() as usize, w!("&Save\tCtrl+S"))?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FileSaveAs.to_menu_id() as usize, w!("Save &As..."))?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FileSaveAll.to_menu_id() as usize, w!("Sa&ve All\tCtrl+Shift+S"))?;
        AppendMenuW(file_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FileClose.to_menu_id() as usize, w!("&Close\tCtrl+W"))?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FileCloseAll.to_menu_id() as usize, w!("Clos&e All"))?;
        AppendMenuW(file_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FilePrint.to_menu_id() as usize, w!("&Print...\tCtrl+P"))?;
        AppendMenuW(file_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(file_menu, MF_STRING, CommandId::FileExit.to_menu_id() as usize, w!("E&xit\tAlt+F4"))?;

        // Edit menu
        let edit_menu = CreatePopupMenu()?;
        AppendMenuW(edit_menu, MF_STRING, CommandId::EditUndo.to_menu_id() as usize, w!("&Undo\tCtrl+Z"))?;
        AppendMenuW(edit_menu, MF_STRING, CommandId::EditRedo.to_menu_id() as usize, w!("&Redo\tCtrl+Y"))?;
        AppendMenuW(edit_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(edit_menu, MF_STRING, CommandId::EditCut.to_menu_id() as usize, w!("Cu&t\tCtrl+X"))?;
        AppendMenuW(edit_menu, MF_STRING, CommandId::EditCopy.to_menu_id() as usize, w!("&Copy\tCtrl+C"))?;
        AppendMenuW(edit_menu, MF_STRING, CommandId::EditPaste.to_menu_id() as usize, w!("&Paste\tCtrl+V"))?;
        AppendMenuW(edit_menu, MF_STRING, CommandId::EditDelete.to_menu_id() as usize, w!("&Delete\tDel"))?;
        AppendMenuW(edit_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(edit_menu, MF_STRING, CommandId::EditSelectAll.to_menu_id() as usize, w!("Select &All\tCtrl+A"))?;

        // Search menu
        let search_menu = CreatePopupMenu()?;
        AppendMenuW(search_menu, MF_STRING, CommandId::SearchFind.to_menu_id() as usize, w!("&Find...\tCtrl+F"))?;
        AppendMenuW(search_menu, MF_STRING, CommandId::SearchFindNext.to_menu_id() as usize, w!("Find &Next\tF3"))?;
        AppendMenuW(search_menu, MF_STRING, CommandId::SearchFindPrevious.to_menu_id() as usize, w!("Find &Previous\tShift+F3"))?;
        AppendMenuW(search_menu, MF_STRING, CommandId::SearchReplace.to_menu_id() as usize, w!("&Replace...\tCtrl+H"))?;
        AppendMenuW(search_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(search_menu, MF_STRING, CommandId::SearchGoToLine.to_menu_id() as usize, w!("&Go to Line...\tCtrl+G"))?;

        // View menu
        let view_menu = CreatePopupMenu()?;
        AppendMenuW(view_menu, MF_STRING, CommandId::ViewZoomIn.to_menu_id() as usize, w!("Zoom &In\tCtrl++"))?;
        AppendMenuW(view_menu, MF_STRING, CommandId::ViewZoomOut.to_menu_id() as usize, w!("Zoom &Out\tCtrl+-"))?;
        AppendMenuW(view_menu, MF_STRING, CommandId::ViewZoomRestore.to_menu_id() as usize, w!("&Restore Default Zoom\tCtrl+0"))?;
        AppendMenuW(view_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(view_menu, MF_STRING, CommandId::ViewWordWrap.to_menu_id() as usize, w!("&Word wrap"))?;
        AppendMenuW(view_menu, MF_STRING, CommandId::ViewShowWhitespace.to_menu_id() as usize, w!("Show &Whitespace and TAB"))?;
        AppendMenuW(view_menu, MF_STRING, CommandId::ViewShowEol.to_menu_id() as usize, w!("Show End of &Line"))?;
        AppendMenuW(view_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuW(view_menu, MF_STRING, CommandId::ViewFullScreen.to_menu_id() as usize, w!("&Full Screen\tF11"))?;

        // Help menu
        let help_menu = CreatePopupMenu()?;
        AppendMenuW(help_menu, MF_STRING, CommandId::HelpAbout.to_menu_id() as usize, w!("&About Notepad++..."))?;

        // Add menus to menu bar
        AppendMenuW(menu_bar, MF_POPUP, file_menu.0 as usize, w!("&File"))?;
        AppendMenuW(menu_bar, MF_POPUP, edit_menu.0 as usize, w!("&Edit"))?;
        AppendMenuW(menu_bar, MF_POPUP, search_menu.0 as usize, w!("&Search"))?;
        AppendMenuW(menu_bar, MF_POPUP, view_menu.0 as usize, w!("&View"))?;
        AppendMenuW(menu_bar, MF_POPUP, help_menu.0 as usize, w!("&Help"))?;

        // Set the menu
        SetMenu(hwnd, menu_bar)?;

        Ok(menu_bar)
    }
}

/// Handle menu command
pub fn handle_menu_command(command_id: u32) -> Option<CommandId> {
    // Convert menu ID back to CommandId
    match command_id {
        41001 => Some(CommandId::FileNew),
        41002 => Some(CommandId::FileOpen),
        41006 => Some(CommandId::FileSave),
        41007 => Some(CommandId::FileSaveAs),
        41008 => Some(CommandId::FileSaveAll),
        41005 => Some(CommandId::FileClose),
        41009 => Some(CommandId::FileCloseAll),
        41012 => Some(CommandId::FilePrint),
        41004 => Some(CommandId::FileExit),

        42001 => Some(CommandId::EditUndo),
        42002 => Some(CommandId::EditRedo),
        42003 => Some(CommandId::EditCut),
        42004 => Some(CommandId::EditCopy),
        42005 => Some(CommandId::EditPaste),
        42006 => Some(CommandId::EditDelete),
        42010 => Some(CommandId::EditSelectAll),

        43001 => Some(CommandId::SearchFind),
        43002 => Some(CommandId::SearchFindNext),
        43004 => Some(CommandId::SearchReplace),
        43020 => Some(CommandId::SearchGoToLine),

        44001 => Some(CommandId::ViewFullScreen),
        44010 => Some(CommandId::ViewZoomIn),
        44011 => Some(CommandId::ViewZoomOut),
        44012 => Some(CommandId::ViewZoomRestore),
        44020 => Some(CommandId::ViewWordWrap),
        44021 => Some(CommandId::ViewShowWhitespace),
        44022 => Some(CommandId::ViewShowEol),

        // Recent files: 45000-45009
        45000..=45009 => Some(CommandId::Custom(command_id)),

        _ => None,
    }
}

/// Update the File menu with recent files
pub fn update_recent_files_menu(hwnd: HWND, recent_files: &[PathBuf]) {
    unsafe {
        let menu_bar = GetMenu(hwnd);
        if menu_bar.0 == 0 {
            return;
        }

        // Get the File menu (index 0)
        let file_menu = GetSubMenu(menu_bar, 0);
        if file_menu.0 == 0 {
            return;
        }

        // Remove existing recent files items (positions 9-19, counting from Print separator)
        // File menu structure: New, Open, |, Save, SaveAs, SaveAll, |, Close, CloseAll, |, Print, |, [Recent Files], |, Exit
        // After Print separator is position 11, before Exit separator

        // Find the position before Exit (we'll insert recent files before the last separator)
        // For simplicity, we'll add recent files after Print separator at position 12

        // First, remove any existing recent files menu items (IDs 45000-45009)
        for id in 45000..=45009 {
            let mut pos = 0;
            while pos < 50 {
                // Try to remove - if it doesn't exist, it will fail silently
                if DeleteMenu(file_menu, id, MF_BYPOSITION).is_err() {
                    break;
                }
                pos += 1;
            }
        }

        // If we have recent files, add them
        if !recent_files.is_empty() {
            // Insert separator before recent files (position 12, after Print separator)
            let separator_pos = 12;

            // Add each recent file
            for (i, path) in recent_files.iter().enumerate() {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown");

                let menu_text = format!("&{} {}", i + 1, filename);
                let menu_text_wide: Vec<u16> = menu_text.encode_utf16().chain(std::iter::once(0)).collect();

                let menu_id = 45000 + i as u32;
                InsertMenuW(
                    file_menu,
                    separator_pos + i as u32,
                    MF_BYPOSITION | MF_STRING,
                    menu_id as usize,
                    PCWSTR(menu_text_wide.as_ptr()),
                ).ok();
            }

            // Add separator after recent files
            InsertMenuW(
                file_menu,
                separator_pos + recent_files.len() as u32,
                MF_BYPOSITION | MF_SEPARATOR,
                0,
                None,
            ).ok();
        }

        // Force menu redraw
        use windows::Win32::UI::WindowsAndMessaging::DrawMenuBar;
        DrawMenuBar(hwnd).ok();
    }
}

/// Handle opening a recent file
pub fn open_recent_file(_hwnd: HWND, command_id: u32) -> Option<PathBuf> {
    let file_index = (command_id - 45000) as usize;

    crate::global_state::read_state(|state| {
        let files = state.get_recent_files().get_files();
        if file_index < files.len() {
            Some(files[file_index].clone())
        } else {
            None
        }
    })
}
