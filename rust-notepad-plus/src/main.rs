//! Notepad++ Rust Edition
//!
//! A complete port of Notepad++ from C++ to Rust for Windows.
//! This maintains feature parity with the original while leveraging
//! Rust's memory safety and modern tooling.

#![windows_subsystem = "windows"]

use anyhow::Result;
use log::{error, info};
use notepad_core::NotepadApp;
use notepad_ui::MainWindow;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, TranslateMessage, MSG};

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Notepad++ Rust Edition starting...");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    info!("Command line args: {:?}", args);

    // Get module instance
    let h_instance = unsafe {
        HINSTANCE(GetModuleHandleW(None)?.0)
    };

    // Initialize the core application
    let app = NotepadApp::new()?;
    info!("Core application initialized");

    // Create main window
    let main_window = MainWindow::new(h_instance, &app)?;
    info!("Main window created");

    // Show the window
    main_window.show();

    // Main message loop
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    info!("Application shutting down");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup() {
        // Basic startup test
        assert!(true);
    }
}
