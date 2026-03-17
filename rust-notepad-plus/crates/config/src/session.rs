//! Session management for remembering open files

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Represents a saved session with open files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session name
    pub name: String,

    /// When this session was last saved
    pub last_saved: DateTime<Utc>,

    /// List of open files
    pub files: Vec<SessionFile>,

    /// Active file index
    pub active_file_index: Option<usize>,

    /// View-specific data
    #[serde(default)]
    pub view_data: ViewData,
}

/// Information about a file in the session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionFile {
    /// File path
    pub path: PathBuf,

    /// Encoding used for this file
    pub encoding: String,

    /// Line ending format
    pub line_ending: String,

    /// Cursor position (line, column)
    pub cursor_position: Option<(usize, usize)>,

    /// Scroll position (first visible line)
    pub scroll_position: Option<usize>,

    /// Whether the file has unsaved changes
    pub is_modified: bool,

    /// Bookmark lines
    #[serde(default)]
    pub bookmarks: Vec<usize>,
}

/// View-specific data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewData {
    /// Zoom level (100 = normal)
    pub zoom_level: i32,

    /// Word wrap enabled
    pub word_wrap: bool,

    /// Show line numbers
    pub show_line_numbers: bool,
}

impl Session {
    /// Create a new empty session
    pub fn new(name: String) -> Self {
        Self {
            name,
            last_saved: Utc::now(),
            files: Vec::new(),
            active_file_index: None,
            view_data: ViewData::default(),
        }
    }

    /// Add a file to the session
    pub fn add_file(&mut self, file: SessionFile) {
        self.files.push(file);
        if self.active_file_index.is_none() {
            self.active_file_index = Some(0);
        }
    }

    /// Remove a file from the session by path
    pub fn remove_file(&mut self, path: &PathBuf) {
        if let Some(index) = self.files.iter().position(|f| &f.path == path) {
            self.files.remove(index);

            // Update active index if needed
            if let Some(active) = self.active_file_index {
                if active >= self.files.len() {
                    self.active_file_index = if self.files.is_empty() {
                        None
                    } else {
                        Some(self.files.len() - 1)
                    };
                }
            }
        }
    }

    /// Set the active file by path
    pub fn set_active_file(&mut self, path: &PathBuf) {
        if let Some(index) = self.files.iter().position(|f| &f.path == path) {
            self.active_file_index = Some(index);
        }
    }

    /// Get the active file
    pub fn get_active_file(&self) -> Option<&SessionFile> {
        self.active_file_index
            .and_then(|idx| self.files.get(idx))
    }

    /// Update a file's state in the session
    pub fn update_file(&mut self, path: &PathBuf, update_fn: impl FnOnce(&mut SessionFile)) {
        if let Some(file) = self.files.iter_mut().find(|f| &f.path == path) {
            update_fn(file);
        }
    }

    /// Load session from file
    pub fn load(path: PathBuf) -> Result<Self> {
        log::info!("Loading session from: {:?}", path);

        let contents = fs::read_to_string(&path)
            .context("Failed to read session file")?;

        let session: Session = toml::from_str(&contents)
            .context("Failed to parse session file")?;

        Ok(session)
    }

    /// Save session to file
    pub fn save(&mut self, path: PathBuf) -> Result<()> {
        // Update last saved time
        self.last_saved = Utc::now();

        log::info!("Saving session to: {:?}", path);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create session directory")?;
        }

        let toml_string = toml::to_string_pretty(self)
            .context("Failed to serialize session")?;

        fs::write(&path, toml_string)
            .context("Failed to write session file")?;

        Ok(())
    }

    /// Check if session file exists
    pub fn exists(path: &PathBuf) -> bool {
        path.exists()
    }

    /// Get number of files in session
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Check if session is empty
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

impl SessionFile {
    /// Create a new session file entry
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            encoding: "UTF-8".to_string(),
            line_ending: "Windows".to_string(),
            cursor_position: None,
            scroll_position: None,
            is_modified: false,
            bookmarks: Vec::new(),
        }
    }

    /// Create with full details
    pub fn with_details(
        path: PathBuf,
        encoding: String,
        line_ending: String,
        cursor_position: Option<(usize, usize)>,
    ) -> Self {
        Self {
            path,
            encoding,
            line_ending,
            cursor_position,
            scroll_position: None,
            is_modified: false,
            bookmarks: Vec::new(),
        }
    }

    /// Add a bookmark to this file
    pub fn add_bookmark(&mut self, line: usize) {
        if !self.bookmarks.contains(&line) {
            self.bookmarks.push(line);
            self.bookmarks.sort();
        }
    }

    /// Remove a bookmark from this file
    pub fn remove_bookmark(&mut self, line: usize) {
        self.bookmarks.retain(|&l| l != line);
    }

    /// Toggle bookmark at line
    pub fn toggle_bookmark(&mut self, line: usize) {
        if self.bookmarks.contains(&line) {
            self.remove_bookmark(line);
        } else {
            self.add_bookmark(line);
        }
    }
}

