//! Macro recorder for capturing user actions

use crate::action::MacroAction;
use crate::macro_def::Macro;
use std::sync::{Arc, RwLock};

/// State of the macro recorder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecorderState {
    /// Not recording
    Idle,

    /// Currently recording
    Recording,

    /// Recording paused
    Paused,
}

/// Macro recorder for capturing user actions
pub struct MacroRecorder {
    /// Current state
    state: RecorderState,

    /// Current macro being recorded
    current_macro: Option<Macro>,

    /// Recorded actions buffer
    actions: Vec<MacroAction>,

    /// Record timestamps
    record_timing: bool,

    /// Last action timestamp (for timing)
    last_action_time: Option<std::time::Instant>,
}

impl MacroRecorder {
    /// Create a new macro recorder
    pub fn new() -> Self {
        Self {
            state: RecorderState::Idle,
            current_macro: None,
            actions: Vec::new(),
            record_timing: false,
            last_action_time: None,
        }
    }

    /// Start recording a new macro
    pub fn start_recording(&mut self, name: String, description: String) -> Result<(), String> {
        if self.state != RecorderState::Idle {
            return Err("Already recording a macro".to_string());
        }

        self.current_macro = Some(Macro::new(name, description));
        self.actions.clear();
        self.state = RecorderState::Recording;
        self.last_action_time = Some(std::time::Instant::now());

        log::info!("Started recording macro");
        Ok(())
    }

    /// Stop recording and return the recorded macro
    pub fn stop_recording(&mut self) -> Result<Macro, String> {
        if self.state == RecorderState::Idle {
            return Err("Not currently recording".to_string());
        }

        let mut macro_def = self
            .current_macro
            .take()
            .ok_or_else(|| "No macro being recorded".to_string())?;

        // Add all recorded actions to the macro
        for action in self.actions.drain(..) {
            macro_def.add_action(action);
        }

        self.state = RecorderState::Idle;
        self.last_action_time = None;

        log::info!("Stopped recording macro: {} actions", macro_def.action_count());
        Ok(macro_def)
    }

    /// Pause recording
    pub fn pause_recording(&mut self) -> Result<(), String> {
        if self.state != RecorderState::Recording {
            return Err("Not currently recording".to_string());
        }

        self.state = RecorderState::Paused;
        log::info!("Paused macro recording");
        Ok(())
    }

    /// Resume recording
    pub fn resume_recording(&mut self) -> Result<(), String> {
        if self.state != RecorderState::Paused {
            return Err("Recording is not paused".to_string());
        }

        self.state = RecorderState::Recording;
        self.last_action_time = Some(std::time::Instant::now());
        log::info!("Resumed macro recording");
        Ok(())
    }

    /// Cancel recording and discard the macro
    pub fn cancel_recording(&mut self) {
        self.current_macro = None;
        self.actions.clear();
        self.state = RecorderState::Idle;
        self.last_action_time = None;
        log::info!("Cancelled macro recording");
    }

    /// Record an action
    pub fn record_action(&mut self, action: MacroAction) -> Result<(), String> {
        if self.state != RecorderState::Recording {
            return Err("Not currently recording".to_string());
        }

        // Calculate delay if timing is enabled
        if self.record_timing {
            if let Some(last_time) = self.last_action_time {
                let elapsed = last_time.elapsed();
                let millis = elapsed.as_millis() as u64;

                // Only record delays longer than 100ms
                if millis > 100 {
                    self.actions.push(MacroAction::Delay {
                        milliseconds: millis,
                    });
                }
            }
        }

        self.actions.push(action);
        self.last_action_time = Some(std::time::Instant::now());

        Ok(())
    }

    /// Get the current state
    pub fn state(&self) -> RecorderState {
        self.state
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        self.state == RecorderState::Recording
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.state == RecorderState::Paused
    }

    /// Get the number of recorded actions
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Enable or disable timing recording
    pub fn set_record_timing(&mut self, enabled: bool) {
        self.record_timing = enabled;
    }

    /// Check if timing recording is enabled
    pub fn is_recording_timing(&self) -> bool {
        self.record_timing
    }

    /// Get a preview of the current actions
    pub fn preview_actions(&self) -> &[MacroAction] {
        &self.actions
    }

    /// Clear all recorded actions (while still recording)
    pub fn clear_actions(&mut self) {
        self.actions.clear();
        log::info!("Cleared recorded actions");
    }

    /// Remove the last recorded action
    pub fn undo_last_action(&mut self) -> Option<MacroAction> {
        self.actions.pop()
    }
}

