//! Syntax highlighting and lexer management

pub mod language;
pub mod registry;

pub use language::Language;
pub use registry::LanguageRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageType {
    Text,
    C,
    Cpp,
    CSharp,
    Java,
    Javascript,
    Python,
    Rust,
    Go,
    Ruby,
    Php,
    Html,
    Css,
    Xml,
    Json,
    Yaml,
    Markdown,
    Sql,
    Bash,
    PowerShell,
    // Add more as needed
}

impl LanguageType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "c" | "h" => Self::C,
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Self::Cpp,
            "cs" => Self::CSharp,
            "java" => Self::Java,
            "js" | "jsx" => Self::Javascript,
            "py" | "pyw" => Self::Python,
            "rs" => Self::Rust,
            "go" => Self::Go,
            "rb" => Self::Ruby,
            "php" => Self::Php,
            "html" | "htm" => Self::Html,
            "css" => Self::Css,
            "xml" => Self::Xml,
            "json" => Self::Json,
            "yml" | "yaml" => Self::Yaml,
            "md" | "markdown" => Self::Markdown,
            "sql" => Self::Sql,
            "sh" | "bash" => Self::Bash,
            "ps1" | "psm1" => Self::PowerShell,
            _ => Self::Text,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Text => "Plain Text",
            Self::C => "C",
            Self::Cpp => "C++",
            Self::CSharp => "C#",
            Self::Java => "Java",
            Self::Javascript => "JavaScript",
            Self::Python => "Python",
            Self::Rust => "Rust",
            Self::Go => "Go",
            Self::Ruby => "Ruby",
            Self::Php => "PHP",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::Xml => "XML",
            Self::Json => "JSON",
            Self::Yaml => "YAML",
            Self::Markdown => "Markdown",
            Self::Sql => "SQL",
            Self::Bash => "Bash",
            Self::PowerShell => "PowerShell",
        }
    }
}
