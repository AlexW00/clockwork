use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use strum_macros::{EnumCount, EnumIter};

use std::fmt::Display;
use crate::params::categorical_int_param::{CategoricalIntParam};

#[derive(FromPrimitive, ToPrimitive, Clone, Copy, EnumCount, EnumIter, Default)]
pub enum TriggerMode {
    #[default]
    Continue,
    ReTrigger,
    ReTriggerDelayed,
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

impl TriggerMode {
    pub fn description(&self) -> &'static str {
        match self {
            TriggerMode::Continue => "MIDI notes have no effect on the repetition loop",
            TriggerMode::ReTrigger => "MIDI notes re-trigger the repetition loop",
            TriggerMode::ReTriggerDelayed => "MIDI notes re-trigger the repetition loop with an initial delay ( = frequency)",
        }
    }
}

impl CategoricalIntParam<TriggerMode> for TriggerMode {
    fn title() -> String {
        "Trigger mode".to_string()
    }
}