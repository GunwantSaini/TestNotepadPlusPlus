//! File management singleton
//!
//! Corresponds to the C++ FileManager class

use crate::{Buffer, Result};
use crate::app::EncodingType;
use std::path::PathBuf;

/// File manager singleton
///
/// Handles file I/O, encoding detection, and backup management
pub struct FileManager {
    // Future: Add caching, backup management, etc.
}

impl FileManager {
    /// Create a new file manager
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Load a file and create a buffer
    pub fn load_file(&self, path: PathBuf) -> Result<Buffer> {
        log::info!("Loading file: {:?}", path);

        // Read file content
        let bytes = std::fs::read(&path)?;

        // Detect encoding (simplified - will use chardetng later)
        let (content, encoding) = Self::decode_bytes(&bytes);

        Ok(Buffer::from_file(path, content, encoding))
    }

    /// Save a buffer to disk
    pub fn save_buffer(&self, buffer: &Buffer) -> Result<()> {
        if let Some(path) = buffer.file_path() {
            log::info!("Saving file: {:?}", path);

            // Encode content based on buffer encoding
            let bytes = Self::encode_string(buffer.content(), buffer.encoding());

            // Write to file
            std::fs::write(path, bytes)?;

            Ok(())
        } else {
            Err(crate::NotepadError::Command(
                "Cannot save buffer without file path".to_string(),
            ))
        }
    }

    /// Decode bytes to string with encoding detection
    fn decode_bytes(bytes: &[u8]) -> (String, EncodingType) {
        // Check for UTF-8 BOM
        if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            let content = String::from_utf8_lossy(&bytes[3..]).to_string();
            return (content, EncodingType::Utf8Bom);
        }

        // Check for UTF-16 LE BOM
        if bytes.starts_with(&[0xFF, 0xFE]) {
            // UTF-16 LE with BOM
            return (
                Self::decode_utf16_le(&bytes[2..]),
                EncodingType::Utf16LeBom,
            );
        }

        // Check for UTF-16 BE BOM
        if bytes.starts_with(&[0xFE, 0xFF]) {
            // UTF-16 BE with BOM
            return (
                Self::decode_utf16_be(&bytes[2..]),
                EncodingType::Utf16BeBom,
            );
        }

        // Try UTF-8 without BOM
        if let Ok(content) = std::str::from_utf8(bytes) {
            return (content.to_string(), EncodingType::Utf8);
        }

        // Fallback to Windows-1252/ANSI
        let content = bytes.iter().map(|&b| b as char).collect();
        (content, EncodingType::Ansi)
    }

    /// Decode UTF-16 LE bytes
    fn decode_utf16_le(bytes: &[u8]) -> String {
        let u16_vec: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        String::from_utf16_lossy(&u16_vec)
    }

    /// Decode UTF-16 BE bytes
    fn decode_utf16_be(bytes: &[u8]) -> String {
        let u16_vec: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        String::from_utf16_lossy(&u16_vec)
    }

    /// Encode string to bytes with specified encoding
    fn encode_string(content: &str, encoding: EncodingType) -> Vec<u8> {
        match encoding {
            EncodingType::Utf8 => content.as_bytes().to_vec(),
            EncodingType::Utf8Bom => {
                let mut bytes = vec![0xEF, 0xBB, 0xBF];
                bytes.extend_from_slice(content.as_bytes());
                bytes
            }
            EncodingType::Utf16Le => Self::encode_utf16_le(content, false),
            EncodingType::Utf16LeBom => Self::encode_utf16_le(content, true),
            EncodingType::Utf16Be => Self::encode_utf16_be(content, false),
            EncodingType::Utf16BeBom => Self::encode_utf16_be(content, true),
            EncodingType::Ansi => {
                // Simplified ANSI encoding
                content.chars().map(|c| c as u8).collect()
            }
        }
    }

    /// Encode string as UTF-16 LE
    fn encode_utf16_le(content: &str, with_bom: bool) -> Vec<u8> {
        let mut bytes = Vec::new();
        if with_bom {
            bytes.extend_from_slice(&[0xFF, 0xFE]);
        }
        for c in content.encode_utf16() {
            bytes.extend_from_slice(&c.to_le_bytes());
        }
        bytes
    }

    /// Encode string as UTF-16 BE
    fn encode_utf16_be(content: &str, with_bom: bool) -> Vec<u8> {
        let mut bytes = Vec::new();
        if with_bom {
            bytes.extend_from_slice(&[0xFE, 0xFF]);
        }
        for c in content.encode_utf16() {
            bytes.extend_from_slice(&c.to_be_bytes());
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_encoding() {
        let content = "Hello, world!";
        let bytes = FileManager::encode_string(content, EncodingType::Utf8);
        let (decoded, encoding) = FileManager::decode_bytes(&bytes);

        assert_eq!(decoded, content);
        assert_eq!(encoding, EncodingType::Utf8);
    }

    #[test]
    fn test_utf8_bom_encoding() {
        let content = "Hello, BOM!";
        let bytes = FileManager::encode_string(content, EncodingType::Utf8Bom);
        let (decoded, encoding) = FileManager::decode_bytes(&bytes);

        assert_eq!(decoded, content);
        assert_eq!(encoding, EncodingType::Utf8Bom);
    }
}
