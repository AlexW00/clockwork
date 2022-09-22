mod editor;
mod params;

use crate::editor::gui::GuiEditor;
use crate::editor::note_options::NoteOptionsPanelType;
use crate::params::freq_type::FrequencyType;
use crate::params::trigger_mode::TriggerMode;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, EguiState};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

struct TinyArp {
    params: Arc<PluginParams>,
    active_notes: HashMap<u8, NoteEvent>,
    last_note_timestamp: SystemTime,
    next_step_index: usize,
}

#[derive(Params)]
pub struct PluginParams {
    #[id = "freq-hz"]
    pub freq_hz: FloatParam,
    #[id = "freq-ms"]
    pub freq_ms: FloatParam,
    #[id = "freq-bpm"]
    pub freq_bpm: FloatParam,
    #[id = "freq-type"]
    pub freq_type: EnumParam<FrequencyType>,
    #[id = "trigger-mode"]
    pub trigger_mode: EnumParam<TriggerMode>,

    // Note options (1-16 each)
    #[id = "number-enabled-notes"]
    pub num_steps: IntParam,

    // Enable
    #[id = "note-enable-1"]
    pub enabled_1: BoolParam,
    #[id = "note-enable-2"]
    pub enabled_2: BoolParam,
    #[id = "note-enable-3"]
    pub enabled_3: BoolParam,
    #[id = "note-enable-4"]
    pub enabled_4: BoolParam,
    #[id = "note-enable-5"]
    pub enabled_5: BoolParam,
    #[id = "note-enable-6"]
    pub enabled_6: BoolParam,
    #[id = "note-enable-7"]
    pub enabled_7: BoolParam,
    #[id = "note-enable-8"]
    pub enabled_8: BoolParam,
    #[id = "note-enable-9"]
    pub enabled_9: BoolParam,
    #[id = "note-enable-10"]
    pub enabled_10: BoolParam,
    #[id = "note-enable-11"]
    pub enabled_11: BoolParam,
    #[id = "note-enable-12"]
    pub enabled_12: BoolParam,
    #[id = "note-enable-13"]
    pub enabled_13: BoolParam,
    #[id = "note-enable-14"]
    pub enabled_14: BoolParam,
    #[id = "note-enable-15"]
    pub enabled_15: BoolParam,
    #[id = "note-enable-16"]
    pub enabled_16: BoolParam,

    // Transpose
    #[id = "transpose-1"]
    pub transpose_1: IntParam,
    #[id = "transpose-2"]
    pub transpose_2: IntParam,
    #[id = "transpose-3"]
    pub transpose_3: IntParam,
    #[id = "transpose-4"]
    pub transpose_4: IntParam,
    #[id = "transpose-5"]
    pub transpose_5: IntParam,
    #[id = "transpose-6"]
    pub transpose_6: IntParam,
    #[id = "transpose-7"]
    pub transpose_7: IntParam,
    #[id = "transpose-8"]
    pub transpose_8: IntParam,
    #[id = "transpose-9"]
    pub transpose_9: IntParam,
    #[id = "transpose-10"]
    pub transpose_10: IntParam,
    #[id = "transpose-11"]
    pub transpose_11: IntParam,
    #[id = "transpose-12"]
    pub transpose_12: IntParam,
    #[id = "transpose-13"]
    pub transpose_13: IntParam,
    #[id = "transpose-14"]
    pub transpose_14: IntParam,
    #[id = "transpose-15"]
    pub transpose_15: IntParam,
    #[id = "transpose-16"]
    pub transpose_16: IntParam,

