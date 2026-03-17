//! Macro playback engine

use crate::action::MacroAction;
use crate::macro_def::Macro;

/// Result of a macro playback
#[derive(Debug, Clone)]
pub enum PlaybackResult {
    /// Playback completed successfully
    Success {
        /// Number of actions executed
        actions_executed: usize,
    },

    /// Playback completed with warnings
    SuccessWithWarnings {
        /// Number of actions executed
        actions_executed: usize,
        /// Warning messages
        warnings: Vec<String>,
    },

    /// Playback was interrupted
    Interrupted {
        /// Number of actions executed before interruption
        actions_executed: usize,
    },

    /// Playback failed
    Failed {
        /// Number of actions executed before failure
        actions_executed: usize,
        /// Error message
        error: String,
    },
}

/// Handler trait for executing macro actions
///
/// Implementations of this trait provide the actual execution logic for actions
pub trait ActionHandler: Send + Sync {
    /// Execute an action
    fn execute_action(&mut self, action: &MacroAction) -> Result<(), String>;

    /// Called before playback starts
    fn on_playback_start(&mut self) {}

    /// Called after playback completes
    fn on_playback_complete(&mut self) {}

    /// Called when an action is about to be executed
    fn on_action_start(&mut self, _action: &MacroAction) {}

    /// Called after an action has been executed
    fn on_action_complete(&mut self, _action: &MacroAction) {}

    /// Check if playback should be interrupted
    fn should_interrupt(&self) -> bool {
        false
    }
}

/// Playback options
#[derive(Debug, Clone)]
pub struct PlaybackOptions {
    /// Number of times to repeat the macro
    pub repeat_count: usize,

    /// Whether to stop on first error
    pub stop_on_error: bool,

    /// Whether to respect timing delays
    pub respect_timing: bool,

    /// Speed multiplier for timing (1.0 = normal, 2.0 = double speed)
    pub speed_multiplier: f64,

    /// Whether to allow interruption
    pub allow_interruption: bool,
}

impl Default for PlaybackOptions {
    fn default() -> Self {
        Self {
            repeat_count: 1,
            stop_on_error: true,
            respect_timing: false,
            speed_multiplier: 1.0,
            allow_interruption: true,
        }
    }
}

impl PlaybackOptions {
    /// Create options for single playback
    pub fn once() -> Self {
        Self::default()
    }

    /// Create options for repeated playback
    pub fn repeat(count: usize) -> Self {
        Self {
            repeat_count: count,
            ..Self::default()
        }
    }

    /// Create options with timing
    pub fn with_timing() -> Self {
        Self {
            respect_timing: true,
            ..Self::default()
        }
    }

    /// Set repeat count
    pub fn set_repeat(mut self, count: usize) -> Self {
        self.repeat_count = count;
        self
    }

    /// Set stop on error
    pub fn set_stop_on_error(mut self, stop: bool) -> Self {
        self.stop_on_error = stop;
        self
    }

    /// Set timing respect
    pub fn set_respect_timing(mut self, respect: bool) -> Self {
        self.respect_timing = respect;
        self
    }

    /// Set speed multiplier
    pub fn set_speed(mut self, speed: f64) -> Self {
        self.speed_multiplier = speed.max(0.1).min(10.0);
        self
    }
}

/// Macro playback engine
pub struct MacroPlayback {
    /// Current playback state
    is_playing: bool,

    /// Total actions executed
    total_actions_executed: usize,
}

impl MacroPlayback {
    /// Create a new playback engine
    pub fn new() -> Self {
        Self {
            is_playing: false,
            total_actions_executed: 0,
        }
    }

    /// Play a macro
    pub fn play(
        &mut self,
        macro_def: &Macro,
        handler: &mut dyn ActionHandler,
        options: PlaybackOptions,
    ) -> PlaybackResult {
        if self.is_playing {
            return PlaybackResult::Failed {
                actions_executed: 0,
                error: "Already playing a macro".to_string(),
            };
        }

        self.is_playing = true;
        self.total_actions_executed = 0;

        handler.on_playback_start();

        let mut warnings = Vec::new();
        let result = self.play_internal(macro_def, handler, &options, &mut warnings);

        handler.on_playback_complete();
        self.is_playing = false;

        result
    }

