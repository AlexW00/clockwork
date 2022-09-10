use std::sync::Arc;
use nih_plug::context::ParamSetter;
use nih_plug_egui::egui;
use crate::{PluginParams, TriggerMode};
use nih_plug::param::Param;
use nih_plug::prelude::Enum;

pub fn trigger_mode_category (ui: &mut egui::Ui, setter: &ParamSetter, params: &Arc<PluginParams>) {
    let trigger_mode = params.trigger_mode.value();
    let text = TriggerMode::variants()[trigger_mode.clone().to_index()];
    ui.horizontal(|ui| {
        if ui
            .add(egui::Button::new(
                text
            ))
            .clicked()
        {
            if trigger_mode.clone().to_index() == TriggerMode::variants().len() - 1 {
                setter.set_parameter(&params.trigger_mode, TriggerMode::from_index(0));
            } else {
                setter.set_parameter(&params.trigger_mode, params.trigger_mode.next_step(trigger_mode.clone()));
            }
        }
        let label = egui::Label::new(trigger_mode.description()).wrap(true);
        ui.add(label);
    });

}