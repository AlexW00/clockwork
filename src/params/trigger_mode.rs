use nih_plug::prelude::Enum;

#[derive(PartialEq, Enum, Clone)]
pub enum TriggerMode {
    #[name="Continue"]
    Continue,
    #[name="Re-trigger"]
    ReTrigger,
    #[name="Re-trigger delayed"]
    ReTriggerDelayed,
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
