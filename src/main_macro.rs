#[macro_export]
macro_rules! lv2_main {
    ($s:ident, $u:expr) => {
        extern "C" fn instantiate(
            descriptor: *const lv2::LV2Descriptor,
            rate: f64,
            bundle_path: *const i8,
            features: *const *const lv2::LV2Feature,
        ) -> lv2::LV2Handle {
            let instance = Box::new($s::instantiate(descriptor, rate, bundle_path, features));
            Box::leak(instance) as *const $s as lv2::LV2Handle
        }

        extern "C" fn connect_port(instance: lv2::LV2Handle, port: u32, data: *mut libc::c_void) {
            let instance = unsafe { (instance as *mut $s).as_mut() }.unwrap();
            instance.connect_port(port, data as *mut ());
        }

        extern "C" fn activate(instance: lv2::LV2Handle) {
            let instance = unsafe { (instance as *mut $s).as_mut() }.unwrap();
            instance.activate();
        }

        extern "C" fn run(instance: lv2::LV2Handle, n_sinstanceles: u32) {
            let instance = unsafe { (instance as *mut $s).as_mut() }.unwrap();
            instance.run(n_sinstanceles);
        }

        extern "C" fn deactivate(instance: lv2::LV2Handle) {
            let instance = unsafe { (instance as *mut $s).as_mut() }.unwrap();
            instance.deactivate();
        }

        extern "C" fn cleanup(instance: lv2::LV2Handle) {
            unsafe {
                core::ptr::drop_in_place(instance as *mut $s);
            }
        }

        extern "C" fn extension_data(uri: *const u8) -> *const libc::c_void {
            $s::extension_data(uri)
        }

        #[no_mangle]
        pub extern "C" fn lv2_descriptor(index: u32) -> *const lv2::LV2Descriptor {
            if index == 0 {
                let descriptor = Box::new(lv2::LV2Descriptor {
                    uri: $u.as_ptr() as *const i8,
                    instantiate: instantiate,
                    connect_port: connect_port,
                    activate: Some(activate),
                    run: run,
                    deactivate: Some(deactivate),
                    cleanup: cleanup,
                    extension_data: extension_data,
                });
                Box::leak(descriptor)
            } else {
                std::ptr::null()
            }
        }
    };
}