impl Default for MacroRecorder {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe macro recorder
pub type SharedRecorder = Arc<RwLock<MacroRecorder>>;

/// Create a shared macro recorder
pub fn create_shared_recorder() -> SharedRecorder {
    Arc::new(RwLock::new(MacroRecorder::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_creation() {
        let recorder = MacroRecorder::new();
        assert_eq!(recorder.state(), RecorderState::Idle);
        assert!(!recorder.is_recording());
        assert_eq!(recorder.action_count(), 0);
    }

    #[test]
    fn test_start_stop_recording() {
        let mut recorder = MacroRecorder::new();

        recorder
            .start_recording("Test".to_string(), "Test macro".to_string())
            .unwrap();
        assert!(recorder.is_recording());

        recorder
            .record_action(MacroAction::InsertText {
                text: "Hello".to_string(),
            })
            .unwrap();
        assert_eq!(recorder.action_count(), 1);

        let macro_def = recorder.stop_recording().unwrap();
        assert_eq!(macro_def.name, "Test");
        assert_eq!(macro_def.action_count(), 1);
        assert!(!recorder.is_recording());
    }

    #[test]
    fn test_pause_resume() {
        let mut recorder = MacroRecorder::new();

        recorder
            .start_recording("Test".to_string(), "Test".to_string())
            .unwrap();
        assert!(recorder.is_recording());

        recorder.pause_recording().unwrap();
        assert!(recorder.is_paused());
        assert!(!recorder.is_recording());

        // Cannot record while paused
        assert!(recorder
            .record_action(MacroAction::Copy)
            .is_err());

        recorder.resume_recording().unwrap();
        assert!(recorder.is_recording());

        recorder.record_action(MacroAction::Copy).unwrap();
        assert_eq!(recorder.action_count(), 1);
    }

    #[test]
    fn test_cancel_recording() {
        let mut recorder = MacroRecorder::new();

        recorder
            .start_recording("Test".to_string(), "Test".to_string())
            .unwrap();
        recorder
            .record_action(MacroAction::Copy)
            .unwrap();

        recorder.cancel_recording();
        assert_eq!(recorder.state(), RecorderState::Idle);
        assert_eq!(recorder.action_count(), 0);
    }

    #[test]
    fn test_timing_recording() {
        let mut recorder = MacroRecorder::new();
        assert!(!recorder.is_recording_timing());

        recorder.set_record_timing(true);
        assert!(recorder.is_recording_timing());

        recorder.set_record_timing(false);
        assert!(!recorder.is_recording_timing());
    }

    #[test]
    fn test_preview_actions() {
        let mut recorder = MacroRecorder::new();

        recorder
            .start_recording("Test".to_string(), "Test".to_string())
            .unwrap();
        recorder
            .record_action(MacroAction::Copy)
            .unwrap();
        recorder
            .record_action(MacroAction::Paste)
            .unwrap();

        let preview = recorder.preview_actions();
        assert_eq!(preview.len(), 2);
    }

    #[test]
    fn test_clear_actions() {
        let mut recorder = MacroRecorder::new();

        recorder
            .start_recording("Test".to_string(), "Test".to_string())
            .unwrap();
        recorder
            .record_action(MacroAction::Copy)
            .unwrap();

        recorder.clear_actions();
        assert_eq!(recorder.action_count(), 0);
        assert!(recorder.is_recording()); // Still recording
    }

    #[test]
    fn test_undo_last_action() {
        let mut recorder = MacroRecorder::new();

        recorder
            .start_recording("Test".to_string(), "Test".to_string())
            .unwrap();
        recorder
            .record_action(MacroAction::Copy)
            .unwrap();
        recorder
            .record_action(MacroAction::Paste)
            .unwrap();

        let last = recorder.undo_last_action();
        assert!(matches!(last, Some(MacroAction::Paste)));
        assert_eq!(recorder.action_count(), 1);
    }

    #[test]
    fn test_cannot_start_while_recording() {
        let mut recorder = MacroRecorder::new();

        recorder
            .start_recording("Test1".to_string(), "Test".to_string())
            .unwrap();

        let result = recorder.start_recording("Test2".to_string(), "Test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_stop_when_not_recording() {
        let mut recorder = MacroRecorder::new();
        let result = recorder.stop_recording();
        assert!(result.is_err());
    }

    #[test]
    fn test_shared_recorder() {
        let recorder = create_shared_recorder();

        {
            let mut rec = recorder.write().unwrap();
            rec.start_recording("Test".to_string(), "Test".to_string())
                .unwrap();
        }

        {
            let rec = recorder.read().unwrap();
            assert!(rec.is_recording());
        }
    }
}
