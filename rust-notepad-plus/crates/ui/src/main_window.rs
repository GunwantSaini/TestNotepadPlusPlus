//! Main application window

use crate::Result;
use notepad_core::NotepadApp;
use notepad_editor::EditorView;

// Placeholder window handle type
#[derive(Debug, Clone, Copy)]
pub struct WindowHandle(usize);

pub struct MainWindow {
    handle: WindowHandle,
    editor: Option<EditorView>,
}

impl MainWindow {
    /// Create a new main window
    ///
    /// Note: This is a simplified implementation for demonstration.
    /// Full Win32 window creation will be implemented later with proper
    /// windows-rs bindings.
    pub fn new(_app: &NotepadApp) -> Result<Self> {
        log::info!("Creating main window (stub implementation)");

        // TODO: Implement actual Win32 window creation
        // This requires:
        // 1. Register window class with RegisterClassW
        // 2. Create window with CreateWindowExW
        // 3. Set up window procedure for message handling

        Ok(Self {
            handle: WindowHandle(0),
            editor: Some(EditorView::new()),
        })
    }

    /// Show the window
    pub fn show(&self) {
        log::info!("Showing main window (stub)");
        // TODO: Call ShowWindow with SW_SHOW
    }

    /// Get window handle
    pub fn handle(&self) -> WindowHandle {
        self.handle
    }

    /// Get editor view
    pub fn editor(&self) -> Option<&EditorView> {
        self.editor.as_ref()
    }
}
