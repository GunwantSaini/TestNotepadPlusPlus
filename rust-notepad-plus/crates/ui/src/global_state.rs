//! Global application state singleton

use crate::app_state::AppState;
use std::sync::{Mutex, OnceLock};

/// Global application state
static GLOBAL_STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

/// Initialize the global state
pub fn init_global_state() {
    GLOBAL_STATE.get_or_init(|| Mutex::new(AppState::new()));
}

/// Get a reference to the global state
pub fn get_global_state() -> &'static Mutex<AppState> {
    GLOBAL_STATE.get().expect("Global state not initialized")
}

/// Update the global state with a closure
pub fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut AppState) -> R,
{
    let mut state = get_global_state().lock().unwrap();
    f(&mut *state)
}

/// Read from the global state with a closure
pub fn read_state<F, R>(f: F) -> R
where
    F: FnOnce(&AppState) -> R,
{
    let state = get_global_state().lock().unwrap();
    f(&*state)
}
