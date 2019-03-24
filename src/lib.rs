extern crate lv2rs as lv2;

use lv2::atom::ports::AtomInputPort;
use lv2::atom::prelude::*;
use lv2::core::{self, ports::*, *};
use lv2::urid::*;
use std::ffi::CStr;

pub struct Midigate {
    control_port: AtomInputPort<Sequence>,
    in_port: AudioInputPort,
    out_port: AudioOutputPort,

    urid_map: CachedMap,

    n_active_notes: u32,
    program: u32,
}

impl Plugin for Midigate {
    fn instantiate(
        _descriptor: &Descriptor,
        _rate: f64,
        _bundle_path: &CStr,
        features: Option<&FeaturesList>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let features = features?;
        let mut cached_map = CachedMap::try_from_features(features)?;

        Some(Self {
            control_port: AtomInputPort::new(&mut cached_map),
            in_port: AudioInputPort::new(),
            out_port: AudioOutputPort::new(),

            urid_map: cached_map,

            n_active_notes: 0,
            program: 0,
        })
    }

    unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.control_port.connect_port(data as *const AtomHeader),
            1 => self.in_port.connect(data as *const f32),
            2 => self.out_port.connect(data as *mut f32),
            _ => (),
        }
    }

    fn run(&mut self, _n_samples: u32) {}
}

lv2_main!(
    core,
    Midigate,
    b"https://github.com/Janonard/eg-midigate-rs\0"
);
