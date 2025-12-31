//! Character encoding example
//!
//! Run with: cargo run --example encoding_demo

use notepad_core::FileManager;
use notepad_core::app::EncodingType;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("=== Notepad++ Rust Edition - Encoding Example ===\n");

    let file_manager = FileManager::new()?;
    let temp_dir = std::env::temp_dir();

    // Test content with various characters
    let test_content = "Hello, World! 你好世界! Привет мир! مرحبا بالعالم";

    println!("Test content: {}\n", test_content);

    // Example 1: UTF-8 without BOM
    println!("--- Example 1: UTF-8 (no BOM) ---");
    let utf8_file = temp_dir.join("test_utf8.txt");
    test_encoding(&file_manager, &utf8_file, test_content, EncodingType::Utf8)?;

    // Example 2: UTF-8 with BOM
    println!("\n--- Example 2: UTF-8 with BOM ---");
    let utf8_bom_file = temp_dir.join("test_utf8_bom.txt");
    test_encoding(&file_manager, &utf8_bom_file, test_content, EncodingType::Utf8Bom)?;

    // Example 3: UTF-16 LE with BOM
    println!("\n--- Example 3: UTF-16 LE with BOM ---");
    let utf16le_file = temp_dir.join("test_utf16le.txt");
    test_encoding(&file_manager, &utf16le_file, test_content, EncodingType::Utf16LeBom)?;

    // Example 4: UTF-16 BE with BOM
    println!("\n--- Example 4: UTF-16 BE with BOM ---");
    let utf16be_file = temp_dir.join("test_utf16be.txt");
    test_encoding(&file_manager, &utf16be_file, test_content, EncodingType::Utf16BeBom)?;

    // Clean up
    let _ = std::fs::remove_file(utf8_file);
    let _ = std::fs::remove_file(utf8_bom_file);
    let _ = std::fs::remove_file(utf16le_file);
    let _ = std::fs::remove_file(utf16be_file);

    println!("\n✓ Example completed successfully!");

    Ok(())
}

fn test_encoding(
    file_manager: &FileManager,
    path: &PathBuf,
    content: &str,
    encoding: EncodingType,
) -> anyhow::Result<()> {
    println!("Encoding type: {:?}", encoding);

    // Create a buffer with the content
    let buffer = notepad_core::Buffer::from_file(path.clone(), content.to_string(), encoding);

    // Save the buffer
    file_manager.save_buffer(&buffer)?;
    println!("✓ Saved file: {:?}", path);

    // Read it back
    let loaded_buffer = file_manager.load_file(path.clone())?;
    println!("✓ Loaded file");
    println!("  Detected encoding: {:?}", loaded_buffer.encoding());
    println!("  Content matches: {}", loaded_buffer.content() == content);

    // Show file size
    let metadata = std::fs::metadata(path)?;
    println!("  File size: {} bytes", metadata.len());

    Ok(())
}
