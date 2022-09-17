use crate::{PluginParams, TriggerMode};
use nih_plug::context::ParamSetter;

use nih_plug::prelude::EnumParam;
use nih_plug_egui::egui::{self, Layout, Response, Ui};
use std::sync::Arc;

use super::atomics::reset_button::ResetButton;

pub struct TriggerModePanel<'a> {
    pub setter: &'a ParamSetter<'a>,
    pub params: &'a Arc<PluginParams>,
}

impl<'a> TriggerModePanel<'a> {
    fn add_nav_item(
        freq_type_param: &EnumParam<TriggerMode>,
        setter: &'a ParamSetter<'a>,
        this_freq_type: TriggerMode,
        ui: &mut Ui,
    ) -> Response {
        let is_selected = freq_type_param.value() == this_freq_type;
        let widget = ui.add(egui::SelectableLabel::new(
            is_selected,
            this_freq_type.to_string(),
        ));
        if widget.clicked() {
            setter.set_parameter(freq_type_param, this_freq_type);
        }
        widget
    }
}

impl<'a> egui::Widget for TriggerModePanel<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let trigger_mode = self.params.trigger_mode.value();

        ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
            ui.group(|ui| {
                // Navbar to select freq types

                ui.horizontal(|ui| {
                    Self::add_nav_item(
                        &self.params.trigger_mode,
                        self.setter,
                        TriggerMode::Continue,
                        ui,
                    );
                    Self::add_nav_item(
                        &self.params.trigger_mode,
                        self.setter,
                        TriggerMode::ReTrigger,
                        ui,
                    );
                    Self::add_nav_item(
                        &self.params.trigger_mode,
                        self.setter,
                        TriggerMode::ReTriggerDelayed,
                        ui,
                    );

                    ui.add_space(ui.available_width() - 16.0);
                    ui.add(ResetButton::new(
                        self.setter,
                        Some(vec![&self.params.trigger_mode]),
                        None,
                    ))
                });

                ui.separator();

                let label = egui::Label::new(trigger_mode.description()).wrap(true);
                ui.add(label);
            });
        })
        .response
    }
}
