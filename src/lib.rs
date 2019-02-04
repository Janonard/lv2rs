extern crate libc;
extern crate lv2_raw as lv2;

mod main_macro;

struct ExAmp {
    gain: *const f32,
    input: *const f32,
    output: *mut f32,
}

impl ExAmp {
    fn new(
        _descriptor: *const lv2::LV2Descriptor,
        _rate: f64,
        _bundle_path: *const i8,
        _features: *const *const lv2::LV2Feature,
    ) -> Self {
        Self {
            gain: std::ptr::null(),
            input: std::ptr::null(),
            output: std::ptr::null_mut(),
        }
    }

    fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.gain = data as *const f32,
            1 => self.input = data as *const f32,
            2 => self.output = data as *mut f32,
            _ => (),
        }
    }

    fn run(&mut self, n_samples: u32) {
        for i in 0..n_samples {
            unsafe {
                *(self.output.add(i as usize)) = *(self.input.add(i as usize)) * *(self.gain);
            }
        }
    }

    fn extension_data(_uri: *const u8) -> *const libc::c_void {
        std::ptr::null()
    }
}

lv2_main!(ExAmp, "https://github.com/Janonard/ExAmp");