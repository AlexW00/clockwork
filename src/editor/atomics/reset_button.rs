use nih_plug::prelude::Param;
use nih_plug_egui::egui::{self, Response, Ui, Widget};

pub struct ResetButton<'a, P: Param> {
    setter: &'a nih_plug::prelude::ParamSetter<'a>,
    params_to_reset: Option<Vec<&'a P>>,
    callback: Option<Box<dyn Fn() + 'a>>,
}

impl<'a, P: Param> ResetButton<'a, P> {
    pub fn new(
        setter: &'a nih_plug::prelude::ParamSetter<'a>,
        params_to_reset: Option<Vec<&'a P>>,
        callback: Option<Box<dyn Fn() + 'a>>,
    ) -> Self {
        Self {
            setter,
            params_to_reset,
            callback,
        }
    }
}

impl<'a, P> Widget for ResetButton<'a, P>
where
    P: Param,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let widget = ui.add(egui::Button::new("X"));
        if widget.clicked() {
            if let Some(params_to_reset) = self.params_to_reset {
                for param in params_to_reset {
                    self.setter
                        .set_parameter(param, param.default_plain_value());
                }
            }
            if let Some(callback) = self.callback {
                callback();
            }
        }
        widget
    }
}
