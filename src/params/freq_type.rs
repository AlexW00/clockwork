use nih_plug::prelude::Enum;

#[derive(PartialEq, Enum, Clone)]
pub enum FrequencyType {
    #[name="Hz"]
    Hertz,
    #[name="ms"]
    Milliseconds,
    #[name="bpm"]
    Bpm
}