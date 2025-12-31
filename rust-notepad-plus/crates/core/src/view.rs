//! View management for split-view support

/// View identifier (main or sub view)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewId {
    Main,
    Sub,
}

impl ViewId {
    pub fn other(&self) -> Self {
        match self {
            ViewId::Main => ViewId::Sub,
            ViewId::Sub => ViewId::Main,
        }
    }
}
