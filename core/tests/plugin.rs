extern crate lv2rs_core as core;

use std::ffi::CStr;

use core::ports::*;
use std::os::raw::*;
use std::ptr::{null, null_mut};

/// A simple test plugin.
///
/// It takes an audio input, multiplies it by the input parameter and writes the result in the
/// audio output. Additionally, it calculates the RMS of the applified signal and writes it into
/// the output parameter.
struct TestPlugin {
    audio_in: AudioInputPort,
    audio_out: AudioOutputPort,
    parameter_in: ParameterInputPort,
    parameter_out: ParameterOutputPort,
}

impl core::Plugin for TestPlugin {
    fn instantiate(
        _descriptor: &core::Descriptor,
        _rate: f64,
        _bundle_path: &CStr,
        _features: Option<&core::FeaturesList>,
    ) -> Option<Self> {
        Some(Self {
            audio_in: AudioInputPort::new(),
            audio_out: AudioOutputPort::new(),
            parameter_in: ParameterInputPort::new(),
            parameter_out: ParameterOutputPort::new(),
        })
    }

    unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.audio_in.connect(data as *const f32),
            1 => self.audio_out.connect(data as *mut f32),
            2 => self.parameter_in.connect(data as *const f32),
            3 => self.parameter_out.connect(data as *mut f32),
            _ => (),
        }
    }

    fn run(&mut self, n_samples: u32) {
        let audio_in = unsafe { self.audio_in.as_slice(n_samples) }.unwrap();
        let audio_out = unsafe { self.audio_out.as_slice(n_samples) }.unwrap();
        let parameter_in = unsafe { self.parameter_in.get() }.unwrap();
        let parameter_out = unsafe { self.parameter_out.get_mut() }.unwrap();

        for (sample_in, sample_out) in audio_in.iter().zip(audio_out.iter_mut()) {
            *sample_out = *parameter_in * sample_in;
        }

        let sum: f32 = audio_out.iter().map(|sample| sample.powi(2)).sum();
        let sum = (sum / n_samples as f32).sqrt();
        *parameter_out = sum;
    }
}

core::lv2_main!(core, TestPlugin, b"http://example.org/TestPlugin\0");

struct TestHost {
    handle: core::Handle,
    audio_input: [f32; 256],
    audio_output: [f32; 256],
    parameter_input: f32,
    parameter_output: f32,
}

const BUNDLE_PATH: &[u8] = b"/\0";
impl TestHost {
    fn new() -> Self {
        Self {
            handle: std::ptr::null_mut(),
            audio_input: [0.0; 256],
            audio_output: [0.0; 256],
            parameter_input: 0.0,
            parameter_output: 0.0,
        }
    }
}

#[test]
fn test_plugin() {
    let mut host = TestHost::new();

    let descriptor = unsafe { lv2_descriptor(0) };
    let descriptor_ref = unsafe { descriptor.as_ref() }.unwrap();

    // test invalid parameters.
    unsafe {
        assert_eq!(
            (descriptor_ref.instantiate)(null(), 0.0, null(), null()),
            null_mut()
        );
        assert_eq!(
            (descriptor_ref.instantiate)(descriptor, 0.0, null(), null()),
            null_mut()
        );
        assert_eq!(
            (descriptor_ref.instantiate)(
                null(),
                0.0,
                BUNDLE_PATH.as_ptr() as *const c_char,
                null()
            ),
            null_mut()
        );
    }

    // get the proper handle.
    host.handle = unsafe {
        (descriptor_ref.instantiate)(
            descriptor,
            44100.0,
            BUNDLE_PATH.as_ptr() as *const c_char,
            null(),
        )
    };
    assert_ne!(host.handle, null_mut());

    // connect_port.
    unsafe {
        (descriptor_ref.connect_port)(host.handle, 0, host.audio_input.as_ptr() as *mut c_void);
        (descriptor_ref.connect_port)(host.handle, 1, host.audio_output.as_ptr() as *mut c_void);
        (descriptor_ref.connect_port)(
            host.handle,
            2,
            &host.parameter_input as *const f32 as *mut c_void,
        );
        (descriptor_ref.connect_port)(
            host.handle,
            3,
            &host.parameter_output as *const f32 as *mut c_void,
        );
    }

    // setting input.
    for (index, frame) in host.audio_input.iter_mut().enumerate() {
        *frame = index as f32
    }
    host.parameter_input = 2.0;

    // run.
    unsafe { (descriptor_ref.run)(host.handle, 256) };

    // verifying output.
    for (index, frame) in host.audio_output.iter().enumerate() {
        assert_eq!(*frame, index as f32 * 2.0);
    }
    let rms: f32 = host.audio_output.iter().map(|x| x.powi(2)).sum();
    let rms: f32 = (rms / host.audio_output.len() as f32).sqrt();
    assert_eq!(host.parameter_output, rms);

    // cleanup.
    unsafe { (descriptor_ref.cleanup)(host.handle) };
}
