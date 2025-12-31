//! File Open and Save dialogs using Win32 Common Dialogs

use std::path::PathBuf;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Controls::Dialogs::{
    GetOpenFileNameW, GetSaveFileNameW, OPENFILENAMEW, OFN_EXPLORER, OFN_FILEMUSTEXIST,
    OFN_HIDEREADONLY, OFN_OVERWRITEPROMPT, OFN_PATHMUSTEXIST,
};

/// Result type for file dialog operations
pub type FileDialogResult = Option<PathBuf>;

/// Show a File Open dialog and return the selected file path
pub fn show_open_dialog(parent: HWND) -> FileDialogResult {
    unsafe {
        // Buffer for the file path
        let mut file_buffer = vec![0u16; 260]; // MAX_PATH

        // Filter for text files
        let filter = encode_filter(&[
            ("Text Files", "*.txt;*.log;*.md;*.rs;*.cpp;*.h;*.c;*.js;*.py;*.java"),
            ("All Files", "*.*"),
        ]);

        let mut ofn = OPENFILENAMEW {
            lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
            hwndOwner: parent,
            lpstrFilter: PCWSTR(filter.as_ptr()),
            lpstrFile: windows::core::PWSTR(file_buffer.as_mut_ptr()),
            nMaxFile: file_buffer.len() as u32,
            lpstrTitle: to_wide_null("Open File"),
            Flags: OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_HIDEREADONLY | OFN_EXPLORER,
            ..Default::default()
        };

        if GetOpenFileNameW(&mut ofn).as_bool() {
            // Find the null terminator
            let len = file_buffer.iter().position(|&c| c == 0).unwrap_or(0);
            if len > 0 {
                let path_str = String::from_utf16_lossy(&file_buffer[..len]);
                return Some(PathBuf::from(path_str));
            }
        }

        None
    }
}

/// Show a File Save dialog and return the selected file path
pub fn show_save_dialog(parent: HWND, default_filename: Option<&str>) -> FileDialogResult {
    unsafe {
        // Buffer for the file path
        let mut file_buffer = vec![0u16; 260]; // MAX_PATH

        // If a default filename is provided, copy it to the buffer
        if let Some(filename) = default_filename {
            let filename_wide: Vec<u16> = filename.encode_utf16().collect();
            let copy_len = filename_wide.len().min(file_buffer.len() - 1);
            file_buffer[..copy_len].copy_from_slice(&filename_wide[..copy_len]);
        }

        // Filter for text files
        let filter = encode_filter(&[
            ("Text Files", "*.txt;*.log;*.md;*.rs;*.cpp;*.h;*.c;*.js;*.py;*.java"),
            ("All Files", "*.*"),
        ]);

        let mut ofn = OPENFILENAMEW {
            lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
            hwndOwner: parent,
            lpstrFilter: PCWSTR(filter.as_ptr()),
            lpstrFile: windows::core::PWSTR(file_buffer.as_mut_ptr()),
            nMaxFile: file_buffer.len() as u32,
            lpstrTitle: to_wide_null("Save File"),
            lpstrDefExt: to_wide_null("txt"),
            Flags: OFN_PATHMUSTEXIST | OFN_OVERWRITEPROMPT | OFN_HIDEREADONLY | OFN_EXPLORER,
            ..Default::default()
        };

        if GetSaveFileNameW(&mut ofn).as_bool() {
            // Find the null terminator
            let len = file_buffer.iter().position(|&c| c == 0).unwrap_or(0);
            if len > 0 {
                let path_str = String::from_utf16_lossy(&file_buffer[..len]);
                return Some(PathBuf::from(path_str));
            }
        }

        None
    }
}

/// Helper function to encode a filter string
/// Format: "Text Files\0*.txt\0All Files\0*.*\0\0"
fn encode_filter(filters: &[(&str, &str)]) -> Vec<u16> {
    let mut result = Vec::new();

    for (name, pattern) in filters {
        // Add name
        result.extend(name.encode_utf16());
        result.push(0);

        // Add pattern
        result.extend(pattern.encode_utf16());
        result.push(0);
    }

    // Double null terminator
    result.push(0);

    result
}

/// Helper function to convert string to wide null-terminated PCWSTR
fn to_wide_null(s: &str) -> PCWSTR {
    let wide: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    PCWSTR(wide.as_ptr())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_filter() {
        let filter = encode_filter(&[("Text Files", "*.txt"), ("All Files", "*.*")]);

        // Should contain: "Text Files\0*.txt\0All Files\0*.*\0\0"
        assert!(filter.len() > 0);
        assert_eq!(filter.last(), Some(&0));
    }
}
