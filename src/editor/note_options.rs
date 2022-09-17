use crate::editor::note_options_slider_pane::NoteOptionsSliderPane;
use crate::{PluginParams, TinyArp};
use nih_plug::context::ParamSetter;
use nih_plug::prelude::{Enum, IntParam, Param};
use nih_plug_egui::egui::{Id, Layout, Response, Ui, Widget};
use nih_plug_egui::{egui, widgets};
use std::fmt::Display;
use std::sync::Arc;

use super::atomics::reset_button::ResetButton;

#[derive(PartialEq, Enum, Clone, Copy)]
pub enum NoteOptionsPanelType {
    #[name = "Transpose"]
    Transpose,
    #[name = "Velocity"]
    Velocity,
}

impl Display for NoteOptionsPanelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteOptionsPanelType::Transpose => write!(f, "Transpose"),
            NoteOptionsPanelType::Velocity => write!(f, "Velocity"),
        }
    }
}

pub struct NoteOptions<'a> {
    pub setter: &'a ParamSetter<'a>,
    pub params: &'a Arc<PluginParams>,
}

impl<'a> NoteOptions<'a> {
    fn add_nav_item(
        open_panel: NoteOptionsPanelType,
        panel_type: NoteOptionsPanelType,
        ui: &mut Ui,
        id: Id,
    ) -> Response {
        let widget = ui.add(egui::SelectableLabel::new(
            open_panel == panel_type,
            panel_type.to_string(),
        ));
        if widget.clicked() {
            ui.memory().data.insert_temp(id, panel_type);
        }
        widget
    }
}

impl<'a> Widget for NoteOptions<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let note_options_panel = ui.make_persistent_id("note_options_panel");
        ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
            ui.group(|ui| {
                let open_panel = ui
                    .memory()
                    .data
                    .get_temp(note_options_panel)
                    .unwrap_or(NoteOptionsPanelType::Transpose);
                ui.horizontal(|ui| {
                    Self::add_nav_item(
                        open_panel,
                        NoteOptionsPanelType::Transpose,
                        ui,
                        note_options_panel,
                    );
                    Self::add_nav_item(
                        open_panel,
                        NoteOptionsPanelType::Velocity,
                        ui,
                        note_options_panel,
                    );
                    // TODO: Make this dynamic
                    let space_size = ui.available_width();
                    ui.add_space(space_size);

                    ui.with_layout(Layout::right_to_left(), |ui| {
                        ui.add(ResetButton::new(
                            self.setter,
                            Some(vec![&self.params.num_steps]),
                            Some(Box::new(|| {
                                TinyArp::get_velocity_params(self.params)
                                    .into_iter()
                                    .for_each(|p| {
                                        self.setter.set_parameter(p, p.default_plain_value())
                                    });
                                TinyArp::get_transpose_params(self.params)
                                    .into_iter()
                                    .for_each(|p| {
                                        self.setter.set_parameter(p, p.default_plain_value())
                                    });
                                TinyArp::get_enabled_params(self.params)
                                    .into_iter()
                                    .for_each(|p| {
                                        self.setter.set_parameter(p, p.default_plain_value())
                                    });
                                self.setter.set_parameter(
                                    &self.params.num_steps,
                                    self.params.num_steps.default_plain_value(),
                                )
                            })),
                        ));

                        let mut num_steps = self.params.num_steps.value();
                        if ui.add(egui::DragValue::new(&mut num_steps)).changed() {
                            self.setter.set_parameter(&self.params.num_steps, num_steps);
                        };
                    });
                });
                ui.separator();

                ui.add(NoteOptionsSliderPane {
                    setter: self.setter,
                    params: self.params,
                    panel_type: open_panel,
                });
            });
        })
        .response
    }
}
