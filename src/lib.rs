extern crate libc;
extern crate lv2_raw as lv2;

pub mod buffer;
mod main_macro;
mod plugin;

use std::collections::VecDeque;
pub use plugin::Plugin;

struct ExAmp {
    delay: buffer::InputParameter,
    input: buffer::InputBuffer,
    output: buffer::OutputBuffer,

    frate: f32,
    backlog: VecDeque<f32>,
}

impl plugin::Plugin for ExAmp {
    fn instantiate(
        _descriptor: *const lv2::LV2Descriptor,
        rate: f64,
        _bundle_path: *const i8,
        _features: *const *const lv2::LV2Feature,
    ) -> Self {
        
        Self {
            delay: buffer::InputParameter::new(),
            input: buffer::InputBuffer::new(),
            output: buffer::OutputBuffer::new(),

            frate: rate as f32,
            backlog: VecDeque::with_capacity(rate as usize),
        }
    }

    fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.delay.connect(data),
            1 => self.input.connect(data),
            2 => self.output.connect(data),
            _ => (),
        }
    }

    fn run(&mut self, n_samples: u32) {
        let input = self.input.iter(n_samples).unwrap();
        let output = self.output.iter_mut(n_samples).unwrap();

        let delay = self.delay.get().unwrap();
        let delay = (delay / 1000.0 * self.frate) as usize;

        while self.backlog.len() > delay {
            self.backlog.pop_back();
        }

        for (input, output) in input.zip(output) {
            if self.backlog.len() >= delay {
                *output = input + self.backlog.pop_back().unwrap();
            } else {
                *output = *input;
            }
            self.backlog.push_front(*input);
        }
    }
}

lv2_main!(ExAmp, "https://github.com/Janonard/ExAmp");
