use std::borrow::Cow;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use strum_macros::{EnumCount, EnumIter};
use strum::{EnumCount, IntoEnumIterator};

use std::fmt::Display;
use crate::CategoricalIntParam;

#[derive(FromPrimitive, ToPrimitive, Clone, Copy, EnumCount, EnumIter, Default)]
pub enum FrequencyType {
    Hertz,
    #[default]
    Milliseconds,
}

impl Display for FrequencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyType::Hertz => write!(f, "Hz"),
            FrequencyType::Milliseconds => write!(f, "ms"),
        }
    }
}

impl CategoricalIntParam<FrequencyType> for FrequencyType {
    fn title() -> String {
        "Frequency type".to_string()
    }
}