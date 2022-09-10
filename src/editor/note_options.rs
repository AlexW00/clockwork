use std::fmt::Display;
use std::sync::Arc;
use nih_plug::context::ParamSetter;
use nih_plug_egui::egui;
use nih_plug_egui::egui::{Id, Layout, Response, Ui, Widget};
use crate::{PluginParams};
use nih_plug::prelude::Enum;
use crate::editor::note_options_slider_pane::NoteOptionsSliderPane;


#[derive(PartialEq, Enum, Clone, Copy)]
pub enum NoteOptionsPanelType {
    #[name="Transform"]
    Transform,
    #[name="Velocity"]
    Velocity,
}

impl Display for NoteOptionsPanelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteOptionsPanelType::Transform => write!(f, "Transform"),
            NoteOptionsPanelType::Velocity => write!(f, "Velocity"),
        }
    }
}

pub struct NoteOptions<'a> {
    pub setter: &'a ParamSetter<'a>,
    pub params: &'a Arc<PluginParams>,
}

impl <'a> NoteOptions<'a> {
    fn add_note_options_nav_item(open_panel: NoteOptionsPanelType, panel_type: NoteOptionsPanelType, ui: &mut Ui, id: Id) {
        if ui.add(egui::SelectableLabel::new(open_panel == panel_type, panel_type.to_string())).clicked() {
            ui.memory().data.insert_temp(
                id,
            panel_type
            );
        }
    }
}

impl <'a> Widget for NoteOptions<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let note_options_panel = ui.make_persistent_id("note_options_panel");
        ui.with_layout(
            Layout::top_down(egui::Align::LEFT),
            |ui| {
                let open_panel = ui.memory().data.get_temp(note_options_panel).unwrap_or(NoteOptionsPanelType::Transform);
                ui.horizontal(|ui| {
                    Self::add_note_options_nav_item(open_panel, NoteOptionsPanelType::Transform, ui, note_options_panel);
                    Self::add_note_options_nav_item(open_panel, NoteOptionsPanelType::Velocity, ui, note_options_panel);
                });
                ui.separator();

                match open_panel {
                    NoteOptionsPanelType::Transform => {
                        ui.add(NoteOptionsSliderPane {
                            setter: self.setter,
                            params: self.params,
                            panel_type: NoteOptionsPanelType::Transform,
                        });
                    }
                    NoteOptionsPanelType::Velocity => {
                        ui.add(NoteOptionsSliderPane {
                            setter: self.setter,
                            params: self.params,
                            panel_type: NoteOptionsPanelType::Velocity,
                        });
                    }
                }
            }).response
    }
}