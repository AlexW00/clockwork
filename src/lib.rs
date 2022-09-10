mod params;
mod editor;

use std::collections::HashMap;
use std::fmt::format;
use nih_plug::prelude::*;
use std::sync::{Arc};
use std::sync::atomic::AtomicBool;
use std::time::SystemTime;
use nih_plug_egui::{create_egui_editor, EguiState};
use crate::params::freq_type::{ FrequencyType };
use crate::params::trigger_mode::{TriggerMode};
use crate::editor::gui::{ GuiEditor };
use crate::editor::note_options::NoteOptionsPanelType;

struct TinyArp {
    params: Arc<PluginParams>,
    active_notes: HashMap<u8, NoteEvent>,
    last_note_on_send: SystemTime,

    is_typing: Arc<AtomicBool>,
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
            last_note_on_send: SystemTime::UNIX_EPOCH,
            is_typing: Arc::new(AtomicBool::new(false)),
        }
    }
}

fn make_enabled_param (id: i8) -> BoolParam {
    BoolParam::new(
        format!("Enable {}", id),
        true
    )
}

fn make_transpose_param (id: i8) -> IntParam {
    IntParam::new(
        format!("Transpose {}", id),
        0,
        IntRange::Linear {
            min: TinyArp::TRANSPOSE_MIN,
            max: TinyArp::TRANSPOSE_MAX
        },
    )
}

fn make_velocity_param (id: i8) -> FloatParam {
    FloatParam::new(
        format!("Velocity {}", id),
        1.0,
        FloatRange::Linear { 
            min: 0.0,
            max: 1.0
        },
    )
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
                    max: 100.0
                },
            ),

            // MS Frequency
            freq_ms: FloatParam::new(
                "Frequency (ms)",
                1000.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10000.0
                },
            ),

            // BPM Frequency
            freq_bpm: FloatParam::new(
                "Frequency (bpm)",
                175.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1000.0
                },
            ),

            // Frequency Type
            freq_type: EnumParam::new(
                "Frequency Type",
                FrequencyType::Hertz,
            ),
            // Trigger Mode
            trigger_mode: EnumParam::new(
                "Trigger Mode",
                TriggerMode::Continue,
            ),

            // Note Options

            num_steps: IntParam::new(
                "Number of steps",
                4,
                IntRange::Linear {
                    min: 1,
                    max: 16,
                },
            ),

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
        let is_typing = self.is_typing.clone();
        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            move |egui_ctx, setter, _state| {
                TinyArp::draw_ui(egui_ctx, setter, &params, &is_typing);
            }
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

    fn on_note_on (&mut self, note_event: NoteEvent) {
        match self.params.trigger_mode.value() {
            TriggerMode::ReTrigger => {
                self.last_note_on_send = SystemTime::UNIX_EPOCH;
            }
            TriggerMode::ReTriggerDelayed=> {
                self.last_note_on_send = SystemTime::now();
            }
            _ => (),
        }
        if let NoteEvent::NoteOn {note, ..} = note_event {
            self.active_notes.insert(note, note_event);
        }
    }

    fn on_note_off (&mut self, note_event: NoteEvent) {
        if let NoteEvent::NoteOff {note, ..} = note_event {
            if self.active_notes.contains_key(&note) {
                self.active_notes.remove(&note);
            }
        }
    }

    fn on_midi_send_opportunity (&mut self, context: &mut impl ProcessContext) {
        if self.do_send_midi() {
            self.send_midi(context);
        }
    }

    fn send_midi (&mut self, context: &mut impl ProcessContext) {
        let mut note_events_to_remove = Vec::<u8>::new();
        let mut did_send_note_on = false;
        for note_event in self.active_notes.values() {
            if let NoteEvent::NoteOn { .. } = note_event {
                context.send_event(note_event.clone());
                did_send_note_on = true;
            }
            if let NoteEvent::NoteOff { note, .. } = note_event {
                context.send_event(note_event.clone());
                note_events_to_remove.push(note.clone());
            }
        }
        for note in note_events_to_remove {
            self.active_notes.remove(&note);
        }

        if did_send_note_on {self.last_note_on_send = SystemTime::now()};
    }

    fn do_send_midi (&self) -> bool {
        match self.params.freq_type.value() {
            FrequencyType::Milliseconds => self.ms_since_last_midi_send() as f32 > self.params.freq_ms.value,
            FrequencyType::Hertz  => {
                let ms = 1000.0 / self.params.freq_hz.value;
                self.ms_since_last_midi_send() as f32 > ms
            }
            FrequencyType::Bpm => {
                let ms = 60000.0 / self.params.freq_bpm.value;
                self.ms_since_last_midi_send() as f32 > ms
            }
        }
    }

    fn ms_since_last_midi_send(&self) -> u64 {
        let now = SystemTime::now();
        let dur = now.duration_since(self.last_note_on_send);
        if let Ok(dur) = dur {
            dur.as_millis() as u64
        } else {
            0
        }
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
