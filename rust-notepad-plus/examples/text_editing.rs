//! Text editing with Ropey buffer example
//!
//! Run with: cargo run --example text_editing

use notepad_editor::TextBuffer;

fn main() {
    println!("=== Notepad++ Rust Edition - Text Editing Example ===\n");

    // Create a new empty buffer
    let mut buffer = TextBuffer::new();
    println!("Created empty buffer");
    println!("  Length: {} characters", buffer.len());
    println!("  Lines: {}", buffer.line_count());

    // Insert some text
    println!("\n--- Inserting text ---");
    buffer.insert(0, "Hello, World!\n").unwrap();
    buffer.insert(buffer.len(), "This is line 2.\n").unwrap();
    buffer.insert(buffer.len(), "And this is line 3.\n").unwrap();

    println!("After insertions:");
    println!("  Length: {} characters", buffer.len());
    println!("  Lines: {}", buffer.line_count());
    println!("\nContent:\n{}", buffer.get_text());

    // Get individual lines
    println!("\n--- Reading lines ---");
    for i in 0..buffer.line_count() {
        if let Some(line) = buffer.get_line(i) {
            println!("Line {}: {}", i + 1, line.trim_end());
        }
    }

    // Delete some text
    println!("\n--- Deleting text ---");
    let delete_start = 7; // After "Hello, "
    let delete_end = 13;  // Before "!\n"
    buffer.delete(delete_start, delete_end).unwrap();

    println!("After deleting 'World':");
    println!("{}", buffer.get_text());

    // Insert replacement
    println!("\n--- Inserting replacement ---");
    buffer.insert(delete_start, "Rust").unwrap();
    println!("After inserting 'Rust':");
    println!("{}", buffer.get_text());

    // Check modified status
    println!("\n--- Modified status ---");
    println!("Is modified: {}", buffer.is_modified());

    buffer.clear_modified();
    println!("After clearing: {}", buffer.is_modified());

    // Create from string
    println!("\n--- Creating buffer from string ---");
    let code_sample = r#"fn main() {
    println!("Hello, Rust!");
}
"#;

    let code_buffer = TextBuffer::from_string(code_sample.to_string());
    println!("Code buffer created:");
    println!("  Length: {} characters", code_buffer.len());
    println!("  Lines: {}", code_buffer.line_count());
    println!("\nContent:\n{}", code_buffer.get_text());

    println!("\n✓ Example completed successfully!");
}
