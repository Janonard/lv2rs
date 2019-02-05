extern crate libc;
extern crate lv2_raw as lv2;

pub mod buffer;
mod main_macro;
mod plugin;

pub use plugin::Plugin;

struct ExAmp {
    gain: buffer::InputParameter,
    input: buffer::InputBuffer,
    output: buffer::OutputBuffer,
}

impl plugin::Plugin for ExAmp {
    fn instantiate(
        _descriptor: *const lv2::LV2Descriptor,
        _rate: f64,
        _bundle_path: *const i8,
        _features: *const *const lv2::LV2Feature,
    ) -> Self {
        Self {
            gain: buffer::InputParameter::new(),
            input: buffer::InputBuffer::new(),
            output: buffer::OutputBuffer::new(),
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
