//! Tokenizer for syntax highlighting
//!
//! Provides token-based syntax highlighting for various programming languages

use std::ops::Range;

/// Token type for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// Plain text or unknown
    Text,
    /// Keywords (if, else, fn, class, etc.)
    Keyword,
    /// Built-in types (int, str, bool, etc.)
    Type,
    /// String literals
    String,
    /// Character literals
    Char,
    /// Numeric literals
    Number,
    /// Comments (line and block)
    Comment,
    /// Operators (+, -, *, /, etc.)
    Operator,
    /// Punctuation (brackets, braces, semicolons)
    Punctuation,
    /// Function names
    Function,
    /// Variable/identifier names
    Identifier,
    /// Preprocessor directives (#include, etc.)
    Preprocessor,
    /// Documentation comments
    DocComment,
    /// Regular expressions
    Regex,
    /// HTML/XML tags
    Tag,
    /// HTML/XML attributes
    Attribute,
    /// Constants (true, false, null, etc.)
    Constant,
    /// Macros (Rust macros, C macros)
    Macro,
    /// Labels (goto labels, etc.)
    Label,
}

/// A highlighted token with position
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// Token type for styling
    pub token_type: TokenType,
    /// Range in the source text (byte positions)
    pub range: Range<usize>,
    /// The actual text of this token
    pub text: String,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, range: Range<usize>, text: String) -> Self {
        Self {
            token_type,
            range,
            text,
        }
    }

    /// Get the length of the token
    pub fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    /// Check if token is empty
    pub fn is_empty(&self) -> bool {
        self.range.start == self.range.end
    }
}

/// Trait for language-specific tokenizers
pub trait Tokenizer: Send + Sync {
    /// Tokenize a line of text
    ///
    /// Returns a vector of tokens for the given line.
    /// The byte offset is the position of this line in the full document.
    fn tokenize_line(&self, line: &str, byte_offset: usize) -> Vec<Token>;

    /// Tokenize multiple lines
    fn tokenize_lines(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut byte_offset = 0;

        for line in text.lines() {
            tokens.extend(self.tokenize_line(line, byte_offset));
            byte_offset += line.len() + 1; // +1 for newline
        }

        tokens
    }

    /// Get the name of this tokenizer
    fn name(&self) -> &str;
}

/// Simple regex-based tokenizer for basic syntax highlighting
pub struct RegexTokenizer {
    name: String,
    keywords: Vec<String>,
    types: Vec<String>,
    constants: Vec<String>,
}

impl RegexTokenizer {
    /// Create a new regex tokenizer
    pub fn new(
        name: String,
        keywords: Vec<String>,
        types: Vec<String>,
        constants: Vec<String>,
    ) -> Self {
        Self {
            name,
            keywords,
            types,
            constants,
        }
    }

    /// Check if a word is a keyword
    fn is_keyword(&self, word: &str) -> bool {
        self.keywords.iter().any(|k| k == word)
    }

    /// Check if a word is a type
    fn is_type(&self, word: &str) -> bool {
        self.types.iter().any(|t| t == word)
    }

    /// Check if a word is a constant
    fn is_constant(&self, word: &str) -> bool {
        self.constants.iter().any(|c| c == word)
    }

    /// Tokenize a word (identifier, keyword, type, or constant)
    fn tokenize_word(&self, word: &str, start: usize, byte_offset: usize) -> Token {
        let token_type = if self.is_keyword(word) {
            TokenType::Keyword
        } else if self.is_type(word) {
            TokenType::Type
        } else if self.is_constant(word) {
            TokenType::Constant
        } else if word.chars().next().map_or(false, |c| c.is_uppercase()) {
            // Capitalized words are often types or constants
            TokenType::Type
        } else {
            TokenType::Identifier
        };

        Token::new(
            token_type,
            byte_offset + start..byte_offset + start + word.len(),
            word.to_string(),
        )
    }
}

