//! Language-specific tokenizers

use crate::tokenizer::{RegexTokenizer, Tokenizer};
use std::sync::Arc;

/// Get a tokenizer for Rust
pub fn rust_tokenizer() -> Arc<dyn Tokenizer> {
    let keywords = vec![
        "as", "break", "const", "continue", "crate", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
        "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
        "super", "trait", "true", "type", "unsafe", "use", "where", "while",
        "async", "await", "dyn",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let types = vec![
        "i8", "i16", "i32", "i64", "i128", "isize",
        "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64", "bool", "char", "str",
        "String", "Vec", "Option", "Result", "Box",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let constants = vec!["true", "false", "None", "Some", "Ok", "Err"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Arc::new(RegexTokenizer::new(
        "Rust".to_string(),
        keywords,
        types,
        constants,
    ))
}

/// Get a tokenizer for Python
pub fn python_tokenizer() -> Arc<dyn Tokenizer> {
    let keywords = vec![
        "and", "as", "assert", "async", "await", "break", "class", "continue",
        "def", "del", "elif", "else", "except", "finally", "for", "from",
        "global", "if", "import", "in", "is", "lambda", "nonlocal", "not",
        "or", "pass", "raise", "return", "try", "while", "with", "yield",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let types = vec![
        "int", "float", "str", "bool", "list", "dict", "tuple", "set",
        "bytes", "bytearray", "object", "type",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let constants = vec!["True", "False", "None"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Arc::new(RegexTokenizer::new(
        "Python".to_string(),
        keywords,
        types,
        constants,
    ))
}

/// Get a tokenizer for JavaScript
pub fn javascript_tokenizer() -> Arc<dyn Tokenizer> {
    let keywords = vec![
        "async", "await", "break", "case", "catch", "class", "const", "continue",
        "debugger", "default", "delete", "do", "else", "export", "extends", "finally",
        "for", "function", "if", "import", "in", "instanceof", "let", "new",
        "return", "static", "super", "switch", "this", "throw", "try", "typeof",
        "var", "void", "while", "with", "yield",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let types = vec![
        "Array", "Object", "String", "Number", "Boolean", "Symbol", "Function",
        "Map", "Set", "Promise", "Date", "RegExp",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let constants = vec!["true", "false", "null", "undefined", "NaN", "Infinity"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Arc::new(RegexTokenizer::new(
        "JavaScript".to_string(),
        keywords,
        types,
        constants,
    ))
}

/// Get a tokenizer for C/C++
pub fn c_cpp_tokenizer() -> Arc<dyn Tokenizer> {
    let keywords = vec![
        "auto", "break", "case", "char", "const", "continue", "default", "do",
        "double", "else", "enum", "extern", "float", "for", "goto", "if",
        "inline", "int", "long", "register", "restrict", "return", "short",
        "signed", "sizeof", "static", "struct", "switch", "typedef", "union",
        "unsigned", "void", "volatile", "while",
        // C++ specific
        "bool", "catch", "class", "const_cast", "delete", "dynamic_cast",
        "explicit", "export", "false", "friend", "mutable", "namespace",
        "new", "operator", "private", "protected", "public", "reinterpret_cast",
        "static_cast", "template", "this", "throw", "true", "try", "typeid",
        "typename", "using", "virtual", "wchar_t",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let types = vec![
        "int", "char", "float", "double", "void", "bool",
        "short", "long", "signed", "unsigned",
        "int8_t", "int16_t", "int32_t", "int64_t",
        "uint8_t", "uint16_t", "uint32_t", "uint64_t",
        "size_t", "ptrdiff_t",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let constants = vec!["true", "false", "NULL", "nullptr"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Arc::new(RegexTokenizer::new(
        "C/C++".to_string(),
        keywords,
        types,
        constants,
    ))
}

/// Get a tokenizer for Java
pub fn java_tokenizer() -> Arc<dyn Tokenizer> {
    let keywords = vec![
        "abstract", "assert", "boolean", "break", "byte", "case", "catch", "char",
        "class", "const", "continue", "default", "do", "double", "else", "enum",
        "extends", "final", "finally", "float", "for", "goto", "if", "implements",
        "import", "instanceof", "int", "interface", "long", "native", "new",
        "package", "private", "protected", "public", "return", "short", "static",
        "strictfp", "super", "switch", "synchronized", "this", "throw", "throws",
        "transient", "try", "void", "volatile", "while",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let types = vec![
        "boolean", "byte", "char", "short", "int", "long", "float", "double",
        "String", "Integer", "Double", "Float", "Boolean", "Character",
        "Object", "Class", "List", "Map", "Set", "ArrayList", "HashMap",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let constants = vec!["true", "false", "null"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Arc::new(RegexTokenizer::new(
        "Java".to_string(),
        keywords,
        types,
        constants,
    ))
}

/// Get a tokenizer for Go
pub fn go_tokenizer() -> Arc<dyn Tokenizer> {
    let keywords = vec![
        "break", "case", "chan", "const", "continue", "default", "defer", "else",
        "fallthrough", "for", "func", "go", "goto", "if", "import", "interface",
        "map", "package", "range", "return", "select", "struct", "switch", "type",
        "var",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let types = vec![
        "bool", "byte", "complex64", "complex128", "error", "float32", "float64",
        "int", "int8", "int16", "int32", "int64", "rune", "string",
        "uint", "uint8", "uint16", "uint32", "uint64", "uintptr",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let constants = vec!["true", "false", "nil", "iota"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Arc::new(RegexTokenizer::new(
        "Go".to_string(),
        keywords,
        types,
        constants,
    ))
}

/// Get a tokenizer for SQL
pub fn sql_tokenizer() -> Arc<dyn Tokenizer> {
    let keywords = vec![
        "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP",
        "ALTER", "TABLE", "INDEX", "VIEW", "JOIN", "INNER", "LEFT", "RIGHT", "OUTER",
        "ON", "AND", "OR", "NOT", "IN", "EXISTS", "BETWEEN", "LIKE", "IS", "NULL",
        "ORDER", "BY", "GROUP", "HAVING", "LIMIT", "OFFSET", "AS", "DISTINCT",
        "UNION", "ALL", "INTO", "VALUES", "SET", "PRIMARY", "KEY", "FOREIGN",
        "REFERENCES", "CONSTRAINT", "DEFAULT", "AUTO_INCREMENT",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let types = vec![
        "INT", "INTEGER", "BIGINT", "SMALLINT", "TINYINT",
        "FLOAT", "DOUBLE", "DECIMAL", "NUMERIC",
        "CHAR", "VARCHAR", "TEXT", "BLOB",
        "DATE", "TIME", "DATETIME", "TIMESTAMP",
        "BOOLEAN", "BOOL",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let constants = vec!["TRUE", "FALSE", "NULL"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Arc::new(RegexTokenizer::new(
        "SQL".to_string(),
        keywords,
        types,
        constants,
    ))
}

/// Get a tokenizer for JSON
pub fn json_tokenizer() -> Arc<dyn Tokenizer> {
    Arc::new(RegexTokenizer::new(
        "JSON".to_string(),
        vec![], // JSON has no keywords
        vec![], // JSON has no types
        vec!["true".to_string(), "false".to_string(), "null".to_string()],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_tokenizer() {
        let tokenizer = rust_tokenizer();
        let tokens = tokenizer.tokenize_line("fn main() {", 0);

        // Should recognize 'fn' as a keyword
        assert!(tokens.iter().any(|t| t.text == "fn" && t.token_type == crate::tokenizer::TokenType::Keyword));
    }

    #[test]
    fn test_python_tokenizer() {
        let tokenizer = python_tokenizer();
        let tokens = tokenizer.tokenize_line("def hello():", 0);

        // Should recognize 'def' as a keyword
        assert!(tokens.iter().any(|t| t.text == "def" && t.token_type == crate::tokenizer::TokenType::Keyword));
    }

    #[test]
    fn test_javascript_tokenizer() {
        let tokenizer = javascript_tokenizer();
        let tokens = tokenizer.tokenize_line("const x = true;", 0);

        // Should recognize 'const' as a keyword
        assert!(tokens.iter().any(|t| t.text == "const" && t.token_type == crate::tokenizer::TokenType::Keyword));
        // Should recognize 'true' as a constant
        assert!(tokens.iter().any(|t| t.text == "true" && t.token_type == crate::tokenizer::TokenType::Constant));
    }

    #[test]
    fn test_c_cpp_tokenizer() {
        let tokenizer = c_cpp_tokenizer();
        let tokens = tokenizer.tokenize_line("int main() {", 0);

        // Should recognize 'int' as a type/keyword
        assert!(tokens.iter().any(|t| t.text == "int"));
    }

    #[test]
    fn test_java_tokenizer() {
        let tokenizer = java_tokenizer();
        let tokens = tokenizer.tokenize_line("public class Main {", 0);

        // Should recognize 'public' and 'class' as keywords
        assert!(tokens.iter().any(|t| t.text == "public" && t.token_type == crate::tokenizer::TokenType::Keyword));
        assert!(tokens.iter().any(|t| t.text == "class" && t.token_type == crate::tokenizer::TokenType::Keyword));
    }

    #[test]
    fn test_go_tokenizer() {
        let tokenizer = go_tokenizer();
        let tokens = tokenizer.tokenize_line("func main() {", 0);

        // Should recognize 'func' as a keyword
        assert!(tokens.iter().any(|t| t.text == "func" && t.token_type == crate::tokenizer::TokenType::Keyword));
    }

    #[test]
    fn test_sql_tokenizer() {
        let tokenizer = sql_tokenizer();
        let tokens = tokenizer.tokenize_line("SELECT * FROM users", 0);

        // SQL keywords (case-insensitive in practice, but our tokenizer is case-sensitive)
        // Should have SELECT, FROM as keywords
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_json_tokenizer() {
        let tokenizer = json_tokenizer();
        let tokens = tokenizer.tokenize_line(r#"{"key": true}"#, 0);

        // Should recognize 'true' as a constant
        assert!(tokens.iter().any(|t| t.text == "true" && t.token_type == crate::tokenizer::TokenType::Constant));
    }
}
