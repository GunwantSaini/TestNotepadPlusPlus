//! Text encoding detection and conversion

use std::path::Path;

/// Supported text encodings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Utf8Bom,
    Utf16Le,
    Utf16LeBom,
    Utf16Be,
    Utf16BeBom,
    Ansi,
}

impl Encoding {
    /// Get the display name for this encoding
    pub fn display_name(&self) -> &'static str {
        match self {
            Encoding::Utf8 => "UTF-8",
            Encoding::Utf8Bom => "UTF-8 with BOM",
            Encoding::Utf16Le => "UTF-16 LE",
            Encoding::Utf16LeBom => "UTF-16 LE with BOM",
            Encoding::Utf16Be => "UTF-16 BE",
            Encoding::Utf16BeBom => "UTF-16 BE with BOM",
            Encoding::Ansi => "ANSI",
        }
    }

    /// Detect encoding from byte content
    pub fn detect(bytes: &[u8]) -> Self {
        // Check for BOM (Byte Order Mark)
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            return Encoding::Utf8Bom;
        }

        if bytes.len() >= 2 {
            if bytes[0] == 0xFF && bytes[1] == 0xFE {
                return Encoding::Utf16LeBom;
            }
            if bytes[0] == 0xFE && bytes[1] == 0xFF {
                return Encoding::Utf16BeBom;
            }
        }

        // Try to validate as UTF-8
        if std::str::from_utf8(bytes).is_ok() {
            return Encoding::Utf8;
        }

        // Check if it looks like UTF-16 LE (many null bytes in even positions)
        if bytes.len() >= 4 && Self::looks_like_utf16_le(bytes) {
            return Encoding::Utf16Le;
        }

        // Check if it looks like UTF-16 BE (many null bytes in odd positions)
        if bytes.len() >= 4 && Self::looks_like_utf16_be(bytes) {
            return Encoding::Utf16Be;
        }

        // Default to ANSI
        Encoding::Ansi
    }

    /// Check if bytes look like UTF-16 LE (Little Endian)
    fn looks_like_utf16_le(bytes: &[u8]) -> bool {
        if bytes.len() < 100 {
            return false;
        }

        let mut null_count_odd = 0;
        let sample_size = 100.min(bytes.len());

        for i in (1..sample_size).step_by(2) {
            if bytes[i] == 0 {
                null_count_odd += 1;
            }
        }

        // If more than 30% of odd-position bytes are null, likely UTF-16 LE
        null_count_odd > (sample_size / 2) * 3 / 10
    }

    /// Check if bytes look like UTF-16 BE (Big Endian)
    fn looks_like_utf16_be(bytes: &[u8]) -> bool {
        if bytes.len() < 100 {
            return false;
        }

        let mut null_count_even = 0;
        let sample_size = 100.min(bytes.len());

        for i in (0..sample_size).step_by(2) {
            if bytes[i] == 0 {
                null_count_even += 1;
            }
        }

        // If more than 30% of even-position bytes are null, likely UTF-16 BE
        null_count_even > (sample_size / 2) * 3 / 10
    }

    /// Read text file with encoding detection
    pub fn read_file_with_detection(path: &Path) -> std::io::Result<(String, Encoding)> {
        let bytes = std::fs::read(path)?;
        let encoding = Self::detect(&bytes);
        let text = Self::decode(&bytes, encoding)?;
        Ok((text, encoding))
    }

    /// Decode bytes to string using specified encoding
    pub fn decode(bytes: &[u8], encoding: Encoding) -> std::io::Result<String> {
        match encoding {
            Encoding::Utf8 => {
                String::from_utf8(bytes.to_vec())
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
            Encoding::Utf8Bom => {
                // Skip BOM (3 bytes)
                let data = if bytes.len() >= 3 { &bytes[3..] } else { bytes };
                String::from_utf8(data.to_vec())
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
            Encoding::Utf16Le | Encoding::Utf16LeBom => {
                // Skip BOM if present (2 bytes)
                let data = if matches!(encoding, Encoding::Utf16LeBom) && bytes.len() >= 2 {
                    &bytes[2..]
                } else {
                    bytes
                };

                // Convert bytes to u16 slice
                if data.len() % 2 != 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid UTF-16 data (odd length)",
                    ));
                }

                let u16_data: Vec<u16> = data
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();

                String::from_utf16(&u16_data)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
            Encoding::Utf16Be | Encoding::Utf16BeBom => {
                // Skip BOM if present (2 bytes)
                let data = if matches!(encoding, Encoding::Utf16BeBom) && bytes.len() >= 2 {
                    &bytes[2..]
                } else {
                    bytes
                };

                // Convert bytes to u16 slice
                if data.len() % 2 != 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid UTF-16 data (odd length)",
                    ));
                }

                let u16_data: Vec<u16> = data
                    .chunks_exact(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                    .collect();

                String::from_utf16(&u16_data)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
            Encoding::Ansi => {
                // For ANSI, we'll use lossy UTF-8 conversion
                // In a real implementation, you'd use platform-specific codepage conversion
                Ok(String::from_utf8_lossy(bytes).into_owned())
            }
        }
    }

    /// Encode string to bytes using specified encoding
    pub fn encode(text: &str, encoding: Encoding) -> Vec<u8> {
        match encoding {
            Encoding::Utf8 => text.as_bytes().to_vec(),
            Encoding::Utf8Bom => {
                let mut bytes = vec![0xEF, 0xBB, 0xBF]; // UTF-8 BOM
                bytes.extend_from_slice(text.as_bytes());
                bytes
            }
            Encoding::Utf16Le => {
                let u16_data: Vec<u16> = text.encode_utf16().collect();
                u16_data
                    .iter()
                    .flat_map(|&c| c.to_le_bytes())
                    .collect()
            }
            Encoding::Utf16LeBom => {
                let mut bytes = vec![0xFF, 0xFE]; // UTF-16 LE BOM
                let u16_data: Vec<u16> = text.encode_utf16().collect();
                bytes.extend(u16_data.iter().flat_map(|&c| c.to_le_bytes()));
                bytes
            }
            Encoding::Utf16Be => {
                let u16_data: Vec<u16> = text.encode_utf16().collect();
                u16_data
                    .iter()
                    .flat_map(|&c| c.to_be_bytes())
                    .collect()
            }
            Encoding::Utf16BeBom => {
                let mut bytes = vec![0xFE, 0xFF]; // UTF-16 BE BOM
                let u16_data: Vec<u16> = text.encode_utf16().collect();
                bytes.extend(u16_data.iter().flat_map(|&c| c.to_be_bytes()));
                bytes
            }
            Encoding::Ansi => {
                // For ANSI, just use UTF-8 bytes (simplified)
                // In a real implementation, you'd convert to platform codepage
                text.as_bytes().to_vec()
            }
        }
    }

    /// Write text to file with specified encoding
    pub fn write_file_with_encoding(
        path: &Path,
        text: &str,
        encoding: Encoding,
    ) -> std::io::Result<()> {
        let bytes = Self::encode(text, encoding);
        std::fs::write(path, bytes)
    }
}

