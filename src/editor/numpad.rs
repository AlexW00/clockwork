use std::fmt::Display;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use nih_plug::context::ParamSetter;
use nih_plug::param::FloatParam;
use nih_plug_egui::egui;
use nih_plug_egui::egui::{Layout, Response, Ui, Widget};

pub struct Numpad<'a> {
    pub setter: &'a ParamSetter<'a>,
    pub param_to_edit: &'a FloatParam,
    pub is_typing: &'a Arc<AtomicBool>,
}

impl <'a> Numpad<'a> {

    fn on_key(&self, key: NumpadKey) {
        let mut value = self.param_to_edit.value();
        match key {
            NumpadKey::Number(n) => {
                value = value * 10.0 + n as f32;
            }
            NumpadKey::Backspace => {
                value = (value / 10.0).trunc();
            }
            NumpadKey::Enter => {
                self.is_typing.store(false, std::sync::atomic::Ordering::Relaxed);
            }
        }
        self.setter.set_parameter(self.param_to_edit, value);
    }
}

impl <'a> Numpad<'a> {
    const NUM_ROWS: usize = 4;
    const NUM_COLS: usize = 3;
}


impl <'a> Widget for Numpad<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let numpad = ui.with_layout(
            Layout::top_down(egui::Align::Center),
            |ui| {
                for row in 0..Self::NUM_ROWS {
                    ui.horizontal(|ui| {
                        ui.wrap_text();
                        for col in 0..Self::NUM_COLS {
                            let key = NumpadKey::from(row * Self::NUM_COLS + col + 1);
                            let key_widget = ui.add(
                                egui::Button::new(key.to_string())
                            );
                            if key_widget.clicked() {
                                self.on_key(key)
                            }
                        }
                    });
                }
            }
        );
        numpad.response
    }
}

// Key types

enum NumpadKey {
    Number(u8),
    Backspace,
    Enter
}

impl From<usize> for NumpadKey {
    fn from(i: usize) -> Self {
        match i {
            0..=9 => NumpadKey::Number(i as u8),
            10 => NumpadKey::Backspace,
            12 => NumpadKey::Enter,
            _ => NumpadKey::Number(0),
        }
    }
}

impl Display for NumpadKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumpadKey::Number(n) => write!(f, "{}", n),
            NumpadKey::Backspace => write!(f, "<"),
            NumpadKey::Enter => write!(f, "E"),
        }
    }
}