//! Theme configuration for syntax highlighting

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Color in RGB format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create from hex value (0xRRGGBB)
    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Convert to hex value (0xRRGGBB)
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Convert to CSS hex string (#RRGGBB)
    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Theme for syntax highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,

    /// Editor background color
    pub background: Color,

    /// Default text color
    pub foreground: Color,

    /// Selection background color
    pub selection_background: Color,

    /// Line number foreground color
    pub line_number_foreground: Color,

    /// Current line highlight color
    pub current_line_background: Color,

    /// Syntax highlighting colors
    pub syntax: SyntaxColors,
}

/// Syntax highlighting colors for different token types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxColors {
    /// Keywords (if, else, fn, class)
    pub keyword: Color,

    /// Types (int, str, bool)
    pub type_name: Color,

    /// String literals
    pub string: Color,

    /// Character literals
    pub char: Color,

    /// Numeric literals
    pub number: Color,

    /// Comments
    pub comment: Color,

    /// Operators (+, -, *, /)
    pub operator: Color,

    /// Punctuation (brackets, braces)
    pub punctuation: Color,

    /// Function names
    pub function: Color,

    /// Variables/identifiers
    pub identifier: Color,

    /// Constants (true, false, null)
    pub constant: Color,

    /// Macros
    pub macro_name: Color,

    /// Preprocessor directives
    pub preprocessor: Color,

    /// Documentation comments
    pub doc_comment: Color,
}

impl Theme {
    /// Create a new theme
    pub fn new(name: String) -> Self {
        Self {
            name,
            background: Color::from_hex(0xFFFFFF),
            foreground: Color::from_hex(0x000000),
            selection_background: Color::from_hex(0xADD6FF),
            line_number_foreground: Color::from_hex(0x888888),
            current_line_background: Color::from_hex(0xF0F0F0),
            syntax: SyntaxColors::default(),
        }
    }

    /// Default light theme
    pub fn default_light() -> Self {
        Self {
            name: "Default Light".to_string(),
            background: Color::from_hex(0xFFFFFF),
            foreground: Color::from_hex(0x000000),
            selection_background: Color::from_hex(0xADD6FF),
            line_number_foreground: Color::from_hex(0x888888),
            current_line_background: Color::from_hex(0xF0F0F0),
            syntax: SyntaxColors {
                keyword: Color::from_hex(0x0000FF),       // Blue
                type_name: Color::from_hex(0x2B91AF),     // Light blue
                string: Color::from_hex(0xA31515),        // Red
                char: Color::from_hex(0xA31515),          // Red
                number: Color::from_hex(0x098658),        // Green
                comment: Color::from_hex(0x008000),       // Green
                operator: Color::from_hex(0x000000),      // Black
                punctuation: Color::from_hex(0x000000),   // Black
                function: Color::from_hex(0x795E26),      // Brown
                identifier: Color::from_hex(0x000000),    // Black
                constant: Color::from_hex(0x0000FF),      // Blue
                macro_name: Color::from_hex(0x811F3F),    // Purple
                preprocessor: Color::from_hex(0x9B9B9B),  // Gray
                doc_comment: Color::from_hex(0x008000),   // Green
            },
        }
    }

