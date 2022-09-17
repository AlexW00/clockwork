use crate::editor::gui::GuiEditor;
use crate::params::trigger_mode;
use crate::{NoteOptionsPanelType, PluginParams, TinyArp};
use nih_plug::context::ParamSetter;
use nih_plug::prelude::{BoolParam, FloatParam, IntParam, Param};
use nih_plug_egui::egui::emath::{self, Numeric};
use nih_plug_egui::egui::{Id, Layout, Response, Ui, Widget};
use nih_plug_egui::{egui, widgets};
use std::convert::TryInto;
use std::ops::RangeInclusive;
use std::sync::Arc;

use super::atomics::reset_button::ResetButton;

pub struct NoteOptionsSliderPane<'a> {
    pub setter: &'a ParamSetter<'a>,
    pub params: &'a Arc<PluginParams>,
    pub panel_type: NoteOptionsPanelType,
}

impl<'a> NoteOptionsSliderPane<'a> {
    fn build_transpose_panel(&self, ui: &mut Ui) {
        let transpose_params = TinyArp::get_transpose_params(self.params);
        let enabled_params = TinyArp::get_enabled_params(self.params);
        let num_enabled_notes = self.params.num_steps.value();

        ui.vertical(|ui| {
            ui.vertical(|ui| {
                transpose_params
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, transpose_param)| {
                        if num_enabled_notes > i.try_into().unwrap_or(0) {
                            self.create_slider_row::<IntParam>(
                                ui,
                                transpose_param,
                                enabled_params[i],
                                i,
                            );
                        }
                    });
            });
        });
    }

    fn create_slider_row<T>(&self, ui: &mut Ui, param: &T, enabled_param: &BoolParam, index: usize)
    where
        T: Param,
    {
        ui.horizontal(|ui| {
            let mut note_button = egui::Button::new(format!(
                "{}Note {}{}",
                if index + 1 > 9 { " " } else { "  " },
                index + 1,
                if index + 1 > 9 { " " } else { "  " }
            ));

            if !enabled_param.value() {
                note_button = note_button.fill(egui::Color32::TRANSPARENT);
            }

            if ui.add(note_button).clicked() {
                self.setter
                    .set_parameter(enabled_param, !enabled_param.value());
            };

            ui.add_enabled(
                enabled_param.value(),
                widgets::ParamSlider::for_param(param, self.setter)
                    .with_width(TinyArp::WINDOW_WIDTH as f32 * 0.7),
            );

            // TODO: Make dynamic
            ui.add_space(ui.available_width() - 16.0);
            ui.add_enabled(
                enabled_param.value(),
                ResetButton::new(self.setter, Some(vec![param]), None),
            );
        });
    }

    fn build_velocity_panel(&self, ui: &mut Ui) {
        let velocity_params = TinyArp::get_velocity_params(self.params);
        let enabled_params = TinyArp::get_enabled_params(self.params);
        let num_enabled_notes = self.params.num_steps.value();

        ui.vertical(|ui| {
            velocity_params
                .into_iter()
                .enumerate()
                .for_each(|(i, velocity_param)| {
                    if num_enabled_notes > i.try_into().unwrap_or(0) {
                        self.create_slider_row::<FloatParam>(
                            ui,
                            velocity_param,
                            enabled_params[i],
                            i,
                        );
                    }
                });
        });
    }
}

impl<'a> Widget for NoteOptionsSliderPane<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
            match self.panel_type {
                NoteOptionsPanelType::Transpose => self.build_transpose_panel(ui),
                NoteOptionsPanelType::Velocity => self.build_velocity_panel(ui),
            }
        })
        .response
    }
}