impl Default for ViewData {
    fn default() -> Self {
        Self {
            zoom_level: 100,
            word_wrap: false,
            show_line_numbers: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_new_session() {
        let session = Session::new("test-session".to_string());

        assert_eq!(session.name, "test-session");
        assert!(session.files.is_empty());
        assert!(session.active_file_index.is_none());
        assert!(session.is_empty());
    }

    #[test]
    fn test_add_file() {
        let mut session = Session::new("test".to_string());

        let file = SessionFile::new(PathBuf::from("/test/file1.txt"));
        session.add_file(file);

        assert_eq!(session.file_count(), 1);
        assert_eq!(session.active_file_index, Some(0));
    }

    #[test]
    fn test_remove_file() {
        let mut session = Session::new("test".to_string());

        let path1 = PathBuf::from("/test/file1.txt");
        let path2 = PathBuf::from("/test/file2.txt");

        session.add_file(SessionFile::new(path1.clone()));
        session.add_file(SessionFile::new(path2.clone()));

        assert_eq!(session.file_count(), 2);

        session.remove_file(&path1);

        assert_eq!(session.file_count(), 1);
        assert_eq!(session.files[0].path, path2);
    }

    #[test]
    fn test_set_active_file() {
        let mut session = Session::new("test".to_string());

        let path1 = PathBuf::from("/test/file1.txt");
        let path2 = PathBuf::from("/test/file2.txt");

        session.add_file(SessionFile::new(path1.clone()));
        session.add_file(SessionFile::new(path2.clone()));

        session.set_active_file(&path2);

        assert_eq!(session.active_file_index, Some(1));

        let active = session.get_active_file().unwrap();
        assert_eq!(active.path, path2);
    }

    #[test]
    fn test_save_and_load() -> Result<()> {
        let temp_dir = env::temp_dir().join("notepad-test-session");
        fs::create_dir_all(&temp_dir)?;

        let session_file = temp_dir.join("session.toml");

        // Create session
        let mut session = Session::new("test-session".to_string());
        session.add_file(SessionFile::new(PathBuf::from("/test/file1.txt")));
        session.add_file(SessionFile::with_details(
            PathBuf::from("/test/file2.txt"),
            "UTF-8".to_string(),
            "Unix".to_string(),
            Some((10, 5)),
        ));

        session.save(session_file.clone())?;

        // Load session
        let loaded = Session::load(session_file)?;

        assert_eq!(loaded.name, "test-session");
        assert_eq!(loaded.file_count(), 2);
        assert_eq!(loaded.files[0].path, PathBuf::from("/test/file1.txt"));
        assert_eq!(loaded.files[1].cursor_position, Some((10, 5)));

        // Cleanup
        fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    #[test]
    fn test_update_file() {
        let mut session = Session::new("test".to_string());
        let path = PathBuf::from("/test/file1.txt");

        session.add_file(SessionFile::new(path.clone()));

        session.update_file(&path, |file| {
            file.cursor_position = Some((5, 10));
            file.is_modified = true;
        });

        let file = &session.files[0];
        assert_eq!(file.cursor_position, Some((5, 10)));
        assert!(file.is_modified);
    }

    #[test]
    fn test_session_file_bookmarks() {
        let mut file = SessionFile::new(PathBuf::from("/test/file.txt"));

        // Add bookmarks
        file.add_bookmark(5);
        file.add_bookmark(10);
        file.add_bookmark(3);

        assert_eq!(file.bookmarks, vec![3, 5, 10]); // Should be sorted

        // Toggle bookmark
        file.toggle_bookmark(5);
        assert_eq!(file.bookmarks, vec![3, 10]);

        file.toggle_bookmark(5);
        assert_eq!(file.bookmarks, vec![3, 5, 10]);

        // Remove bookmark
        file.remove_bookmark(10);
        assert_eq!(file.bookmarks, vec![3, 5]);
    }

    #[test]
    fn test_toml_serialization() -> Result<()> {
        let mut session = Session::new("test".to_string());
        session.add_file(SessionFile::new(PathBuf::from("/test/file.txt")));

        let toml_string = toml::to_string_pretty(&session)?;

        assert!(toml_string.contains("name = \"test\""));
        assert!(toml_string.contains("[[files]]"));
        assert!(toml_string.contains("path ="));

        Ok(())
    }
}