    // Velocity
    #[id = "velocity-1"]
    pub velocity_1: FloatParam,
    #[id = "velocity-2"]
    pub velocity_2: FloatParam,
    #[id = "velocity-3"]
    pub velocity_3: FloatParam,
    #[id = "velocity-4"]
    pub velocity_4: FloatParam,
    #[id = "velocity-5"]
    pub velocity_5: FloatParam,
    #[id = "velocity-6"]
    pub velocity_6: FloatParam,
    #[id = "velocity-7"]
    pub velocity_7: FloatParam,
    #[id = "velocity-8"]
    pub velocity_8: FloatParam,
    #[id = "velocity-9"]
    pub velocity_9: FloatParam,
    #[id = "velocity-10"]
    pub velocity_10: FloatParam,
    #[id = "velocity-11"]
    pub velocity_11: FloatParam,
    #[id = "velocity-12"]
    pub velocity_12: FloatParam,
    #[id = "velocity-13"]
    pub velocity_13: FloatParam,
    #[id = "velocity-14"]
    pub velocity_14: FloatParam,
    #[id = "velocity-15"]
    pub velocity_15: FloatParam,
    #[id = "velocity-16"]
    pub velocity_16: FloatParam,

    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
}

impl Default for TinyArp {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
            active_notes: HashMap::new(),
            last_note_timestamp: SystemTime::UNIX_EPOCH,
            next_step_index: 0,
        }
    }
}

fn make_enabled_param(id: i8) -> BoolParam {
    BoolParam::new(format!("Enable {}", id), true)
}

fn make_transpose_param(id: i8) -> IntParam {
    IntParam::new(
        format!("Transpose {}", id),
        0,
        IntRange::Linear {
            min: TinyArp::TRANSPOSE_MIN,
            max: TinyArp::TRANSPOSE_MAX,
        },
    )
}

fn make_velocity_param(id: i8) -> FloatParam {
    FloatParam::new(
        format!("Velocity {}", id),
        1.0,
        FloatRange::Linear { min: 0.0, max: 1.0 },
    )
    .with_step_size(0.01)
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(TinyArp::WINDOW_WIDTH, TinyArp::WINDOW_HEIGHT),

            // HZ Frequency
            freq_hz: FloatParam::new(
                "Frequency (Hz)",
                1.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_step_size(0.1),

            // MS Frequency
            freq_ms: FloatParam::new(
                "Frequency (ms)",
                1000.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10000.0,
                },
            )
            .with_step_size(1.0),

            // BPM Frequency
            freq_bpm: FloatParam::new(
                "Frequency (bpm)",
                175.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1000.0,
                },
            )
            .with_step_size(1.0),

            // Frequency Type
            freq_type: EnumParam::new("Frequency Type", FrequencyType::Hertz),
            // Trigger Mode
            trigger_mode: EnumParam::new("Trigger Mode", TriggerMode::Continue),

            // Note Options
            num_steps: IntParam::new("Number of steps", 4, IntRange::Linear { min: 1, max: 16 }),

            // Enable 1-16
            enabled_1: make_enabled_param(1),
            enabled_2: make_enabled_param(2),
            enabled_3: make_enabled_param(3),
            enabled_4: make_enabled_param(4),
            enabled_5: make_enabled_param(5),
            enabled_6: make_enabled_param(6),
            enabled_7: make_enabled_param(7),
            enabled_8: make_enabled_param(8),
            enabled_9: make_enabled_param(9),
            enabled_10: make_enabled_param(10),
            enabled_11: make_enabled_param(11),
            enabled_12: make_enabled_param(12),
            enabled_13: make_enabled_param(13),
            enabled_14: make_enabled_param(14),
            enabled_15: make_enabled_param(15),
            enabled_16: make_enabled_param(16),

            // Transpose 1-16
            transpose_1: make_transpose_param(1),
            transpose_2: make_transpose_param(2),
            transpose_3: make_transpose_param(3),
            transpose_4: make_transpose_param(4),
            transpose_5: make_transpose_param(5),
            transpose_6: make_transpose_param(6),
            transpose_7: make_transpose_param(7),
            transpose_8: make_transpose_param(8),
            transpose_9: make_transpose_param(9),
            transpose_10: make_transpose_param(10),
            transpose_11: make_transpose_param(11),
            transpose_12: make_transpose_param(12),
            transpose_13: make_transpose_param(13),
            transpose_14: make_transpose_param(14),
            transpose_15: make_transpose_param(15),
            transpose_16: make_transpose_param(16),

            // Velocity 1-16
            velocity_1: make_velocity_param(1),
            velocity_2: make_velocity_param(2),
            velocity_3: make_velocity_param(3),
            velocity_4: make_velocity_param(4),
            velocity_5: make_velocity_param(5),
            velocity_6: make_velocity_param(6),
            velocity_7: make_velocity_param(7),
            velocity_8: make_velocity_param(8),
            velocity_9: make_velocity_param(9),
            velocity_10: make_velocity_param(10),
            velocity_11: make_velocity_param(11),
            velocity_12: make_velocity_param(12),
            velocity_13: make_velocity_param(13),
            velocity_14: make_velocity_param(14),
            velocity_15: make_velocity_param(15),
            velocity_16: make_velocity_param(16),
        }
    }
}

