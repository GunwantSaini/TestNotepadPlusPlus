//! Language definition

use crate::LanguageType;

#[derive(Debug, Clone)]
pub struct Language {
    pub lang_type: LanguageType,
    pub name: String,
    pub extensions: Vec<String>,
}

impl Language {
    pub fn new(lang_type: LanguageType, extensions: Vec<String>) -> Self {
        Self {
            lang_type,
            name: lang_type.name().to_string(),
            extensions,
        }
    }
}
