//! Integration tests for search and replace functionality

use notepad_search::{SearchEngine, SearchOptions};

#[test]
fn test_basic_search() {
    let text = "Hello world!\nThis is a test.\nHello again!";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    let result = engine.find(text, "Hello", 0).expect("Search failed");
    assert!(result.is_some(), "Should find 'Hello'");

    let found = result.unwrap();
    assert_eq!(found.start, 0);
    assert_eq!(found.matched_text, "Hello");
}

#[test]
fn test_case_sensitive_search() {
    let text = "Hello HELLO hello";

    // Case insensitive (default)
    let options = SearchOptions {
        case_sensitive: false,
        ..Default::default()
    };
    let engine = SearchEngine::new(options);
    let result = engine.find(text, "hello", 0).expect("Search failed");
    assert!(result.is_some(), "Should find 'hello' case-insensitively");

    // Case sensitive
    let options = SearchOptions {
        case_sensitive: true,
        ..Default::default()
    };
    let engine = SearchEngine::new(options);
    let result = engine.find(text, "hello", 0).expect("Search failed");
    assert!(result.is_some(), "Should find exact 'hello'");
    assert_eq!(result.unwrap().start, 12); // Position of lowercase 'hello'
}

#[test]
fn test_regex_search() {
    let text = "Error: line 123\nWarning: line 456\nError: line 789";
    let options = SearchOptions {
        use_regex: true,
        ..Default::default()
    };
    let engine = SearchEngine::new(options);

    // Search for pattern "Error: line \d+"
    let result = engine.find(text, r"Error: line \d+", 0).expect("Search failed");
    assert!(result.is_some(), "Should find error pattern");

    let found = result.unwrap();
    assert_eq!(found.matched_text, "Error: line 123");
}

#[test]
fn test_replace_all() {
    let text = "foo bar foo baz foo";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    let result = engine.replace_all(text, "foo", "qux").expect("Replace failed");
    assert_eq!(result, "qux bar qux baz qux");
}

#[test]
fn test_replace_first() {
    let text = "foo bar foo baz foo";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    let (result, replaced) = engine.replace_first(text, "foo", "qux").expect("Replace failed");
    assert!(replaced, "Should have replaced first occurrence");
    assert_eq!(result, "qux bar foo baz foo");
}

#[test]
fn test_search_not_found() {
    let text = "Hello world!";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    let result = engine.find(text, "notfound", 0).expect("Search failed");
    assert!(result.is_none(), "Should not find non-existent text");
}

#[test]
fn test_multiple_search_occurrences() {
    let text = "apple banana apple cherry apple";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    // Find first occurrence
    let result1 = engine.find(text, "apple", 0).expect("Search failed");
    assert!(result1.is_some());
    assert_eq!(result1.as_ref().unwrap().start, 0);

    // Find second occurrence
    let result2 = engine.find(text, "apple", result1.unwrap().end).expect("Search failed");
    assert!(result2.is_some());
    assert_eq!(result2.as_ref().unwrap().start, 13);

    // Find third occurrence
    let result3 = engine.find(text, "apple", result2.unwrap().end).expect("Search failed");
    assert!(result3.is_some());
    assert_eq!(result3.as_ref().unwrap().start, 26);
}

#[test]
fn test_regex_replace() {
    let text = "Date: 2024-01-15, Time: 14:30";
    let options = SearchOptions {
        use_regex: true,
        ..Default::default()
    };
    let engine = SearchEngine::new(options);

    // Replace date format YYYY-MM-DD with DD/MM/YYYY
    let result = engine.replace_all(text, r"(\d{4})-(\d{2})-(\d{2})", "$3/$2/$1")
        .expect("Replace failed");
    assert_eq!(result, "Date: 15/01/2024, Time: 14:30");
}

#[test]
fn test_whole_word_search() {
    let text = "cat category cats";
    let options = SearchOptions {
        whole_word: true,
        ..Default::default()
    };
    let engine = SearchEngine::new(options);

    let result = engine.find(text, "cat", 0).expect("Search failed");
    assert!(result.is_some(), "Should find whole word 'cat'");
    assert_eq!(result.unwrap().start, 0);
    assert_eq!(result.unwrap().end, 3);
}

#[test]
fn test_empty_search_pattern() {
    let text = "Hello world!";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    let result = engine.find(text, "", 0).expect("Search failed");
    assert!(result.is_none(), "Empty pattern should not match");
}

#[test]
fn test_search_with_special_chars() {
    let text = "Price: $100.50 (USD)";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    let result = engine.find(text, "$100.50", 0).expect("Search failed");
    assert!(result.is_some(), "Should find text with special chars");
    assert_eq!(result.unwrap().matched_text, "$100.50");
}

#[test]
fn test_multiline_search() {
    let text = "Line 1\nLine 2\nLine 3";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    let result = engine.find(text, "Line 2", 0).expect("Search failed");
    assert!(result.is_some(), "Should find text across lines");
    assert_eq!(result.unwrap().matched_text, "Line 2");
}

#[test]
fn test_unicode_search() {
    let text = "Hello 世界! Rust 🦀 Programming";
    let options = SearchOptions::default();
    let engine = SearchEngine::new(options);

    // Search for Chinese characters
    let result = engine.find(text, "世界", 0).expect("Search failed");
    assert!(result.is_some(), "Should find Unicode characters");
    assert_eq!(result.unwrap().matched_text, "世界");

    // Search for emoji
    let result = engine.find(text, "🦀", 0).expect("Search failed");
    assert!(result.is_some(), "Should find emoji");
    assert_eq!(result.unwrap().matched_text, "🦀");
}