impl Plugin for TinyArp {
    const NAME: &'static str = "TinyArp";
    const VENDOR: &'static str = "Alexander Weichart";
    const URL: &'static str = "https://github.com/AlexW00/tinyarp";
    const EMAIL: &'static str = "alexanderweichart@icloud.com";

    const VERSION: &'static str = "1.1.0";

    const DEFAULT_INPUT_CHANNELS: u32 = 0;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 0;

    const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
    const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();

        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            move |egui_ctx, setter, _state| {
                TinyArp::draw_ui(egui_ctx, setter, &params);
            },
        )
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        // This works with any symmetrical IO layout
        config.num_input_channels == config.num_output_channels && config.num_input_channels > 0
    }

    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext,
    ) -> bool {
        nih_log!("TinyArp initialized");
        true
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { .. } => self.on_note_on(event),
                NoteEvent::NoteOff { .. } => self.on_note_off(event),
                _ => (),
            }
        }
        self.on_midi_send_opportunity(context);
        ProcessStatus::Normal
    }
}

impl TinyArp {
    const TRANSPOSE_MIN: i32 = -24;
    const TRANSPOSE_MAX: i32 = 24;

    fn on_note_on(&mut self, note_event: NoteEvent) {
        match self.params.trigger_mode.value() {
            TriggerMode::ReTrigger => {
                self.last_note_timestamp = SystemTime::UNIX_EPOCH;
            }
            TriggerMode::ReTriggerDelayed => {
                self.last_note_timestamp = SystemTime::now();
            }
            _ => (),
        }
        if let NoteEvent::NoteOn { note, .. } = note_event {
            self.active_notes.insert(note, note_event);
        }
    }

    fn on_note_off(&mut self, note_event: NoteEvent) {
        if let NoteEvent::NoteOff { note, .. } = note_event {
            if self.active_notes.contains_key(&note) {
                self.active_notes.remove(&note);
            }
        }
    }

    fn on_midi_send_opportunity(&mut self, context: &mut impl ProcessContext) {
        if self.do_send_midi() {
            self.send_midi(context);
        }
    }

    fn is_note_enabled(&self, index: usize) -> bool {
        let params = &self.params;
        let enabled_params = TinyArp::get_enabled_params(params);
        let _is_enabled_param = enabled_params.get(index);
        if let Some(is_enabled_param) = _is_enabled_param {
            return is_enabled_param.value();
        }
        false
    }

    fn get_index(&mut self, recursion_index: usize) -> Option<usize> {
        let max_steps = self.params.num_steps.value() as usize;
        if max_steps - 1 <= recursion_index {
            return None;
        }
        let current_step_index = self.next_step();

        if !self.is_note_enabled(current_step_index) {
            return self.get_index(recursion_index + 1);
        }

        Some(current_step_index)
    }

    fn next_step(&mut self) -> usize {
        let max_steps = self.params.num_steps.value() as usize;
        let current_step_index = self.next_step_index;
        if self.next_step_index >= max_steps - 1 {
            self.next_step_index = 0;
        } else {
            self.next_step_index += 1;
        }
        current_step_index
    }

