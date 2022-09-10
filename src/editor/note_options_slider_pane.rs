use std::convert::TryInto;
use std::ops::RangeInclusive;
use std::sync::Arc;
use nih_plug::context::ParamSetter;
use nih_plug::prelude::{IntParam, FloatParam, BoolParam, Param};
use nih_plug_egui::egui::emath::{Numeric, self};
use nih_plug_egui::{egui, widgets};
use nih_plug_egui::egui::{Id, Layout, Response, Ui, Widget};
use crate::{NoteOptionsPanelType, PluginParams, TinyArp};
use crate::editor::gui::GuiEditor;

pub struct NoteOptionsSliderPane<'a> {
    pub setter: &'a ParamSetter<'a>,
    pub params: &'a Arc<PluginParams>,
    pub panel_type: NoteOptionsPanelType,
}

impl  <'a> NoteOptionsSliderPane <'a> {

    fn get_transpose_params (params: &'a Arc<PluginParams>) -> Vec<&IntParam> {
        vec![
            &params.transpose_1,
            &params.transpose_2,
            &params.transpose_3,
            &params.transpose_4,
            &params.transpose_5,
            &params.transpose_6,
            &params.transpose_7,
            &params.transpose_8,
            &params.transpose_9,
            &params.transpose_10,
            &params.transpose_11,
            &params.transpose_12,
            &params.transpose_13,
            &params.transpose_14,
            &params.transpose_15,
            &params.transpose_16,
        ]
    }

    fn get_velocity_params (params: &'a Arc<PluginParams>) -> Vec<&FloatParam> {
        vec![
            &params.velocity_1,
            &params.velocity_2,
            &params.velocity_3,
            &params.velocity_4,
            &params.velocity_5,
            &params.velocity_6,
            &params.velocity_7,
            &params.velocity_8,
            &params.velocity_9,
            &params.velocity_10,
            &params.velocity_11,
            &params.velocity_12,
            &params.velocity_13,
            &params.velocity_14,
            &params.velocity_15,
            &params.velocity_16,
        ]
    }

    fn get_enabled_params (params: &'a Arc<PluginParams>) -> Vec<&BoolParam> {
        vec![
            &params.enabled_1,
            &params.enabled_2,
            &params.enabled_3,
            &params.enabled_4,
            &params.enabled_5,
            &params.enabled_6,
            &params.enabled_7,
            &params.enabled_8,
            &params.enabled_9,
            &params.enabled_10,
            &params.enabled_11,
            &params.enabled_12,
            &params.enabled_13,
            &params.enabled_14,
            &params.enabled_15,
            &params.enabled_16,
        ]
    }

    fn build_transpose_panel (&self, ui: &mut Ui) {
        let transpose_params = Self::get_transpose_params(self.params);
        let enabled_params = Self::get_enabled_params(self.params);
        let num_enabled_notes = self.params.num_steps.value;

        ui.vertical(|ui| {
            ui.vertical(|ui| {  
                transpose_params.into_iter().enumerate().for_each(|(i, transpose_param)| {
                    if num_enabled_notes > i.try_into().unwrap_or(0) {
                        self.create_slider_row::<IntParam>(ui, transpose_param, enabled_params[i],i);
                    }
                });
            });

    });

    }

    fn create_slider_row <T>(
        &self,
        ui: &mut Ui,
        param: &T,
        enabled_param: &BoolParam,
        index: usize
    )  where T:Param {
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new(
                format!("Note {}{}", index + 1, if index + 1 > 9 { "" } else { "  " }),
            )).clicked() {
                self.setter.set_parameter(enabled_param, !enabled_param.value);
            };

            ui.add_enabled(enabled_param.value, widgets::ParamSlider::for_param(
                param,
                self.setter,
            )
                .with_width(TinyArp::WINDOW_WIDTH as f32 * 0.75)
            );

            if ui.add_enabled(enabled_param.value, egui::Button::new("X")).clicked() {
                self.setter.set_parameter(param, param.default_plain_value());
            }
        });
    }

    fn build_velocity_panel (&self, ui: &mut Ui) {
        let velocity_params = Self::get_velocity_params(self.params);
        let enabled_params = Self::get_enabled_params(self.params);
        let num_enabled_notes = self.params.num_steps.value;


        ui.vertical(|ui| {
            velocity_params.into_iter().enumerate().for_each(|(i, velocity_param)| {
                if num_enabled_notes > i.try_into().unwrap_or(0) {
                    self.create_slider_row::<FloatParam>(ui, velocity_param, enabled_params[i],i);
                }
            });
        });

    }

}

impl <'a> Widget for NoteOptionsSliderPane<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.with_layout(
            Layout::top_down(egui::Align::LEFT),
            |ui| {
                match self.panel_type {
                    NoteOptionsPanelType::Transpose => self.build_transpose_panel(ui),
                    NoteOptionsPanelType::Velocity => self.build_velocity_panel(ui),
                }
            }
        ).response
    }
}