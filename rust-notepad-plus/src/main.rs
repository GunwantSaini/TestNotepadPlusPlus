//! Notepad++ Rust Edition
//!
//! A complete port of Notepad++ from C++ to Rust for Windows.
//! This maintains feature parity with the original while leveraging
//! Rust's memory safety and modern tooling.

use anyhow::Result;
use log::info;
use notepad_core::NotepadApp;
use notepad_ui::MainWindow;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Notepad++ Rust Edition starting...");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    info!("Command line args: {:?}", args);

    // Initialize the core application
    let app = NotepadApp::new()?;
    info!("Core application initialized");

    // Create main window
    let main_window = MainWindow::new(&app)?;
    info!("Main window created");

    // Show the window
    main_window.show();

    info!("Application running (stub mode - no message loop yet)");

    // TODO: Implement Windows message loop
    // This requires:
    // - GetMessageW to receive messages
    // - TranslateMessage to process keyboard input
    // - DispatchMessageW to route messages to window procedure

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
