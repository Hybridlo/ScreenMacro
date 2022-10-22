use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub step_timeout_seconds: u64,
    pub break_whole_macro: bool
}

impl Default for Settings {
    fn default() -> Self {
        Self { step_timeout_seconds: 60, break_whole_macro: true }
    }
}

impl Settings {
    pub fn new(step_timeout_seconds: u64, break_whole_macro: bool) -> Self {
        Settings { step_timeout_seconds, break_whole_macro }
    }
}