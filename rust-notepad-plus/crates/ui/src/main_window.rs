//! Main application window

use crate::Result;
use notepad_core::NotepadApp;
use notepad_editor::EditorView;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::w;

pub struct MainWindow {
    hwnd: HWND,
    editor: Option<EditorView>,
}

impl MainWindow {
    pub fn new(h_instance: HINSTANCE, _app: &NotepadApp) -> Result<Self> {
        log::info!("Creating main window");

        // Register window class
        let class_name = w!("NotepadPlusRust");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(Self::wnd_proc),
            hInstance: h_instance,
            lpszClassName: class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
            hbrBackground: unsafe { GetSysColorBrush(COLOR_WINDOW) },
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
                class_name,
                w!("Notepad++ Rust Edition"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                1024,
                768,
                None,
                None,
                h_instance,
                None,
            )?
        };

        log::info!("Main window created: {:?}", hwnd);

        Ok(Self {
            hwnd,
            editor: None,
        })
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
            let _ = UpdateWindow(self.hwnd);
        }
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_CREATE => {
                log::debug!("WM_CREATE");
                LRESULT(0)
            }
            WM_DESTROY => {
                log::info!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SIZE => {
                log::debug!("WM_SIZE");
                LRESULT(0)
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);
                EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
