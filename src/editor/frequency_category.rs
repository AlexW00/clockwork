use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use nih_plug_egui::{egui, widgets};
use crate::{FrequencyType, GuiEditor, PluginParams, TinyArp};
use nih_plug::context::ParamSetter;
use nih_plug_egui::egui::{DragValue, popup_below_widget};
use crate::editor::numpad::Numpad;
use nih_plug::param::Param;
use nih_plug::prelude::Enum;

pub fn frequency_category (ui: &mut egui::Ui, setter: &ParamSetter, params: &Arc<PluginParams>, is_typing: &Arc<AtomicBool>) {
    let freq_type = params.freq_type.value();
    let freq_param = match freq_type {
        FrequencyType::Hertz => &params.freq_hz,
        FrequencyType::Milliseconds => &params.freq_ms,
        FrequencyType::Bpm => &params.freq_bpm,
    };
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add(widgets::ParamSlider::for_param(
                freq_param,
                setter,
            )
                .with_width(TinyArp::WINDOW_WIDTH as f32 * 0.75)
            );


            // Toggle for frequency type
            let text = FrequencyType::variants()[freq_type.clone().to_index()];
            if ui
                .add(egui::Button::new(text))
                .clicked()
            {
                if freq_type.clone().to_index() == FrequencyType::variants().len() - 1 {
                    setter.set_parameter(&params.freq_type, FrequencyType::from_index(0));
                } else {
                    setter.set_parameter(&params.freq_type, params.freq_type.next_step(freq_type.clone()))
                }
            }
        });
    });
}