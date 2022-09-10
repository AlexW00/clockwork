use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use nih_plug::context::ParamSetter;
use nih_plug_egui::{egui};
use nih_plug_egui::egui::{Context, Vec2};
use nih_plug_egui::egui::style::Margin;

use crate::{TinyArp, PluginParams};
use crate::editor::frequency_category::frequency_category;
use crate::editor::note_options::{NoteOptions, NoteOptionsPanelType};
use crate::editor::trigger_mode_category::trigger_mode_category;

pub trait GuiEditor {
    const WINDOW_WIDTH: u32;
    const WINDOW_HEIGHT: u32;

    fn draw_ui(
        ctx: &Context,
        setter: &ParamSetter,
        params: &Arc<PluginParams>,
        is_typing: &Arc<AtomicBool>,
    );
}

impl GuiEditor for TinyArp {
    const WINDOW_WIDTH: u32 = 600;
    const WINDOW_HEIGHT: u32 = 700;

    fn draw_ui(ctx: &Context, setter: &ParamSetter, params: &Arc<PluginParams>, is_typing: &Arc<AtomicBool>) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {

                ui.style_mut().spacing.window_margin = Margin::from(Vec2::from([
                    TinyArp::WINDOW_HEIGHT as f32 * 0.05,
                    TinyArp::WINDOW_HEIGHT as f32 * 0.05,
                ]));

                ui.vertical(|ui| {
                    egui::CollapsingHeader::new("Frequency")
                        .default_open(true)
                        .show(ui, |ui| {
                            frequency_category(ui, setter, params, is_typing);
                        });

                    egui::CollapsingHeader::new("Notes")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.add(NoteOptions {
                                setter,
                                params,
                            });
                        });

                    egui::CollapsingHeader::new("Trigger Mode")
                        .default_open(true)
                        .show(ui, |ui| {
                            trigger_mode_category(ui, setter, params);
                        });

                })
            });
    }
}

