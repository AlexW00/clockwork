use crate::{FrequencyType, GuiEditor, PluginParams, TinyArp};
use nih_plug::context::ParamSetter;
use nih_plug::prelude::EnumParam;
use nih_plug_egui::egui::{Layout, Response, Ui, Widget};
use nih_plug_egui::{egui, widgets};
use std::sync::Arc;

use super::atomics::reset_button::ResetButton;

pub struct FrequencyPanel<'a> {
    pub setter: &'a ParamSetter<'a>,
    pub params: &'a Arc<PluginParams>,
}

impl<'a> FrequencyPanel<'a> {
    fn add_nav_item(
        freq_type_param: &EnumParam<FrequencyType>,
        setter: &'a ParamSetter<'a>,
        this_freq_type: FrequencyType,
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

impl<'a> Widget for FrequencyPanel<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let freq_type = self.params.freq_type.value();
        let freq_param = match freq_type {
            FrequencyType::Hertz => &self.params.freq_hz,
            FrequencyType::Milliseconds => &self.params.freq_ms,
            FrequencyType::Bpm => &self.params.freq_bpm,
        };
        ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
            ui.group(|ui| {
                // Navbar to select freq types

                ui.horizontal(|ui| {
                    Self::add_nav_item(
                        &self.params.freq_type,
                        self.setter,
                        FrequencyType::Hertz,
                        ui,
                    );
                    Self::add_nav_item(
                        &self.params.freq_type,
                        self.setter,
                        FrequencyType::Milliseconds,
                        ui,
                    );
                    Self::add_nav_item(&self.params.freq_type, self.setter, FrequencyType::Bpm, ui);
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Frequency");

                    ui.add(
                        widgets::ParamSlider::for_param(freq_param, &self.setter)
                            .with_width(TinyArp::WINDOW_WIDTH as f32 * 0.7),
                    );

                    // TODO: make dynamic
                    ui.add_space(ui.available_width() - 16.0);
                    ui.add(ResetButton::new(self.setter, Some(vec![freq_param]), None));
                });
            });
        })
        .response
    }
}