impl Tokenizer for RegexTokenizer {
    fn tokenize_line(&self, line: &str, byte_offset: usize) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = line.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            match ch {
                // Line comments (//)
                '/' if chars.peek().map(|(_, c)| c) == Some(&'/') => {
                    let comment = &line[i..];
                    tokens.push(Token::new(
                        TokenType::Comment,
                        byte_offset + i..byte_offset + line.len(),
                        comment.to_string(),
                    ));
                    break; // Rest of line is comment
                }

                // String literals
                '"' => {
                    let mut end = i + 1;
                    let mut escaped = false;

                    while end < line.len() {
                        match line.chars().nth(end) {
                            Some('\\') if !escaped => escaped = true,
                            Some('"') if !escaped => {
                                end += 1;
                                break;
                            }
                            _ => escaped = false,
                        }
                        end += 1;
                    }

                    tokens.push(Token::new(
                        TokenType::String,
                        byte_offset + i..byte_offset + end,
                        line[i..end].to_string(),
                    ));

                    // Skip ahead
                    while chars.peek().map(|(idx, _)| *idx < end).unwrap_or(false) {
                        chars.next();
                    }
                }

                // Character literals
                '\'' => {
                    let mut end = i + 1;
                    if line.chars().nth(end) == Some('\\') {
                        end += 2; // Escaped char
                    } else {
                        end += 1; // Single char
                    }
                    if line.chars().nth(end) == Some('\'') {
                        end += 1;
                    }

                    tokens.push(Token::new(
                        TokenType::Char,
                        byte_offset + i..byte_offset + end,
                        line[i..end].to_string(),
                    ));

                    // Skip ahead
                    while chars.peek().map(|(idx, _)| *idx < end).unwrap_or(false) {
                        chars.next();
                    }
                }

                // Numbers
                ch if ch.is_ascii_digit() => {
                    let start = i;
                    let mut end = i + 1;

                    while end < line.len() {
                        let c = line.chars().nth(end).unwrap();
                        if c.is_ascii_alphanumeric() || c == '.' || c == '_' {
                            end += 1;
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::new(
                        TokenType::Number,
                        byte_offset + start..byte_offset + end,
                        line[start..end].to_string(),
                    ));

                    // Skip ahead
                    while chars.peek().map(|(idx, _)| *idx < end).unwrap_or(false) {
                        chars.next();
                    }
                }

                // Identifiers and keywords
                ch if ch.is_alphabetic() || ch == '_' => {
                    let start = i;
                    let mut end = i + 1;

                    while end < line.len() {
                        let c = line.chars().nth(end).unwrap();
                        if c.is_alphanumeric() || c == '_' {
                            end += 1;
                        } else {
                            break;
                        }
                    }

                    let word = &line[start..end];
                    tokens.push(self.tokenize_word(word, start, byte_offset));

                    // Skip ahead
                    while chars.peek().map(|(idx, _)| *idx < end).unwrap_or(false) {
                        chars.next();
                    }
                }

                // Operators and punctuation
                ch if "+-*/%=<>!&|^~?:".contains(ch) => {
                    tokens.push(Token::new(
                        TokenType::Operator,
                        byte_offset + i..byte_offset + i + 1,
                        ch.to_string(),
                    ));
                }

                ch if "(){}[],.;".contains(ch) => {
                    tokens.push(Token::new(
                        TokenType::Punctuation,
                        byte_offset + i..byte_offset + i + 1,
                        ch.to_string(),
                    ));
                }

                // Whitespace - skip
                ch if ch.is_whitespace() => {
                    // Could create whitespace tokens if needed
                    continue;
                }

                // Unknown characters
                _ => {
                    tokens.push(Token::new(
                        TokenType::Text,
                        byte_offset + i..byte_offset + i + ch.len_utf8(),
                        ch.to_string(),
                    ));
                }
            }
        }

        tokens
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Plain text tokenizer (no highlighting)
pub struct PlainTextTokenizer;

impl Tokenizer for PlainTextTokenizer {
    fn tokenize_line(&self, line: &str, byte_offset: usize) -> Vec<Token> {
        if line.is_empty() {
            return Vec::new();
        }

        vec![Token::new(
            TokenType::Text,
            byte_offset..byte_offset + line.len(),
            line.to_string(),
        )]
    }

    fn name(&self) -> &str {
        "Plain Text"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Keyword, 0..5, "const".to_string());

        assert_eq!(token.token_type, TokenType::Keyword);
        assert_eq!(token.range, 0..5);
        assert_eq!(token.text, "const");
        assert_eq!(token.len(), 5);
        assert!(!token.is_empty());
    }

