//! File I/O operations and encoding detection

pub mod encoding;
pub mod watcher;

pub use encoding::{detect_encoding, Encoding};
pub use watcher::FileWatcher;
