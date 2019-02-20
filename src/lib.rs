extern crate lv2rs;
use std::ffi::CStr;

use lv2rs::core::{self, *};

struct ExAmp {
    gain: ports::ParameterInputPort,
    input: ports::AudioInputPort,
    output: ports::AudioOutputPort,
}

impl Plugin for ExAmp {
    fn instantiate(
        _descriptor: &raw::Descriptor,
        _rate: f64,
        _bundle_path: &CStr,
        _features: Option<Vec<&mut Feature>>,
    ) -> Self {
        Self {
            gain: ports::ParameterInputPort::new(),
            input: ports::AudioInputPort::new(),
            output: ports::AudioOutputPort::new(),
        }
    }

    unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.gain.connect(data as *const f32),
            1 => self.input.connect(data as *const f32),
            2 => self.output.connect(data as *mut f32),
            _ => (),
        }
    }

    fn run(&mut self, n_samples: u32) {
        let input = unsafe { self.input.as_slice(n_samples) }.unwrap();
        let output = unsafe { self.output.as_slice(n_samples) }.unwrap();
        let gain = unsafe { self.gain.get() }.unwrap();

        for (i_frame, o_frame) in input.iter().zip(output.iter_mut()) {
            *o_frame = *gain * i_frame;
        }
    }
}

lv2_main!(core, ExAmp, b"https://github.com/Janonard/ExAmp\0");
