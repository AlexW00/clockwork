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

    fn int_param () -> IntParam {
        IntParam::new(
            P::title(),
            ToPrimitive::to_i32(&P::default()).expect("Failed to convert default value to i32"),
            IntRange::Linear {
                min: 0,
                max: P::count()
            },
        ).with_value_to_string(P::formatter())
    }
}