    #[test]
    fn test_plain_text_tokenizer() {
        let tokenizer = PlainTextTokenizer;
        let tokens = tokenizer.tokenize_line("Hello, world!", 0);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Text);
        assert_eq!(tokens[0].text, "Hello, world!");
    }

    #[test]
    fn test_regex_tokenizer_keywords() {
        let tokenizer = RegexTokenizer::new(
            "Test".to_string(),
            vec!["if".to_string(), "else".to_string(), "fn".to_string()],
            vec!["i32".to_string(), "str".to_string()],
            vec!["true".to_string(), "false".to_string()],
        );

        let tokens = tokenizer.tokenize_line("if true", 0);

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[0].text, "if");
        assert_eq!(tokens[1].token_type, TokenType::Constant);
        assert_eq!(tokens[1].text, "true");
    }

    #[test]
    fn test_string_literals() {
        let tokenizer = RegexTokenizer::new(
            "Test".to_string(),
            vec![],
            vec![],
            vec![],
        );

        let tokens = tokenizer.tokenize_line(r#"let s = "hello";"#, 0);

        let string_token = tokens.iter().find(|t| t.token_type == TokenType::String);
        assert!(string_token.is_some());
        assert_eq!(string_token.unwrap().text, r#""hello""#);
    }

    #[test]
    fn test_escaped_strings() {
        let tokenizer = RegexTokenizer::new(
            "Test".to_string(),
            vec![],
            vec![],
            vec![],
        );

        let tokens = tokenizer.tokenize_line(r#""hello \"world\"""#, 0);

        assert!(tokens.iter().any(|t| t.token_type == TokenType::String));
        let string_token = tokens.iter().find(|t| t.token_type == TokenType::String).unwrap();
        assert_eq!(string_token.text, r#""hello \"world\"""#);
    }

    #[test]
    fn test_numbers() {
        let tokenizer = RegexTokenizer::new(
            "Test".to_string(),
            vec![],
            vec![],
            vec![],
        );

        let tokens = tokenizer.tokenize_line("let x = 42;", 0);

        let number_token = tokens.iter().find(|t| t.token_type == TokenType::Number);
        assert!(number_token.is_some());
        assert_eq!(number_token.unwrap().text, "42");
    }

    #[test]
    fn test_comments() {
        let tokenizer = RegexTokenizer::new(
            "Test".to_string(),
            vec![],
            vec![],
            vec![],
        );

        let tokens = tokenizer.tokenize_line("let x = 5; // comment", 0);

        let comment_token = tokens.iter().find(|t| t.token_type == TokenType::Comment);
        assert!(comment_token.is_some());
        assert_eq!(comment_token.unwrap().text, "// comment");
    }

    #[test]
    fn test_operators_and_punctuation() {
        let tokenizer = RegexTokenizer::new(
            "Test".to_string(),
            vec![],
            vec![],
            vec![],
        );

        let tokens = tokenizer.tokenize_line("x + y", 0);

        assert!(tokens.iter().any(|t| t.token_type == TokenType::Operator && t.text == "+"));
    }

    #[test]
    fn test_types() {
        let tokenizer = RegexTokenizer::new(
            "Test".to_string(),
            vec![],
            vec!["i32".to_string(), "String".to_string()],
            vec![],
        );

        let tokens = tokenizer.tokenize_line("let x: i32", 0);

        let type_token = tokens.iter().find(|t| t.token_type == TokenType::Type);
        assert!(type_token.is_some());
        assert_eq!(type_token.unwrap().text, "i32");
    }

    #[test]
    fn test_multiline_tokenization() {
        let tokenizer = RegexTokenizer::new(
            "Test".to_string(),
            vec!["fn".to_string()],
            vec![],
            vec![],
        );

        let text = "fn main() {\n    println!(\"Hello\");\n}";
        let tokens = tokenizer.tokenize_lines(text);

        // Should have tokens for fn, main, identifiers, strings, etc.
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Keyword));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::String));
    }

    #[test]
    fn test_character_literals() {
        let tokenizer = PlainTextTokenizer;

        let tokens = tokenizer.tokenize_line("'a'", 0);
        assert!(!tokens.is_empty());
    }
}
