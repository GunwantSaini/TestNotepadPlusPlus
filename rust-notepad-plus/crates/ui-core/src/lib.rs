//! Platform-independent UI abstractions for Notepad++ Rust Edition
//!
//! This crate defines traits and types that abstract over platform-specific UI implementations.
//! Concrete implementations exist in:
//! - `notepad-ui-windows`: Win32 API backend
//! - `notepad-ui-gtk`: GTK4 backend for Linux
//! - `notepad-ui-egui`: egui backend (optional cross-platform)

pub mod traits;
pub mod types;
pub mod encoding;
pub mod recent_files;
pub mod app_state;

pub use traits::*;
pub use types::*;
