use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use nih_plug::context::ParamSetter;
use nih_plug::param::Param;
use nih_plug::prelude::Enum;
use nih_plug_egui::{egui, widgets};
use nih_plug_egui::egui::{Context, DragValue, popup_below_widget, Vec2};
use nih_plug_egui::egui::style::Margin;

use crate::{Clockwork, FrequencyType, PluginParams, TriggerMode};
use crate::editor::numpad::Numpad;

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

impl GuiEditor for Clockwork {
    const WINDOW_WIDTH: u32 = 600;
    const WINDOW_HEIGHT: u32 = 200;

    fn draw_ui(ctx: &Context, setter: &ParamSetter, params: &Arc<PluginParams>, is_typing: &Arc<AtomicBool>) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let freq_type = params.freq_type.value();
                let freq_param = match freq_type {
                    FrequencyType::Hertz => &params.freq_hz,
                    FrequencyType::Milliseconds => &params.freq_ms,
                    FrequencyType::Bpm => &params.freq_bpm,
                };
                let trigger_mode = params.trigger_mode.value();

                ui.style_mut().spacing.window_margin = Margin::from(Vec2::from([
                    Clockwork::WINDOW_HEIGHT as f32 * 0.05,
                    Clockwork::WINDOW_HEIGHT as f32 * 0.05,
                ]));
                ui.vertical(|ui| {
                    ui.heading("Frequency:");
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(widgets::ParamSlider::for_param(
                                freq_param,
                                setter,
                            )
                                .without_value()
                                .with_width(Clockwork::WINDOW_WIDTH as f32 * 0.8)
                            );

                            let mut draval_val = freq_param.value.clone();
                            let dragval = DragValue::new(&mut draval_val);
                            let dragval_widget = ui.add(dragval);
                            let popup_id = ui.make_persistent_id("numpad");
                            if dragval_widget.clicked() {
                                ui.memory().toggle_popup(popup_id);
                                if is_typing.load(
                                    std::sync::atomic::Ordering::Relaxed
                                ) {
                                    is_typing.store(
                                        false,
                                        std::sync::atomic::Ordering::Relaxed,
                                    );
                                } else {
                                    is_typing.store(
                                        true,
                                        std::sync::atomic::Ordering::Relaxed,
                                    );
                                }
                            }

                            let numpad = Numpad {
                                setter: &setter,
                                param_to_edit: &freq_param,
                                is_typing: &is_typing,
                            };

                            popup_below_widget(
                                ui,
                                popup_id,
                                &dragval_widget,
                                |ui| {
                                    ui.add(numpad);
                                },
                            );

                            if is_typing.load(
                                std::sync::atomic::Ordering::Relaxed
                            ) {
                                ui.memory().open_popup(popup_id);
                                ui.memory().request_focus(popup_id);
                            };

                            // Toggle for frequency type
                            let text = FrequencyType::variants()[freq_type.clone().to_index()];
                            if ui
                                .add(egui::Button::new(text))
                                .clicked()
                            {
                                if freq_type.clone().to_index() == FrequencyType::variants().len() - 1 {
                                    setter.set_parameter(&params.freq_type, FrequencyType::from_index(0));
                                } else {
                                    setter.set_parameter(&params.freq_type, params.freq_type.next_step(freq_type.clone()))
                                }
                            }
                        });
                    });

                    ui.heading("Trigger Mode:");
                    ui.separator();
                    ui.horizontal(
                        |ui| {
                            let text = TriggerMode::variants()[trigger_mode.clone().to_index()];
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
                        }
                    );
                })
            });
    }
}

