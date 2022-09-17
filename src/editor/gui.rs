use std::sync::Arc;

use nih_plug::context::ParamSetter;
use nih_plug_egui::egui;
use nih_plug_egui::egui::style::Margin;
use nih_plug_egui::egui::{Context, Vec2};

use crate::editor::note_options::NoteOptions;
use crate::{PluginParams, TinyArp};

use super::frequency_panel::FrequencyPanel;
use super::trigger_mode_panel::TriggerModePanel;

pub trait GuiEditor {
    const WINDOW_WIDTH: u32;
    const WINDOW_HEIGHT: u32;

    fn draw_ui(ctx: &Context, setter: &ParamSetter, params: &Arc<PluginParams>);
}

impl GuiEditor for TinyArp {
    const WINDOW_WIDTH: u32 = 600;
    const WINDOW_HEIGHT: u32 = 700;

    fn draw_ui(ctx: &Context, setter: &ParamSetter, params: &Arc<PluginParams>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.window_margin = Margin::from(Vec2::from([
                TinyArp::WINDOW_HEIGHT as f32 * 0.05,
                TinyArp::WINDOW_HEIGHT as f32 * 0.05,
            ]));

            ui.vertical(|ui| {
                egui::CollapsingHeader::new("Frequency")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add(FrequencyPanel { setter, params });
                    });

                egui::CollapsingHeader::new("Trigger Mode")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add(TriggerModePanel { setter, params });
                    });

                egui::CollapsingHeader::new("Notes")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add(NoteOptions { setter, params });
                    });
            })
        });
    }
}
