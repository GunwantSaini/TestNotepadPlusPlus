//! Main application window with full Win32 implementation

use crate::{editor_control::EditorControl, menu, statusbar::StatusBar, toolbar::Toolbar, Result};
use notepad_core::NotepadApp;
use notepad_editor::EditorView;
use std::sync::{Arc, Mutex};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, HBRUSH, PAINTSTRUCT};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, GetClientRect, LoadCursorW, PostQuitMessage, RegisterClassW,
    ShowWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, SW_SHOW, WNDCLASSW,
    WM_COMMAND, WM_CREATE, WM_DESTROY, WM_PAINT, WM_SIZE, WINDOW_EX_STYLE, WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
};
use windows::Win32::Graphics::Gdi::COLOR_WINDOW;

const WINDOW_CLASS_NAME: PCWSTR = w!("NotepadPlusRustWindow");

/// Main application window
pub struct MainWindow {
    hwnd: HWND,
    editor: Arc<Mutex<EditorView>>,
}

impl MainWindow {
    /// Create a new main window with full Win32 implementation
    pub fn new(_app: &NotepadApp) -> Result<Self> {
        log::info!("Creating main window with Win32 API");

        let editor = Arc::new(Mutex::new(EditorView::new()));

        // Get module handle
        let h_instance = unsafe { GetModuleHandleW(None).unwrap() };

        // Register window class
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance.into(),
            lpszClassName: WINDOW_CLASS_NAME,
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as isize),
            ..Default::default()
        };

        unsafe {
            if RegisterClassW(&wc) == 0 {
                return Err(crate::UiError::WindowCreationFailed);
            }
        }

        // Create main window
        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                WINDOW_CLASS_NAME,
                w!("Notepad++ Rust Edition"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                1024,
                768,
                None,
                None,
                h_instance,
                None,
            )
        };

        if hwnd.0 == 0 {
            return Err(crate::UiError::WindowCreationFailed);
        }

        log::info!("Main window created successfully: {:?}", hwnd);

        Ok(Self { hwnd, editor })
    }

    /// Show the window
    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
        log::info!("Main window shown");
    }

    /// Get window handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    /// Get editor view
    pub fn editor(&self) -> Arc<Mutex<EditorView>> {
        self.editor.clone()
    }
}

/// Window procedure callback
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            log::debug!("WM_CREATE received - creating UI components");

            // Get module handle
            let h_instance = match GetModuleHandleW(None) {
                Ok(h) => h.0 as isize,
                Err(e) => {
                    log::error!("Failed to get module handle: {:?}", e);
                    return LRESULT(-1);
                }
            };

            // Create menu bar
            match menu::create_main_menu(hwnd) {
                Ok(_) => log::info!("Menu created successfully"),
                Err(e) => log::error!("Failed to create menu: {:?}", e),
            }

            // Create toolbar
            match Toolbar::new(hwnd, h_instance) {
                Ok(_toolbar) => {
                    log::info!("Toolbar created successfully");
                    // Toolbar is created and displayed
                }
                Err(e) => log::error!("Failed to create toolbar: {:?}", e),
            }

            // Create status bar
            match StatusBar::new(hwnd, h_instance) {
                Ok(_statusbar) => {
                    log::info!("Status bar created successfully");
                    // Status bar is created and displayed
                }
                Err(e) => log::error!("Failed to create status bar: {:?}", e),
            }

            // Create editor control
            // Initial size will be set in WM_SIZE
            match EditorControl::new(hwnd, 0, 32, 800, 600) {
                Ok(_editor) => {
                    log::info!("Editor control created successfully");
                    // Editor is created and displayed
                }
                Err(e) => log::error!("Failed to create editor: {:?}", e),
            }

            LRESULT(0)
        }
        WM_DESTROY => {
            log::info!("WM_DESTROY received - posting quit message");
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let _hdc = BeginPaint(hwnd, &mut ps);

            // TODO: Paint editor content here

            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_SIZE => {
            log::debug!("WM_SIZE received");

            // Get client area dimensions
            let mut rect = windows::Win32::Foundation::RECT::default();
            if GetClientRect(hwnd, &mut rect).is_ok() {
                let width = rect.right - rect.left;
                let height = rect.bottom - rect.top;

                // Toolbar and status bar resize themselves automatically
                // We need to resize the editor control to fill the space between them

                const TOOLBAR_HEIGHT: i32 = 32;
                const STATUSBAR_HEIGHT: i32 = 24;

                let editor_y = TOOLBAR_HEIGHT;
                let editor_height = height - TOOLBAR_HEIGHT - STATUSBAR_HEIGHT;

                // Find and resize editor control
                // The editor is a child EDIT window
                use windows::Win32::UI::WindowsAndMessaging::FindWindowExW;
                let editor_hwnd = FindWindowExW(hwnd, None, w!("EDIT"), PCWSTR::null());

                if editor_hwnd.0 != 0 {
                    windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
                        editor_hwnd,
                        None,
                        0,
                        editor_y,
                        width,
                        editor_height.max(0),
                        windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER,
                    )
                    .ok();
                }

                log::debug!(
                    "Resized editor to {}x{} at y={}",
                    width,
                    editor_height,
                    editor_y
                );
            }

            // Let DefWindowProc handle default sizing for toolbar/statusbar
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        WM_COMMAND => {
            let command_id = (wparam.0 & 0xFFFF) as u32;
            log::debug!("WM_COMMAND received: {}", command_id);

            if let Some(cmd) = menu::handle_menu_command(command_id) {
                log::info!("Menu command: {:?}", cmd);

                // Handle specific commands
                match cmd {
                    notepad_core::CommandId::FileExit => {
                        PostQuitMessage(0);
                    }
                    _ => {
                        log::info!("Command not yet implemented: {:?}", cmd);
                    }
                }
            }
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
