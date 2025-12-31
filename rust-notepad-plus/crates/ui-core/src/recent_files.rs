//! Recent files list management (MRU - Most Recently Used)

use std::path::PathBuf;
use std::collections::VecDeque;

const MAX_RECENT_FILES: usize = 10;

/// Recent files manager
#[derive(Debug, Clone)]
pub struct RecentFiles {
    files: VecDeque<PathBuf>,
}

impl Default for RecentFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl RecentFiles {
    /// Create a new recent files manager
    pub fn new() -> Self {
        Self {
            files: VecDeque::with_capacity(MAX_RECENT_FILES),
        }
    }

    /// Add a file to the recent files list
    /// If the file is already in the list, it will be moved to the front
    pub fn add_file(&mut self, path: PathBuf) {
        // Remove the file if it already exists in the list
        if let Some(pos) = self.files.iter().position(|p| p == &path) {
            self.files.remove(pos);
        }

        // Add to front
        self.files.push_front(path);

        // Limit to MAX_RECENT_FILES
        if self.files.len() > MAX_RECENT_FILES {
            self.files.pop_back();
        }
    }

    /// Get the list of recent files
    pub fn get_files(&self) -> &VecDeque<PathBuf> {
        &self.files
    }

    /// Clear all recent files
    pub fn clear(&mut self) {
        self.files.clear();
    }

    /// Get the number of recent files
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Check if the recent files list is empty
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Remove a specific file from the recent files list
    pub fn remove_file(&mut self, path: &PathBuf) {
        if let Some(pos) = self.files.iter().position(|p| p == path) {
            self.files.remove(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_file() {
        let mut recent = RecentFiles::new();
        recent.add_file(PathBuf::from("file1.txt"));
        recent.add_file(PathBuf::from("file2.txt"));

        assert_eq!(recent.len(), 2);
        assert_eq!(recent.get_files()[0], PathBuf::from("file2.txt"));
        assert_eq!(recent.get_files()[1], PathBuf::from("file1.txt"));
    }

    #[test]
    fn test_add_duplicate() {
        let mut recent = RecentFiles::new();
        recent.add_file(PathBuf::from("file1.txt"));
        recent.add_file(PathBuf::from("file2.txt"));
        recent.add_file(PathBuf::from("file1.txt")); // Move to front

        assert_eq!(recent.len(), 2);
        assert_eq!(recent.get_files()[0], PathBuf::from("file1.txt"));
        assert_eq!(recent.get_files()[1], PathBuf::from("file2.txt"));
    }

    #[test]
    fn test_max_files() {
        let mut recent = RecentFiles::new();
        for i in 0..15 {
            recent.add_file(PathBuf::from(format!("file{}.txt", i)));
        }

        assert_eq!(recent.len(), MAX_RECENT_FILES);
        assert_eq!(recent.get_files()[0], PathBuf::from("file14.txt"));
    }

    #[test]
    fn test_clear() {
        let mut recent = RecentFiles::new();
        recent.add_file(PathBuf::from("file1.txt"));
        recent.clear();

        assert_eq!(recent.len(), 0);
    }
}