/// Line ending types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Windows, // CRLF (\r\n)
    Unix,    // LF (\n)
    Mac,     // CR (\r) - legacy Mac OS
}

impl LineEnding {
    /// Get the display name for this line ending
    pub fn display_name(&self) -> &'static str {
        match self {
            LineEnding::Windows => "Windows (CRLF)",
            LineEnding::Unix => "Unix (LF)",
            LineEnding::Mac => "Mac (CR)",
        }
    }

    /// Get the short name for status bar
    pub fn short_name(&self) -> &'static str {
        match self {
            LineEnding::Windows => "CRLF",
            LineEnding::Unix => "LF",
            LineEnding::Mac => "CR",
        }
    }

    /// Detect line ending from text
    pub fn detect(text: &str) -> Self {
        let crlf_count = text.matches("\r\n").count();
        let lf_count = text.matches('\n').count() - crlf_count;
        let cr_count = text.matches('\r').count() - crlf_count;

        if crlf_count > lf_count && crlf_count > cr_count {
            LineEnding::Windows
        } else if lf_count > cr_count {
            LineEnding::Unix
        } else if cr_count > 0 {
            LineEnding::Mac
        } else {
            // Default to platform native
            #[cfg(windows)]
            return LineEnding::Windows;
            #[cfg(not(windows))]
            return LineEnding::Unix;
        }
    }

    /// Convert text to use this line ending
    pub fn convert(text: &str, target: LineEnding) -> String {
        // First normalize all line endings to \n
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");

        // Then convert to target
        match target {
            LineEnding::Windows => normalized.replace('\n', "\r\n"),
            LineEnding::Unix => normalized,
            LineEnding::Mac => normalized.replace('\n', "\r"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_detection() {
        // UTF-8 with BOM
        let utf8_bom = vec![0xEF, 0xBB, 0xBF, b'H', b'e', b'l', b'l', b'o'];
        assert_eq!(Encoding::detect(&utf8_bom), Encoding::Utf8Bom);

        // UTF-16 LE with BOM
        let utf16le_bom = vec![0xFF, 0xFE, b'H', 0, b'i', 0];
        assert_eq!(Encoding::detect(&utf16le_bom), Encoding::Utf16LeBom);

        // UTF-16 BE with BOM
        let utf16be_bom = vec![0xFE, 0xFF, 0, b'H', 0, b'i'];
        assert_eq!(Encoding::detect(&utf16be_bom), Encoding::Utf16BeBom);

        // Plain UTF-8
        let utf8 = b"Hello, world!";
        assert_eq!(Encoding::detect(utf8), Encoding::Utf8);
    }

    #[test]
    fn test_line_ending_detection() {
        assert_eq!(LineEnding::detect("Hello\r\nWorld\r\n"), LineEnding::Windows);
        assert_eq!(LineEnding::detect("Hello\nWorld\n"), LineEnding::Unix);
        assert_eq!(LineEnding::detect("Hello\rWorld\r"), LineEnding::Mac);
    }

    #[test]
    fn test_line_ending_conversion() {
        let text = "Line 1\nLine 2\nLine 3";
        assert_eq!(
            LineEnding::convert(text, LineEnding::Windows),
            "Line 1\r\nLine 2\r\nLine 3"
        );
        assert_eq!(
            LineEnding::convert(text, LineEnding::Unix),
            "Line 1\nLine 2\nLine 3"
        );
    }
}
