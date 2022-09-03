use std::sync::{Arc};
use std::sync::atomic::AtomicBool;
use nih_plug::context::ParamSetter;
use nih_plug_egui::{egui, widgets};
use nih_plug_egui::egui::{Context, DragValue, popup_below_widget, Response, Rounding, Slider, Ui, Vec2, Widget};
use nih_plug_egui::egui::style::Margin;
use crate::{CategoricalIntParam, Clockwork, FrequencyType, PluginParams, TriggerMode};
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
                let freq_type = FrequencyType::from_int_param(&params.freq_type);
                let freq_param = match freq_type {
                    FrequencyType::Hertz => &params.freq_hz,
                    FrequencyType::Milliseconds => &params.freq_ms,
                    FrequencyType::Bpm => &params.freq_bpm,
                };
                let trigger_mode = TriggerMode::from_int_param(&params.trigger_mode);

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
                                setter
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
                                        std::sync::atomic::Ordering::Relaxed
                                    );
                                } else {
                                    is_typing.store(
                                        true,
                                        std::sync::atomic::Ordering::Relaxed
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
                                }
                            );

                            if is_typing.load(
                                std::sync::atomic::Ordering::Relaxed
                            ) {
                                ui.memory().open_popup(popup_id);
                                ui.memory().request_focus(popup_id);
                            };

                            // Toggle for frequency type

                            if ui
                                .add(egui::Button::new(freq_type.to_string()))
                                .clicked()
                            {
                               match freq_type {
                                      FrequencyType::Hertz => {
                                        setter.set_parameter(&params.freq_type, FrequencyType::Milliseconds as i32);
                                      },
                                      FrequencyType::Milliseconds => {
                                        setter.set_parameter(&params.freq_type, FrequencyType::Bpm as i32);
                                      },
                                        FrequencyType::Bpm => {
                                            setter.set_parameter(&params.freq_type, FrequencyType::Hertz as i32);
                                        }
                               }
                            }
                        });
                    });

                    ui.heading("Trigger Mode:");
                    ui.separator();
                    ui.horizontal(
                        |ui| {
                            if ui
                                .add(egui::Button::new(
                                    trigger_mode.to_string()
                                ))
                                .clicked()
                            {
                                match trigger_mode {
                                    TriggerMode::Continue => {
                                        setter.set_parameter(&params.trigger_mode, TriggerMode::ReTrigger as i32);
                                    },
                                    TriggerMode::ReTrigger => {
                                        setter.set_parameter(&params.trigger_mode, TriggerMode::ReTriggerDelayed as i32);
                                    },
                                    TriggerMode::ReTriggerDelayed => {
                                        setter.set_parameter(&params.trigger_mode, TriggerMode::Continue as i32);
                                    }
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