    /// Internal playback implementation
    fn play_internal(
        &mut self,
        macro_def: &Macro,
        handler: &mut dyn ActionHandler,
        options: &PlaybackOptions,
        warnings: &mut Vec<String>,
    ) -> PlaybackResult {
        for _ in 0..options.repeat_count {
            for action in &macro_def.actions {
                // Check for interruption
                if options.allow_interruption && handler.should_interrupt() {
                    return PlaybackResult::Interrupted {
                        actions_executed: self.total_actions_executed,
                    };
                }

                handler.on_action_start(action);

                // Handle timing delays
                if options.respect_timing {
                    if let MacroAction::Delay { milliseconds } = action {
                        let adjusted_delay =
                            (*milliseconds as f64 / options.speed_multiplier) as u64;
                        std::thread::sleep(std::time::Duration::from_millis(adjusted_delay));
                        handler.on_action_complete(action);
                        self.total_actions_executed += 1;
                        continue;
                    }
                }

                // Execute the action
                match handler.execute_action(action) {
                    Ok(()) => {
                        handler.on_action_complete(action);
                        self.total_actions_executed += 1;
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to execute action: {}", e);
                        if options.stop_on_error {
                            return PlaybackResult::Failed {
                                actions_executed: self.total_actions_executed,
                                error: error_msg,
                            };
                        } else {
                            warnings.push(error_msg);
                        }
                    }
                }
            }
        }

        if warnings.is_empty() {
            PlaybackResult::Success {
                actions_executed: self.total_actions_executed,
            }
        } else {
            PlaybackResult::SuccessWithWarnings {
                actions_executed: self.total_actions_executed,
                warnings: warnings.clone(),
            }
        }
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Get total actions executed
    pub fn total_actions_executed(&self) -> usize {
        self.total_actions_executed
    }

    /// Play a single action
    pub fn play_action(
        &mut self,
        action: &MacroAction,
        handler: &mut dyn ActionHandler,
    ) -> Result<(), String> {
        handler.on_action_start(action);
        let result = handler.execute_action(action);
        handler.on_action_complete(action);

        if result.is_ok() {
            self.total_actions_executed += 1;
        }

        result
    }
}

impl Default for MacroPlayback {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock action handler for testing
#[cfg(test)]
pub struct MockActionHandler {
    executed_actions: Vec<MacroAction>,
    should_fail: bool,
    should_interrupt: bool,
}

#[cfg(test)]
impl MockActionHandler {
    pub fn new() -> Self {
        Self {
            executed_actions: Vec::new(),
            should_fail: false,
            should_interrupt: false,
        }
    }

    pub fn with_failure() -> Self {
        Self {
            executed_actions: Vec::new(),
            should_fail: true,
            should_interrupt: false,
        }
    }

    pub fn with_interruption() -> Self {
        Self {
            executed_actions: Vec::new(),
            should_fail: false,
            should_interrupt: true,
        }
    }

    pub fn executed_count(&self) -> usize {
        self.executed_actions.len()
    }
}

#[cfg(test)]
impl ActionHandler for MockActionHandler {
    fn execute_action(&mut self, action: &MacroAction) -> Result<(), String> {
        if self.should_fail {
            return Err("Simulated failure".to_string());
        }

        self.executed_actions.push(action.clone());
        Ok(())
    }

