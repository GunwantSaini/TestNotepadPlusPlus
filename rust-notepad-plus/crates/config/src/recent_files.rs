//! Recent files list management

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

/// Manages the list of recently opened files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentFiles {
    /// Maximum number of recent files to track
    max_files: usize,

    /// List of recent files (most recent first)
    files: VecDeque<RecentFileEntry>,
}

/// Entry in the recent files list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecentFileEntry {
    /// File path
    pub path: PathBuf,

    /// Last accessed time
    pub last_accessed: DateTime<Utc>,

    /// Display name (filename only)
    #[serde(skip)]
    pub display_name: String,
}

impl RecentFiles {
    /// Create a new recent files list with the given maximum size
    pub fn new(max_files: usize) -> Self {
        Self {
            max_files,
            files: VecDeque::new(),
        }
    }

    /// Add a file to the recent files list
    ///
    /// If the file already exists, it's moved to the top.
    /// If the list is full, the oldest file is removed.
    pub fn add(&mut self, path: PathBuf) {
        // Remove existing entry if present
        self.files.retain(|entry| entry.path != path);

        // Create new entry
        let display_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let entry = RecentFileEntry {
            path,
            last_accessed: Utc::now(),
            display_name,
        };

        // Add to front
        self.files.push_front(entry);

        // Trim if needed
        while self.files.len() > self.max_files {
            self.files.pop_back();
        }
    }

    /// Get all recent files (most recent first)
    pub fn get_all(&self) -> Vec<&RecentFileEntry> {
        self.files.iter().collect()
    }

    /// Get recent files that still exist on disk
    pub fn get_existing(&self) -> Vec<&RecentFileEntry> {
        self.files
            .iter()
            .filter(|entry| entry.path.exists())
            .collect()
    }

    /// Remove a file from the recent list
    pub fn remove(&mut self, path: &PathBuf) {
        self.files.retain(|entry| &entry.path != path);
    }

    /// Clear all recent files
    pub fn clear(&mut self) {
        self.files.clear();
    }

    /// Remove files that no longer exist on disk
    pub fn clean_nonexistent(&mut self) {
        self.files.retain(|entry| entry.path.exists());
    }

    /// Check if a file is in the recent list
    pub fn contains(&self, path: &PathBuf) -> bool {
        self.files.iter().any(|entry| &entry.path == path)
    }

    /// Get the number of recent files
    pub fn count(&self) -> usize {
        self.files.len()
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Get the most recent file
    pub fn get_most_recent(&self) -> Option<&RecentFileEntry> {
        self.files.front()
    }

    /// Set maximum number of files to track
    pub fn set_max_files(&mut self, max: usize) {
        self.max_files = max;

        // Trim if needed
        while self.files.len() > self.max_files {
            self.files.pop_back();
        }
    }

    /// Load recent files from a file
    pub fn load(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new(10)); // Default to 10 files
        }

        log::info!("Loading recent files from: {:?}", path);

        let contents = fs::read_to_string(&path)
            .context("Failed to read recent files")?;

        let mut recent: RecentFiles = toml::from_str(&contents)
            .context("Failed to parse recent files")?;

        // Update display names (not serialized)
        for entry in &mut recent.files {
            entry.display_name = entry
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
        }

        Ok(recent)
    }

    /// Save recent files to a file
    pub fn save(&self, path: PathBuf) -> Result<()> {
        log::info!("Saving recent files to: {:?}", path);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create recent files directory")?;
        }

        let toml_string = toml::to_string_pretty(self)
            .context("Failed to serialize recent files")?;

        fs::write(&path, toml_string)
            .context("Failed to write recent files")?;

        Ok(())
    }
}

impl Default for RecentFiles {
    fn default() -> Self {
        Self::new(10)
    }
}

impl RecentFileEntry {
    /// Create a new recent file entry
    pub fn new(path: PathBuf) -> Self {
        let display_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Self {
            path,
            last_accessed: Utc::now(),
            display_name,
        }
    }

