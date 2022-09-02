use std::sync::Arc;
use nih_plug::param::IntParam;
use nih_plug::prelude::IntRange;
use num_traits::ToPrimitive;

pub trait CategoricalIntParam<P>
where P : std::fmt::Display + num_traits::FromPrimitive + ToPrimitive + strum::EnumCount + strum::IntoEnumIterator + Default + CategoricalIntParam<P> {
    fn formatter() -> Arc<dyn Fn(i32) -> String + Send + Sync> {
        Arc::new(move |value| {
            if let Some(category) = P::from_i32(value) {
                category.to_string()
            } else {
                "Unknown".to_string()
            }
        })
    }

    fn count() -> i32 {
        P::iter().count() as i32 - 1
    }

    fn title() -> String;

    fn next(param: &P) -> P {
        let mut next = ToPrimitive::to_i32(param).unwrap() + 1;
        if next > Self::count() {
            next = 0;
        }
        P::from_i32(next).unwrap()
    }

    fn new_int_param() -> IntParam {
        IntParam::new(
            P::title(),
            ToPrimitive::to_i32(&P::default()).expect("Failed to convert default value to i32"),
            IntRange::Linear {
                min: 0,
                max: P::count()
            },
        ).with_value_to_string(P::formatter())
    }

    fn from_int_param(param: &IntParam) -> P {
        P::from_i32(param.value).unwrap_or(P::default())
    }

    fn into_int_param(param: &P) -> IntParam {
        let mut int_param = Self::new_int_param();
        int_param.value = ToPrimitive::to_i32(param).expect("Failed to convert value to i32");
        int_param
    }
}
