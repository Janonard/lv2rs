extern crate lv2_core;
use std::ffi::CStr;

use lv2_core::*;

struct ExAmp {
    gain: ports::ParameterInputPort,
    input: ports::AudioInputPort,
    output: ports::AudioOutputPort,
}

impl lv2_core::Plugin for ExAmp {
    fn instantiate(
        _rate: f64,
        _bundle_path: &CStr,
        _features: *const *const raw::LV2Feature,
    ) -> Self {
        Self {
            gain: ports::ParameterInputPort::new(),
            input: ports::AudioInputPort::new(),
            output: ports::AudioOutputPort::new(),
        }
    }

    fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.gain.connect(data),
            1 => self.input.connect(data),
            2 => self.output.connect(data),
            _ => (),
        }
    }

    fn run(&mut self, n_samples: u32) {
        let input = self.input.iter(n_samples).unwrap();
        let output = self.output.iter_mut(n_samples).unwrap();
        let gain = self.gain.get().unwrap();

        for (input, output) in input.zip(output) {
            *output = input * gain;
        }
    }
}

lv2_main!(ExAmp, b"https://github.com/Janonard/ExAmp");
