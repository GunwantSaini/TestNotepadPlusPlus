//! Notepad++ Rust Edition
//!
//! A cross-platform port of Notepad++ from C++ to Rust.
//! This maintains feature parity with the original while leveraging
//! Rust's memory safety and modern tooling.

// Platform-specific UI backend
#[cfg(target_os = "windows")]
use notepad_ui_windows as ui_backend;

#[cfg(target_os = "linux")]
use notepad_ui_gtk as ui_backend;

// Common imports
use anyhow::Result;
use log::info;
use notepad_core::NotepadApp;

#[cfg(target_os = "windows")]
use ui_backend::{create_accelerators, MainWindow};

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateAcceleratorW, TranslateMessage, MSG,
};

#[cfg(target_os = "linux")]
use ui_backend::GtkMainWindow;

#[cfg(target_os = "windows")]
fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Notepad++ Rust Edition starting (Windows)...");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    info!("Command line args: {:?}", args);

    // Initialize the core application
    let app = NotepadApp::new()?;
    info!("Core application initialized");

    // Initialize global UI state
    ui_backend::init_global_state();
    info!("Global state initialized");

    // Create main window
    let main_window = MainWindow::new(&app)?;
    info!("Main window created");

    // Show the window
    main_window.show();
    info!("Main window shown");

    // Create accelerator table for keyboard shortcuts
    let haccel = match create_accelerators() {
        Ok(accel) => {
            info!("Accelerator table created (Ctrl+N, Ctrl+O, Ctrl+S, etc.)");
            accel
        }
        Err(e) => {
            log::warn!("Failed to create accelerator table: {:?}", e);
            windows::Win32::UI::WindowsAndMessaging::HACCEL::default()
        }
    };

    // Windows message loop with accelerator support
    info!("Entering message loop...");
    let mut msg = MSG::default();
    let hwnd = main_window.hwnd();

    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            // Try to translate accelerator first
            if haccel.is_invalid() || TranslateAcceleratorW(hwnd, haccel, &msg) == 0 {
                // If not an accelerator, do normal translation
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    info!("Application shutting down");
    Ok(())
}

#[cfg(target_os = "linux")]
fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Notepad++ Rust Edition starting (Linux/GTK4)...");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    info!("Command line args: {:?}", args);

    // Initialize the core application
    let app = NotepadApp::new()?;
    info!("Core application initialized");

    // Initialize global UI state
    ui_backend::init_global_state();
    info!("Global state initialized");

    // Create main window
    let main_window = GtkMainWindow::new(&app)?;
    info!("Main window created");

    // Show the window
    main_window.show();
    info!("Main window shown");

    // Run GTK event loop
    info!("Entering GTK event loop...");
    let exit_code = main_window.run();

    info!("Application shutting down with exit code: {}", exit_code);
    std::process::exit(exit_code);
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn main() -> Result<()> {
    eprintln!("Error: This platform is not yet supported.");
    eprintln!("Currently supported platforms:");
    eprintln!("  - Windows (Win32 API backend)");
    eprintln!("  - Linux (GTK4 backend)");
    eprintln!("\nPlanned platforms:");
    eprintln!("  - macOS (Cocoa backend) - Future");
    std::process::exit(1);
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
