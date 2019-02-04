pub trait Plugin {
    fn instantiate(
        descriptor: *const lv2::LV2Descriptor,
        rate: f64,
        bundle_path: *const i8,
        features: *const *const lv2::LV2Feature,
    ) -> Self;

    fn connect_port(&mut self, port: u32, data: *mut ());

    fn activate(&mut self) {}

    fn run(&mut self, n_samples: u32);

    fn deactivate(&mut self) {}

    fn extension_data(_uri: *const u8) -> *const libc::c_void  {
        std::ptr::null()
    }
}
