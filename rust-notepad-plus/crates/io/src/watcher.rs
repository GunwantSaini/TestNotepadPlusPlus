//! File system watching for external file changes

use notify::Result;
use std::path::Path;

pub struct FileWatcher {
    // Will implement file watching later
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn watch<P: AsRef<Path>>(&mut self, _path: P) -> Result<()> {
        // TODO: Implement watching
        Ok(())
    }
}