    fn should_interrupt(&self) -> bool {
        self.should_interrupt && self.executed_actions.len() >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_options_default() {
        let options = PlaybackOptions::default();
        assert_eq!(options.repeat_count, 1);
        assert!(options.stop_on_error);
        assert!(!options.respect_timing);
        assert_eq!(options.speed_multiplier, 1.0);
    }

    #[test]
    fn test_playback_options_builder() {
        let options = PlaybackOptions::once()
            .set_repeat(3)
            .set_stop_on_error(false)
            .set_respect_timing(true)
            .set_speed(2.0);

        assert_eq!(options.repeat_count, 3);
        assert!(!options.stop_on_error);
        assert!(options.respect_timing);
        assert_eq!(options.speed_multiplier, 2.0);
    }

    #[test]
    fn test_playback_success() {
        let mut playback = MacroPlayback::new();
        let mut handler = MockActionHandler::new();

        let macro_def = Macro::with_actions(
            "Test".to_string(),
            "Test".to_string(),
            vec![MacroAction::Copy, MacroAction::Paste],
        );

        let result = playback.play(&macro_def, &mut handler, PlaybackOptions::once());

        match result {
            PlaybackResult::Success { actions_executed } => {
                assert_eq!(actions_executed, 2);
            }
            _ => panic!("Expected success"),
        }

        assert_eq!(handler.executed_count(), 2);
    }

    #[test]
    fn test_playback_repeat() {
        let mut playback = MacroPlayback::new();
        let mut handler = MockActionHandler::new();

        let macro_def = Macro::with_actions(
            "Test".to_string(),
            "Test".to_string(),
            vec![MacroAction::Copy],
        );

        let result = playback.play(&macro_def, &mut handler, PlaybackOptions::repeat(3));

        match result {
            PlaybackResult::Success { actions_executed } => {
                assert_eq!(actions_executed, 3);
            }
            _ => panic!("Expected success"),
        }
    }

    #[test]
    fn test_playback_failure() {
        let mut playback = MacroPlayback::new();
        let mut handler = MockActionHandler::with_failure();

        let macro_def = Macro::with_actions(
            "Test".to_string(),
            "Test".to_string(),
            vec![MacroAction::Copy, MacroAction::Paste],
        );

        let result = playback.play(&macro_def, &mut handler, PlaybackOptions::once());

        match result {
            PlaybackResult::Failed {
                actions_executed,
                error,
            } => {
                assert_eq!(actions_executed, 0);
                assert!(error.contains("Simulated failure"));
            }
            _ => panic!("Expected failure"),
        }
    }

    #[test]
    fn test_playback_continue_on_error() {
        let mut playback = MacroPlayback::new();
        let mut handler = MockActionHandler::with_failure();

        let macro_def = Macro::with_actions(
            "Test".to_string(),
            "Test".to_string(),
            vec![MacroAction::Copy, MacroAction::Paste],
        );

        let options = PlaybackOptions::once().set_stop_on_error(false);
        let result = playback.play(&macro_def, &mut handler, options);

        match result {
            PlaybackResult::SuccessWithWarnings {
                actions_executed,
                warnings,
            } => {
                assert_eq!(actions_executed, 0);
                assert_eq!(warnings.len(), 2);
            }
            _ => panic!("Expected success with warnings"),
        }
    }

    #[test]
    fn test_playback_interruption() {
        let mut playback = MacroPlayback::new();
        let mut handler = MockActionHandler::with_interruption();

        let macro_def = Macro::with_actions(
            "Test".to_string(),
            "Test".to_string(),
            vec![
                MacroAction::Copy,
                MacroAction::Paste,
                MacroAction::Cut,
            ],
        );

        let result = playback.play(&macro_def, &mut handler, PlaybackOptions::once());

        match result {
            PlaybackResult::Interrupted { actions_executed } => {
                assert!(actions_executed >= 2);
            }
            _ => panic!("Expected interruption"),
        }
    }

    #[test]
    fn test_play_single_action() {
        let mut playback = MacroPlayback::new();
        let mut handler = MockActionHandler::new();

        let action = MacroAction::Copy;
        playback.play_action(&action, &mut handler).unwrap();

        assert_eq!(playback.total_actions_executed(), 1);
        assert_eq!(handler.executed_count(), 1);
    }

    #[test]
    fn test_cannot_play_while_playing() {
        let mut playback = MacroPlayback::new();
        let mut handler = MockActionHandler::new();

        let macro_def = Macro::with_actions(
            "Test".to_string(),
            "Test".to_string(),
            vec![MacroAction::Copy],
        );

        playback.is_playing = true; // Simulate already playing

        let result = playback.play(&macro_def, &mut handler, PlaybackOptions::once());

        match result {
            PlaybackResult::Failed { error, .. } => {
                assert!(error.contains("Already playing"));
            }
            _ => panic!("Expected failure"),
        }
    }
}
