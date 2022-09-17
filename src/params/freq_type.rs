use std::fmt::Display;

use nih_plug::prelude::Enum;

#[derive(PartialEq, Enum, Clone)]
pub enum FrequencyType {
    #[name = "Hz"]
    Hertz,
    #[name = "Ms"]
    Milliseconds,
    #[name = "Bpm"]
    Bpm,
}

impl Display for FrequencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyType::Hertz => write!(f, "Hz"),
            FrequencyType::Milliseconds => write!(f, "Ms"),
            FrequencyType::Bpm => write!(f, "Bpm"),
        }
    }
}
