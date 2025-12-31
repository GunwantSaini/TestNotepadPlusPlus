//! Language registry

use crate::{Language, LanguageType};
use std::collections::HashMap;

pub struct LanguageRegistry {
    languages: HashMap<LanguageType, Language>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            languages: HashMap::new(),
        };
        registry.register_default_languages();
        registry
    }

    fn register_default_languages(&mut self) {
        self.register(Language::new(LanguageType::Rust, vec!["rs".to_string()]));
        self.register(Language::new(LanguageType::Python, vec!["py".to_string(), "pyw".to_string()]));
        self.register(Language::new(LanguageType::Javascript, vec!["js".to_string(), "jsx".to_string()]));
        // Add more...
    }

    pub fn register(&mut self, language: Language) {
        self.languages.insert(language.lang_type, language);
    }

    pub fn get(&self, lang_type: LanguageType) -> Option<&Language> {
        self.languages.get(&lang_type)
    }
}
