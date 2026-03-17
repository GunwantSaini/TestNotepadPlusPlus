//! Basic usage example demonstrating core Notepad++ functionality
//!
//! Run with: cargo run --example basic_usage

use notepad_core::NotepadApp;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("=== Notepad++ Rust Edition - Basic Usage Example ===\n");

    // Create the application
    let app = NotepadApp::new()?;
    println!("✓ Application initialized");

    // Create a new empty buffer
    println!("\n--- Creating new buffer ---");
    // Since we don't have a direct API for this in the current implementation,
    // we'll demonstrate file loading

    // Create a temporary file for demonstration
    let temp_file = std::env::temp_dir().join("notepad_example.txt");
    std::fs::write(&temp_file, "Hello, Notepad++ Rust!\nThis is a test file.\n")?;
    println!("✓ Created temp file: {:?}", temp_file);

    // Open the file
    println!("\n--- Opening file ---");
    let buffer_id = app.open_file(temp_file.clone())?;
    println!("✓ Opened file with buffer ID: {:?}", buffer_id);

    // Get the buffer
    if let Some(buffer) = app.get_buffer(buffer_id) {
        println!("\nBuffer Info:");
        println!("  - Display name: {}", buffer.display_name());
        println!("  - Path: {:?}", buffer.file_path());
        println!("  - Encoding: {:?}", buffer.encoding());
        println!("  - EOL format: {:?}", buffer.eol_format());
        println!("  - Is dirty: {}", buffer.is_dirty());
        println!("  - Content length: {} bytes", buffer.content().len());
        println!("\nContent:");
        println!("{}", buffer.content());
    }

    // List all open buffers
    println!("\n--- All open buffers ---");
    let all_buffers = app.get_all_buffers();
    println!("Total open buffers: {}", all_buffers.len());

    // Save the buffer
    println!("\n--- Saving buffer ---");
    app.save_buffer(buffer_id)?;
    println!("✓ Buffer saved successfully");

    // Close the buffer
    println!("\n--- Closing buffer ---");
    app.close_buffer(buffer_id)?;
    println!("✓ Buffer closed");

    // Clean up
    std::fs::remove_file(temp_file)?;
    println!("\n✓ Example completed successfully!");

    Ok(())
}