    /// Dark theme (Monokai-inspired)
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background: Color::from_hex(0x272822),
            foreground: Color::from_hex(0xF8F8F2),
            selection_background: Color::from_hex(0x49483E),
            line_number_foreground: Color::from_hex(0x90908A),
            current_line_background: Color::from_hex(0x3E3D32),
            syntax: SyntaxColors {
                keyword: Color::from_hex(0xF92672),       // Pink
                type_name: Color::from_hex(0x66D9EF),     // Cyan
                string: Color::from_hex(0xE6DB74),        // Yellow
                char: Color::from_hex(0xE6DB74),          // Yellow
                number: Color::from_hex(0xAE81FF),        // Purple
                comment: Color::from_hex(0x75715E),       // Gray
                operator: Color::from_hex(0xF8F8F2),      // White
                punctuation: Color::from_hex(0xF8F8F2),   // White
                function: Color::from_hex(0xA6E22E),      // Green
                identifier: Color::from_hex(0xF8F8F2),    // White
                constant: Color::from_hex(0xAE81FF),      // Purple
                macro_name: Color::from_hex(0xF92672),    // Pink
                preprocessor: Color::from_hex(0xF92672),  // Pink
                doc_comment: Color::from_hex(0x75715E),   // Gray
            },
        }
    }

    /// Solarized light theme
    pub fn solarized_light() -> Self {
        Self {
            name: "Solarized Light".to_string(),
            background: Color::from_hex(0xFDF6E3),
            foreground: Color::from_hex(0x657B83),
            selection_background: Color::from_hex(0xEEE8D5),
            line_number_foreground: Color::from_hex(0x93A1A1),
            current_line_background: Color::from_hex(0xEEE8D5),
            syntax: SyntaxColors {
                keyword: Color::from_hex(0x859900),       // Green
                type_name: Color::from_hex(0xB58900),     // Yellow
                string: Color::from_hex(0x2AA198),        // Cyan
                char: Color::from_hex(0x2AA198),          // Cyan
                number: Color::from_hex(0xD33682),        // Magenta
                comment: Color::from_hex(0x93A1A1),       // Gray
                operator: Color::from_hex(0x657B83),      // Base00
                punctuation: Color::from_hex(0x657B83),   // Base00
                function: Color::from_hex(0x268BD2),      // Blue
                identifier: Color::from_hex(0x657B83),    // Base00
                constant: Color::from_hex(0xCB4B16),      // Orange
                macro_name: Color::from_hex(0x6C71C4),    // Violet
                preprocessor: Color::from_hex(0xCB4B16),  // Orange
                doc_comment: Color::from_hex(0x93A1A1),   // Gray
            },
        }
    }

    /// Get a collection of built-in themes
    pub fn built_in_themes() -> HashMap<String, Theme> {
        let mut themes = HashMap::new();
        themes.insert("Default Light".to_string(), Self::default_light());
        themes.insert("Dark".to_string(), Self::dark());
        themes.insert("Solarized Light".to_string(), Self::solarized_light());
        themes
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_light()
    }
}

impl Default for SyntaxColors {
    fn default() -> Self {
        Self {
            keyword: Color::from_hex(0x0000FF),
            type_name: Color::from_hex(0x2B91AF),
            string: Color::from_hex(0xA31515),
            char: Color::from_hex(0xA31515),
            number: Color::from_hex(0x098658),
            comment: Color::from_hex(0x008000),
            operator: Color::from_hex(0x000000),
            punctuation: Color::from_hex(0x000000),
            function: Color::from_hex(0x795E26),
            identifier: Color::from_hex(0x000000),
            constant: Color::from_hex(0x0000FF),
            macro_name: Color::from_hex(0x811F3F),
            preprocessor: Color::from_hex(0x9B9B9B),
            doc_comment: Color::from_hex(0x008000),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex(0xFF8040);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.to_hex(), 0xFF8040);
    }

    #[test]
    fn test_color_to_hex_string() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.to_hex_string(), "#FF8040");
    }

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.name, "Default Light");
        assert_eq!(theme.background, Color::from_hex(0xFFFFFF));
        assert_eq!(theme.foreground, Color::from_hex(0x000000));
    }

    #[test]
    fn test_dark_theme() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "Dark");
        // Dark background
        assert_ne!(theme.background, Color::from_hex(0xFFFFFF));
        // Light foreground
        assert_ne!(theme.foreground, Color::from_hex(0x000000));
    }

    #[test]
    fn test_built_in_themes() {
        let themes = Theme::built_in_themes();
        assert!(themes.contains_key("Default Light"));
        assert!(themes.contains_key("Dark"));
        assert!(themes.contains_key("Solarized Light"));
        assert_eq!(themes.len(), 3);
    }

    #[test]
    fn test_syntax_colors() {
        let theme = Theme::default_light();
        // Keywords should be blue in default light theme
        assert_eq!(theme.syntax.keyword, Color::from_hex(0x0000FF));
        // Strings should be red
        assert_eq!(theme.syntax.string, Color::from_hex(0xA31515));
        // Comments should be green
        assert_eq!(theme.syntax.comment, Color::from_hex(0x008000));
    }

    #[test]
    fn test_solarized_theme() {
        let theme = Theme::solarized_light();
        assert_eq!(theme.name, "Solarized Light");
        assert_eq!(theme.background, Color::from_hex(0xFDF6E3));
    }
}
