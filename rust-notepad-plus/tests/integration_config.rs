//! Integration tests for application configuration

use notepad_core::{NotepadApp, EolFormat, EncodingType};

#[test]
fn test_default_config() {
    let app = NotepadApp::new().expect("Failed to create app");
    let config = app.config();

    // Verify defaults
    assert_eq!(config.tab_size, 4);
    assert!(!config.use_spaces); // Default to tabs
    assert_eq!(config.default_eol, EolFormat::Windows);
    assert_eq!(config.default_encoding, EncodingType::Utf8);
    assert!(config.backup_enabled);
    assert!(config.remember_session);
    assert!(!config.multi_instance);
}

#[test]
fn test_update_tab_size() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Update tab size
    app.update_config(|config| {
        config.tab_size = 2;
    });

    let config = app.config();
    assert_eq!(config.tab_size, 2);
}

#[test]
fn test_update_use_spaces() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Change to spaces
    app.update_config(|config| {
        config.use_spaces = true;
    });

    let config = app.config();
    assert!(config.use_spaces);
}

#[test]
fn test_update_eol_format() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Change to Unix line endings
    app.update_config(|config| {
        config.default_eol = EolFormat::Unix;
    });

    let config = app.config();
    assert_eq!(config.default_eol, EolFormat::Unix);
}

#[test]
fn test_update_encoding() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Change to UTF-16
    app.update_config(|config| {
        config.default_encoding = EncodingType::Utf16Le;
    });

    let config = app.config();
    assert_eq!(config.default_encoding, EncodingType::Utf16Le);
}

#[test]
fn test_multiple_config_updates() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Update multiple settings at once
    app.update_config(|config| {
        config.tab_size = 8;
        config.use_spaces = true;
        config.default_eol = EolFormat::Mac;
        config.backup_enabled = false;
    });

    let config = app.config();
    assert_eq!(config.tab_size, 8);
    assert!(config.use_spaces);
    assert_eq!(config.default_eol, EolFormat::Mac);
    assert!(!config.backup_enabled);
}

#[test]
fn test_config_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let app = Arc::new(NotepadApp::new().expect("Failed to create app"));

    // Spawn multiple threads updating config
    let mut handles = vec![];

    for i in 0..10 {
        let app_clone = Arc::clone(&app);
        let handle = thread::spawn(move || {
            app_clone.update_config(|config| {
                config.tab_size = (i % 4 + 1) * 2; // 2, 4, 6, or 8
            });
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify config is still valid
    let config = app.config();
    assert!(config.tab_size >= 2 && config.tab_size <= 8);
}

#[test]
fn test_backup_config() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Disable backup
    app.update_config(|config| {
        config.backup_enabled = false;
    });
    assert!(!app.config().backup_enabled);

    // Enable backup
    app.update_config(|config| {
        config.backup_enabled = true;
    });
    assert!(app.config().backup_enabled);
}

#[test]
fn test_session_config() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Test remember session toggle
    app.update_config(|config| {
        config.remember_session = false;
    });
    assert!(!app.config().remember_session);

    // Test session path
    app.update_config(|config| {
        config.session_path = Some(std::path::PathBuf::from("/tmp/session.xml"));
    });
    assert!(app.config().session_path.is_some());
}

#[test]
fn test_multi_instance_mode() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Enable multi-instance
    app.update_config(|config| {
        config.multi_instance = true;
    });
    assert!(app.config().multi_instance);

    // Disable multi-instance
    app.update_config(|config| {
        config.multi_instance = false;
    });
    assert!(!app.config().multi_instance);
}

#[test]
fn test_all_eol_formats() {
    let app = NotepadApp::new().expect("Failed to create app");

    // Test Windows
    app.update_config(|config| config.default_eol = EolFormat::Windows);
    assert_eq!(app.config().default_eol, EolFormat::Windows);

    // Test Unix
    app.update_config(|config| config.default_eol = EolFormat::Unix);
    assert_eq!(app.config().default_eol, EolFormat::Unix);

    // Test Mac
    app.update_config(|config| config.default_eol = EolFormat::Mac);
    assert_eq!(app.config().default_eol, EolFormat::Mac);
}

#[test]
fn test_all_encoding_types() {
    let app = NotepadApp::new().expect("Failed to create app");

    let encodings = vec![
        EncodingType::Ansi,
        EncodingType::Utf8,
        EncodingType::Utf8Bom,
        EncodingType::Utf16Le,
        EncodingType::Utf16LeBom,
        EncodingType::Utf16Be,
        EncodingType::Utf16BeBom,
    ];

    for encoding in encodings {
        app.update_config(|config| config.default_encoding = encoding);
        assert_eq!(app.config().default_encoding, encoding);
    }
}