    fn get_next_note_event(&mut self, note_event: &NoteEvent) -> Option<NoteEvent> {
        let _step_index = self.get_index(0);
        if let Some(step_index) = _step_index {
            if let Some(mut midi_data) = note_event.as_midi() {
                let timing = note_event.timing();

                self.apply_transpose_modulation(&mut midi_data, step_index);
                self.apply_velocity_modulation(&mut midi_data, step_index);

                return NoteEvent::from_midi(timing, midi_data).ok();
            }
        } else {
            return None;
        }
        Some(note_event.clone())
    }

    fn apply_transpose_modulation(&mut self, midi_data: &mut [u8], index: usize) {
        if let Some(transpose_param) = self.get_transpose_param_by_index(index) {
            let new_note = midi_data[1] as i32 + transpose_param.value();
            midi_data[1] = new_note as u8;
        }
    }

    fn get_transpose_param_by_index(&self, index: usize) -> Option<&IntParam> {
        let params = &self.params;
        let transpose_params = TinyArp::get_transpose_params(params);
        match transpose_params.get(index) {
            Some(transpose_param) => Some(transpose_param),
            None => None,
        }
    }

    fn apply_velocity_modulation(&self, midi_data: &mut [u8; 3], step_index: usize) {
        if let Some(velocity) = self.get_velocity_param_by_index(step_index) {
            let new_velocity = midi_data[2] as f32 + velocity.value();
            midi_data[2] = new_velocity as u8;
        }
    }

    fn get_velocity_param_by_index(&self, index: usize) -> Option<&FloatParam> {
        let params = &self.params;
        let velocity_params = TinyArp::get_velocity_params(params);
        match velocity_params.get(index) {
            Some(velocity_param) => Some(velocity_param),
            None => None,
        }
    }

    fn send_midi(&mut self, context: &mut impl ProcessContext) {
        let mut note_events_to_remove = Vec::<u8>::new();
        let mut did_send_note_on = false;

        for note_event in self.active_notes.clone().values() {
            if let NoteEvent::NoteOn { .. } = note_event {
                let _next_note_event = self.get_next_note_event(note_event);
                if let Some(next_note_event) = _next_note_event {
                    context.send_event(next_note_event);
                    did_send_note_on = true;
                }
            }
            if let NoteEvent::NoteOff { note, .. } = note_event {
                context.send_event(note_event.clone());
                note_events_to_remove.push(note.clone());
            }
        }
        for note in note_events_to_remove {
            self.active_notes.remove(&note);
        }

        if did_send_note_on {
            self.last_note_timestamp = SystemTime::now()
        };
    }

    fn do_send_midi(&self) -> bool {
        match self.params.freq_type.value() {
            FrequencyType::Milliseconds => {
                self.ms_since_last_midi_send() as f32 > self.params.freq_ms.value()
            }
            FrequencyType::Hertz => {
                let ms = 1000.0 / self.params.freq_hz.value();
                self.ms_since_last_midi_send() as f32 > ms
            }
            FrequencyType::Bpm => {
                let ms = 60000.0 / self.params.freq_bpm.value();
                self.ms_since_last_midi_send() as f32 > ms
            }
        }
    }

    fn ms_since_last_midi_send(&self) -> u64 {
        let now = SystemTime::now();
        let dur = now.duration_since(self.last_note_timestamp);
        if let Ok(dur) = dur {
            dur.as_millis() as u64
        } else {
            0
        }
    }

    fn get_transpose_params<'a>(params: &'a Arc<PluginParams>) -> Vec<&IntParam> {
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

    fn get_velocity_params<'a>(params: &'a Arc<PluginParams>) -> Vec<&FloatParam> {
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

    fn get_enabled_params<'a>(params: &'a Arc<PluginParams>) -> Vec<&BoolParam> {
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
}

impl ClapPlugin for TinyArp {
    const CLAP_ID: &'static str = "com.alexanderweichart.tinyarp";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Simple MIDI note repeater.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for TinyArp {
    const VST3_CLASS_ID: [u8; 16] = *b"Cl0ckw0rkPlug1nX";

    // And don't forget to change these categories, see the docstring on `VST3_CATEGORIES` for more
    // information
    const VST3_CATEGORIES: &'static str = "Instrument|Tools";
}

nih_export_clap!(TinyArp);
nih_export_vst3!(TinyArp);
