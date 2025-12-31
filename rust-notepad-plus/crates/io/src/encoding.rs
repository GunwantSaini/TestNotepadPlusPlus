//! Character encoding detection and conversion

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Utf8Bom,
    Utf16Le,
    Utf16LeBom,
    Utf16Be,
    Utf16BeBom,
    Windows1252,
    Ascii,
}

/// Detect encoding from byte array
pub fn detect_encoding(bytes: &[u8]) -> Encoding {
    // Check for BOM
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Encoding::Utf8Bom;
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return Encoding::Utf16LeBom;
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return Encoding::Utf16BeBom;
    }

    // Try UTF-8
    if std::str::from_utf8(bytes).is_ok() {
        return Encoding::Utf8;
    }

    // Use chardetng for detection
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);

    match encoding.name() {
        "UTF-8" => Encoding::Utf8,
        "UTF-16LE" => Encoding::Utf16Le,
        "UTF-16BE" => Encoding::Utf16Be,
        "windows-1252" => Encoding::Windows1252,
        _ => Encoding::Windows1252, // Default
    }
}
