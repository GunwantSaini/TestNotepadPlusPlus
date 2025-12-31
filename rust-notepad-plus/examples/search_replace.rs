//! Search and replace example
//!
//! Run with: cargo run --example search_replace

use notepad_search::{SearchEngine, SearchOptions};

fn main() -> anyhow::Result<()> {
    println!("=== Notepad++ Rust Edition - Search & Replace Example ===\n");

    let sample_text = r#"
fn main() {
    let name = "World";
    println!("Hello, {}!", name);
    println!("Hello, Rust!");
}
"#;

    println!("Sample text:");
    println!("{}", sample_text);
    println!("\n{}", "=".repeat(60));

    // Example 1: Simple literal search
    println!("\n--- Example 1: Literal Search ---");
    let engine = SearchEngine::new(SearchOptions {
        case_sensitive: false,
        ..Default::default()
    });

    let results = engine.find_all(sample_text, "hello")?;
    println!("Found {} occurrences of 'hello':", results.len());
    for (i, result) in results.iter().enumerate() {
        println!("  {}. Position {}-{}: '{}'",
                 i + 1, result.start, result.end, result.matched_text);
    }

    // Example 2: Case-sensitive search
    println!("\n--- Example 2: Case-Sensitive Search ---");
    let engine_cs = SearchEngine::new(SearchOptions {
        case_sensitive: true,
        ..Default::default()
    });

    let results_cs = engine_cs.find_all(sample_text, "Hello")?;
    println!("Found {} occurrences of 'Hello' (case-sensitive):", results_cs.len());
    for result in results_cs.iter() {
        println!("  Position {}-{}: '{}'",
                 result.start, result.end, result.matched_text);
    }

    // Example 3: Regex search
    println!("\n--- Example 3: Regex Search ---");
    let engine_regex = SearchEngine::new(SearchOptions {
        use_regex: true,
        case_sensitive: false,
        ..Default::default()
    });

    let results_regex = engine_regex.find_all(sample_text, r"println!\(.*?\)")?;
    println!("Found {} println! calls:", results_regex.len());
    for (i, result) in results_regex.iter().enumerate() {
        println!("  {}. '{}'", i + 1, result.matched_text);
    }

    // Example 4: Replace operation
    println!("\n--- Example 4: Replace All ---");
    let replaced = engine.replace(sample_text, "Hello", "Greetings")?;
    println!("After replacing 'Hello' with 'Greetings':");
    println!("{}", replaced);

    // Example 5: Replace first occurrence only
    println!("\n--- Example 5: Replace First ---");
    let replaced_first = engine.replace_first(sample_text, "Hello", "Hi")?;
    println!("After replacing first 'Hello' with 'Hi':");
    println!("{}", replaced_first);

    // Example 6: Regex replace
    println!("\n--- Example 6: Regex Replace ---");
    let engine_regex_replace = SearchEngine::new(SearchOptions {
        use_regex: true,
        ..Default::default()
    });

    let code = "let x = 42; let y = 100; let z = 200;";
    let replaced_regex = engine_regex_replace.replace(code, r"let (\w+)", "var $1")?;
    println!("Original: {}", code);
    println!("After regex replace 'let x' -> 'var x':");
    println!("Result:   {}", replaced_regex);

    println!("\n✓ Example completed successfully!");

    Ok(())
}
