//! Search engine implementation

use crate::{SearchOptions, SearchResult};
use anyhow::Result;
use regex::Regex;

pub struct SearchEngine {
    options: SearchOptions,
}

impl SearchEngine {
    pub fn new(options: SearchOptions) -> Self {
        Self { options }
    }

    /// Find next occurrence of pattern in text starting from start_pos
    pub fn find(&self, text: &str, pattern: &str, start_pos: usize) -> Result<Option<SearchResult>> {
        if pattern.is_empty() {
            return Ok(None);
        }

        let search_text = if start_pos < text.len() {
            &text[start_pos..]
        } else {
            return Ok(None);
        };

        if self.options.use_regex {
            self.find_regex(search_text, pattern, start_pos)
        } else {
            self.find_literal(search_text, pattern, start_pos)
        }
    }

    /// Find all occurrences of pattern in text
    pub fn find_all(&self, text: &str, pattern: &str) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let mut pos = 0;

        while let Some(result) = self.find(text, pattern, pos)? {
            pos = result.end;
            results.push(result);

            if pos >= text.len() {
                break;
            }
        }

        Ok(results)
    }

    /// Replace all occurrences of pattern with replacement
    pub fn replace(&self, text: &str, pattern: &str, replacement: &str) -> Result<String> {
        if self.options.use_regex {
            self.replace_regex(text, pattern, replacement)
        } else {
            self.replace_literal(text, pattern, replacement)
        }
    }

    /// Replace first occurrence of pattern with replacement
    pub fn replace_first(&self, text: &str, pattern: &str, replacement: &str) -> Result<String> {
        if let Some(result) = self.find(text, pattern, 0)? {
            let mut new_text = String::new();
            new_text.push_str(&text[..result.start]);
            new_text.push_str(replacement);
            new_text.push_str(&text[result.end..]);
            Ok(new_text)
        } else {
            Ok(text.to_string())
        }
    }

    fn find_literal(&self, text: &str, pattern: &str, offset: usize) -> Result<Option<SearchResult>> {
        let search_pattern = if self.options.case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };

        let search_text = if self.options.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        if let Some(pos) = search_text.find(&search_pattern) {
            let start = offset + pos;
            let end = start + pattern.len();
            Ok(Some(SearchResult {
                start,
                end,
                matched_text: text[pos..pos + pattern.len()].to_string(),
            }))
        } else {
            Ok(None)
        }
    }

    fn find_regex(&self, text: &str, pattern: &str, offset: usize) -> Result<Option<SearchResult>> {
        let regex_pattern = if self.options.case_sensitive {
            format!("(?-i){}", pattern)
        } else {
            format!("(?i){}", pattern)
        };

        let re = Regex::new(&regex_pattern)?;

        if let Some(mat) = re.find(text) {
            Ok(Some(SearchResult {
                start: offset + mat.start(),
                end: offset + mat.end(),
                matched_text: mat.as_str().to_string(),
            }))
        } else {
            Ok(None)
        }
    }

    fn replace_literal(&self, text: &str, pattern: &str, replacement: &str) -> Result<String> {
        if self.options.case_sensitive {
            Ok(text.replace(pattern, replacement))
        } else {
            // Case-insensitive replace
            let pattern_lower = pattern.to_lowercase();
            let text_lower = text.to_lowercase();
            let mut result = String::new();
            let mut last_end = 0;

            for (i, _) in text.char_indices() {
                if text_lower[i..].starts_with(&pattern_lower) {
                    result.push_str(&text[last_end..i]);
                    result.push_str(replacement);
                    last_end = i + pattern.len();
                }
            }
            result.push_str(&text[last_end..]);
            Ok(result)
        }
    }

    fn replace_regex(&self, text: &str, pattern: &str, replacement: &str) -> Result<String> {
        let regex_pattern = if self.options.case_sensitive {
            format!("(?-i){}", pattern)
        } else {
            format!("(?i){}", pattern)
        };

        let re = Regex::new(&regex_pattern)?;
        Ok(re.replace_all(text, replacement).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_search() {
        let engine = SearchEngine::new(SearchOptions {
            case_sensitive: false,
            ..Default::default()
        });

        let text = "Hello World, hello rust!";
        let results = engine.find_all(text, "hello").unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].start, 0);
        assert_eq!(results[1].start, 13);
    }

    #[test]
    fn test_case_sensitive_search() {
        let engine = SearchEngine::new(SearchOptions {
            case_sensitive: true,
            ..Default::default()
        });

        let text = "Hello World, hello rust!";
        let results = engine.find_all(text, "hello").unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].start, 13);
    }

    #[test]
    fn test_regex_search() {
        let engine = SearchEngine::new(SearchOptions {
            use_regex: true,
            case_sensitive: false,
            ..Default::default()
        });

        let text = "test123 and test456";
        let results = engine.find_all(text, r"test\d+").unwrap();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_replace() {
        let engine = SearchEngine::new(SearchOptions::default());

        let text = "foo bar foo baz";
        let result = engine.replace(text, "foo", "qux").unwrap();

        assert_eq!(result, "qux bar qux baz");
    }

    #[test]
    fn test_replace_first() {
        let engine = SearchEngine::new(SearchOptions::default());

        let text = "foo bar foo baz";
        let result = engine.replace_first(text, "foo", "qux").unwrap();

        assert_eq!(result, "qux bar foo baz");
    }
}
