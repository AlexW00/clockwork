mod util;

use std::collections::HashMap;
use nih_plug::prelude::*;
use std::sync::{Arc};
use std::time::SystemTime;
use log::{info};

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct ClockworkPlugin {
    params: Arc<ClockworkPluginParams>,
    active_notes: HashMap<u8, NoteEvent>,
    last_midi_send_timestamp: SystemTime,
}

#[derive(Params)]
struct ClockworkPluginParams {
    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[id = "freq-hz"]
    pub freq_hz: FloatParam,
    #[id = "freq-ms"]
    pub freq_ms: FloatParam,
    #[id = "freq-type"]
    pub freq_type: IntParam,
    #[id = "trigger-mode"]
    pub trigger_mode: IntParam,
}

impl Default for ClockworkPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(ClockworkPluginParams::default()),
            active_notes: HashMap::new(),
            last_midi_send_timestamp: SystemTime::UNIX_EPOCH,
        }
    }
}

impl Default for ClockworkPluginParams {
    fn default() -> Self {
        Self {
            // HZ Frequency
            freq_hz: FloatParam::new(
                "Frequency (Hz)",
                0.1,
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
                0.1,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0
                },
            )
                .with_unit(" ms"),

            // Frequency Type
            freq_type: IntParam::new(
                "Frequency Type",
                0,
                IntRange::Linear {
                    min: 0,
                    max: 1
                },
            ).with_value_to_string(freq_type_formatter()),

            // Trigger Mode
            trigger_mode: IntParam::new(
                "Trigger Mode",
                0,
                IntRange::Linear {
                    min: 0,
                    max: 2
                },
            ).with_value_to_string(trigger_mode_formatter()),
        }
    }
}


fn freq_type_formatter () -> Arc<dyn Fn(i32) -> String + Send + Sync> {
    Arc::new(move |value| {
        match value {
            0 => "Hz".to_string(),
            1 => "Ms".to_string(),
            _ => "Unknown".to_string(),
        }
    })
}

fn trigger_mode_formatter () -> Arc<dyn Fn(i32) -> String + Send + Sync> {
    Arc::new(move |value| {
        match value {
            0 => "Continue".to_string(),
            1 => "Re-trigger".to_string(),
            2 => "Re-trigger delayed".to_string(),
            _ => "Unknown".to_string(),
        }
    })
}

impl Plugin for ClockworkPlugin {
    const NAME: &'static str = "ClockworkPlugin";
    const VENDOR: &'static str = "Alexander Weichart";
    const URL: &'static str = "https://github.com/AlexW00/clockwork";
    const EMAIL: &'static str = "alexanderweichart@icloud.com";

    const VERSION: &'static str = "0.0.1";

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
        // TODO: Change version here
        util::logger::init(ClockworkPlugin::NAME.to_string(), 1);
        info!("ClockworkPlugin initialized");
        true
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        //info!("Processing");
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { .. } => self.on_note_on(event),
                NoteEvent::NoteOff { .. } => self.on_note_off(event),
                _ => (),
            }
        }
        //info!("done reading events");
        self.on_midi_send_opportunity(context);
        ProcessStatus::Normal
    }
}

impl ClockworkPlugin {

    fn on_note_on (&mut self, note_event: NoteEvent) {
        match self.params.trigger_mode.value {
            // Re-trigger
            1 => {
                self.last_midi_send_timestamp = SystemTime::UNIX_EPOCH;
            }
            // Re-trigger delayed
            2 => {
                self.last_midi_send_timestamp = SystemTime::now();
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
        for note_event in self.active_notes.values() {
            if let NoteEvent::NoteOn { .. } = note_event {
                context.send_event(note_event.clone());
            }
            if let NoteEvent::NoteOff { note, .. } = note_event {
                context.send_event(note_event.clone());
                note_events_to_remove.push(note.clone());
            }
        }
        for note in note_events_to_remove {
            self.active_notes.remove(&note);
        }
    }

    fn do_send_midi (&self) -> bool {
        match self.params.freq_type.value {
            0 => self.ms_since_last_midi_send() as f32 > self.params.freq_ms.value,
            1 => {
                let ms = 1000.0 / self.params.freq_hz.value;
                self.ms_since_last_midi_send() as f32 > ms
            }
            _ => false,
        }
    }

    fn ms_since_last_midi_send(&self) -> u64 {
        let now = SystemTime::now();
        let dur = now.duration_since(self.last_midi_send_timestamp);
        if let Ok(dur) = dur {
            dur.as_millis() as u64
        } else {
            0
        }
    }
}

// TODO: Configure this
impl ClapPlugin for ClockworkPlugin {
    const CLAP_ID: &'static str = "com.your-domain.clockwork";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("VST plugin, which repeats actively played MIDI notes at variable speed. ");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for ClockworkPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";

    // And don't forget to change these categories, see the docstring on `VST3_CATEGORIES` for more
    // information
    const VST3_CATEGORIES: &'static str = "Instrument|Tools";
}

nih_export_clap!(ClockworkPlugin);
nih_export_vst3!(ClockworkPlugin);
