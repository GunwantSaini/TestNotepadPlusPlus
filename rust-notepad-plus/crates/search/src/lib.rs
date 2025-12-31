//! Search and replace functionality

use anyhow::Result;

pub mod engine;
pub mod regex_search;

pub use engine::SearchEngine;

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub use_regex: bool,
    pub wrap_around: bool,
    pub search_backwards: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_word: false,
            use_regex: false,
            wrap_around: true,
            search_backwards: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub start: usize,
    pub end: usize,
    pub matched_text: String,
}
