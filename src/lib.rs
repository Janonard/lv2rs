extern crate lv2rs as lv2;

use lv2::atom::ports::AtomInputPort;
use lv2::atom::prelude::*;
use lv2::atom::sequence::TimeStamp;
use lv2::core::{self, ports::*, *};
use lv2::midi::atom::RawMidiMessage;
use lv2::midi::message::StandardMidiMessage;
use lv2::urid::*;
use std::ffi::CStr;

pub struct Midigate {
    control_port: AtomInputPort<Sequence>,
    in_port: AudioInputPort,
    null: Vec<f32>,
    out_port: AudioOutputPort,

    urid_map: CachedMap,

    n_active_notes: i32,
}

impl Midigate {
    fn assure_null_len(&mut self, min_len: usize) {
        if self.null.len() < min_len {
            let n_new_frames = self.null.len() - min_len;
            self.null.reserve(n_new_frames);
            for _ in 0..n_new_frames {
                self.null.push(0.0);
            }
        }
    }
}

impl Plugin for Midigate {
    fn instantiate(
        _descriptor: &Descriptor,
        rate: f64,
        _bundle_path: &CStr,
        features: Option<&FeaturesList>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let features = features?;
        let mut cached_map = CachedMap::try_from_features(features)?;

        let mut plugin = Self {
            control_port: AtomInputPort::new(&mut cached_map),
            in_port: AudioInputPort::new(),
            null: Vec::new(),
            out_port: AudioOutputPort::new(),

            urid_map: cached_map,

            n_active_notes: 0,
        };

        // Allocate null space for one second of frames. This should be enough for most cases.
        plugin.assure_null_len(rate as usize);

        Some(plugin)
    }

    fn activate(&mut self) {
        self.n_active_notes = 0;
    }

    unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.control_port.connect_port(data as *const AtomHeader),
            1 => self.in_port.connect(data as *const f32),
            2 => self.out_port.connect(data as *mut f32),
            _ => (),
        }
    }

    fn run(&mut self, n_samples: u32) {
        // Assure that we have enough null space.
        // Allocation of new space will occur rarely since one second of frames were already
        // allocated at initialization.
        // If we're in a real-time environment, the block sizes won't be longer than a second, and
        // if we're not, than allocation time does not matter.
        self.assure_null_len(n_samples as usize);

        let mut offset: usize = 0;

        let events_atom = unsafe { self.control_port.get_atom(&mut self.urid_map) }.unwrap();
        let audio_input = unsafe { self.in_port.as_slice(n_samples) }.unwrap();
        let null_input = &self.null.as_slice()[0..(n_samples as usize)];
        let audio_output = unsafe { self.out_port.as_slice(n_samples) }.unwrap();

        for (time_stamp, midi_event) in events_atom.iter(&mut self.urid_map).unwrap() {
            // receiving note-ons and note-offs.
            let midi_event = midi_event
                .cast::<RawMidiMessage>(&mut self.urid_map)
                .unwrap();
            let midi_event = midi_event.interpret().unwrap();
            match midi_event.unwrap_standard() {
                StandardMidiMessage::NoteOn {
                    channel: _,
                    note: _,
                    velocity: _,
                } => {
                    self.n_active_notes += 1;
                }
                StandardMidiMessage::NoteOff {
                    channel: _,
                    note: _,
                    velocity: _,
                } => {
                    self.n_active_notes -= 1;
                }
                _ => (),
            }

            let time: usize = match time_stamp {
                TimeStamp::Frames(frames) => frames as usize,
                TimeStamp::Beats(_) => panic!("We can't handle beats!"),
            };

            let input = if self.n_active_notes > 0 {
                &audio_input[offset..time]
            } else {
                &null_input[offset..time]
            };
            audio_output[offset..time].copy_from_slice(input);

            offset += time;
        }

        let time = n_samples as usize;
        let input = if self.n_active_notes > 0 {
            &audio_input[offset..time]
        } else {
            &null_input[offset..time]
        };
        audio_output[offset..time].copy_from_slice(input);
    }
}

lv2_main!(
    core,
    Midigate,
    b"https://github.com/Janonard/eg-midigate-rs\0"
);
