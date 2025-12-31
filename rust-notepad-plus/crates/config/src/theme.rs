//! Theme configuration

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub background: u32,
    pub foreground: u32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            background: 0xFFFFFF,
            foreground: 0x000000,
        }
    }
}
