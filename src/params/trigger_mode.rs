use std::fmt::Display;

use nih_plug::prelude::Enum;

#[derive(PartialEq, Enum, Clone)]
pub enum TriggerMode {
    #[name = "Continue"]
    Continue,
    #[name = "Re-trigger"]
    ReTrigger,
    #[name = "Re-trigger delayed"]
    ReTriggerDelayed,
}

impl TriggerMode {
    pub fn description(&self) -> &'static str {
        match self {
            TriggerMode::Continue => "MIDI notes have no effect on the repetition loop",
            TriggerMode::ReTrigger => "MIDI notes re-trigger the repetition loop",
            TriggerMode::ReTriggerDelayed => {
                "MIDI notes re-trigger the repetition loop with an initial delay ( = frequency)"
            }
        }
    }
}

impl Display for TriggerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TriggerMode::Continue => write!(f, "Continue"),
            TriggerMode::ReTrigger => write!(f, "Re-trigger"),
            TriggerMode::ReTriggerDelayed => write!(f, "Re-trigger delayed"),
        }
    }
}