    /// Check if the file exists on disk
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get file extension
    pub fn extension(&self) -> Option<String> {
        self.path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    /// Get parent directory
    pub fn parent_dir(&self) -> Option<PathBuf> {
        self.path.parent().map(|p| p.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_recent_files() {
        let recent = RecentFiles::new(5);

        assert_eq!(recent.max_files, 5);
        assert!(recent.is_empty());
        assert_eq!(recent.count(), 0);
    }

    #[test]
    fn test_add_file() {
        let mut recent = RecentFiles::new(5);

        recent.add(PathBuf::from("/test/file1.txt"));
        recent.add(PathBuf::from("/test/file2.txt"));

        assert_eq!(recent.count(), 2);

        let files = recent.get_all();
        assert_eq!(files[0].path, PathBuf::from("/test/file2.txt"));
        assert_eq!(files[1].path, PathBuf::from("/test/file1.txt"));
    }

    #[test]
    fn test_add_duplicate_moves_to_top() {
        let mut recent = RecentFiles::new(5);

        recent.add(PathBuf::from("/test/file1.txt"));
        recent.add(PathBuf::from("/test/file2.txt"));
        recent.add(PathBuf::from("/test/file3.txt"));

        // Add file1 again
        recent.add(PathBuf::from("/test/file1.txt"));

        let files = recent.get_all();
        assert_eq!(files.len(), 3); // Still only 3 files
        assert_eq!(files[0].path, PathBuf::from("/test/file1.txt")); // Now at top
    }

    #[test]
    fn test_max_files_limit() {
        let mut recent = RecentFiles::new(3);

        recent.add(PathBuf::from("/test/file1.txt"));
        recent.add(PathBuf::from("/test/file2.txt"));
        recent.add(PathBuf::from("/test/file3.txt"));
        recent.add(PathBuf::from("/test/file4.txt"));

        assert_eq!(recent.count(), 3); // Should not exceed max

        let files = recent.get_all();
        assert_eq!(files[0].path, PathBuf::from("/test/file4.txt"));
        assert_eq!(files[2].path, PathBuf::from("/test/file2.txt"));

        // file1 should have been removed
        assert!(!recent.contains(&PathBuf::from("/test/file1.txt")));
    }

    #[test]
    fn test_remove_file() {
        let mut recent = RecentFiles::new(5);

        let path1 = PathBuf::from("/test/file1.txt");
        let path2 = PathBuf::from("/test/file2.txt");

        recent.add(path1.clone());
        recent.add(path2.clone());

        assert_eq!(recent.count(), 2);

        recent.remove(&path1);

        assert_eq!(recent.count(), 1);
        assert!(!recent.contains(&path1));
        assert!(recent.contains(&path2));
    }

    #[test]
    fn test_clear() {
        let mut recent = RecentFiles::new(5);

        recent.add(PathBuf::from("/test/file1.txt"));
        recent.add(PathBuf::from("/test/file2.txt"));

        assert_eq!(recent.count(), 2);

        recent.clear();

        assert!(recent.is_empty());
        assert_eq!(recent.count(), 0);
    }

    #[test]
    fn test_get_most_recent() {
        let mut recent = RecentFiles::new(5);

        recent.add(PathBuf::from("/test/file1.txt"));
        thread::sleep(Duration::from_millis(10));
        recent.add(PathBuf::from("/test/file2.txt"));

        let most_recent = recent.get_most_recent().unwrap();
        assert_eq!(most_recent.path, PathBuf::from("/test/file2.txt"));
    }

    #[test]
    fn test_set_max_files_trims() {
        let mut recent = RecentFiles::new(5);

        recent.add(PathBuf::from("/test/file1.txt"));
        recent.add(PathBuf::from("/test/file2.txt"));
        recent.add(PathBuf::from("/test/file3.txt"));

        assert_eq!(recent.count(), 3);

        // Reduce max
        recent.set_max_files(2);

        assert_eq!(recent.count(), 2);

        let files = recent.get_all();
        assert_eq!(files[0].path, PathBuf::from("/test/file3.txt"));
        assert_eq!(files[1].path, PathBuf::from("/test/file2.txt"));
    }

    #[test]
    fn test_save_and_load() -> Result<()> {
        let temp_dir = env::temp_dir().join("notepad-test-recent");
        fs::create_dir_all(&temp_dir)?;

        let recent_file = temp_dir.join("recent.toml");

        // Create and save
        let mut recent = RecentFiles::new(10);
        recent.add(PathBuf::from("/test/file1.txt"));
        recent.add(PathBuf::from("/test/file2.txt"));
        recent.save(recent_file.clone())?;

        // Load
        let loaded = RecentFiles::load(recent_file)?;

        assert_eq!(loaded.count(), 2);
        assert_eq!(loaded.max_files, 10);

        let files = loaded.get_all();
        assert_eq!(files[0].path, PathBuf::from("/test/file2.txt"));
        assert_eq!(files[1].path, PathBuf::from("/test/file1.txt"));

        // Cleanup
        fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    #[test]
    fn test_recent_file_entry() {
        let path = PathBuf::from("/test/document.txt");
        let entry = RecentFileEntry::new(path.clone());

        assert_eq!(entry.path, path);
        assert_eq!(entry.display_name, "document.txt");
        assert_eq!(entry.extension(), Some("txt".to_string()));
    }

    #[test]
    fn test_clean_nonexistent() -> Result<()> {
        let temp_dir = env::temp_dir().join("notepad-test-clean");
        fs::create_dir_all(&temp_dir)?;

        // Create a real file
        let real_file = temp_dir.join("real.txt");
        fs::write(&real_file, "test")?;

        let mut recent = RecentFiles::new(10);
        recent.add(real_file.clone());
        recent.add(PathBuf::from("/nonexistent/file.txt"));

        assert_eq!(recent.count(), 2);

        recent.clean_nonexistent();

        assert_eq!(recent.count(), 1);
        assert!(recent.contains(&real_file));

        // Cleanup
        fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    #[test]
    fn test_display_name() {
        let entry = RecentFileEntry::new(PathBuf::from("/path/to/my_file.rs"));
        assert_eq!(entry.display_name, "my_file.rs");

        let entry2 = RecentFileEntry::new(PathBuf::from("simple.txt"));
        assert_eq!(entry2.display_name, "simple.txt");
    }
}
