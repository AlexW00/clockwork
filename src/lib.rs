mod params;
mod editor;

use std::collections::HashMap;
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
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2)),

            // MS Frequency
            freq_ms: FloatParam::new(
                "Frequency (ms)",
                1000.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10000.0
                },
            )
                .with_unit(" ms"),

            // BPM Frequency
            freq_bpm: FloatParam::new(
                "Frequency (bpm)",
                175.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1000.0
                },
            )
                .with_unit(" bpm"),

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
