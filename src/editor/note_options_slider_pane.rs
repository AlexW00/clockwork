use std::sync::Arc;
use nih_plug::context::ParamSetter;
use nih_plug_egui::egui;
use nih_plug_egui::egui::{Id, Layout, Response, Ui, Widget};
use crate::{NoteOptionsPanelType, PluginParams};

pub struct NoteOptionsSliderPane<'a> {
    pub setter: &'a ParamSetter<'a>,
    pub params: &'a Arc<PluginParams>,
    pub panel_type: NoteOptionsPanelType,
}

impl <'a> Widget for NoteOptionsSliderPane<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.with_layout(
            Layout::top_down(egui::Align::LEFT),
            |ui| {

            }
        ).response
    }
}