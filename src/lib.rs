pub extern crate lv2_raw;

mod feature;
pub mod ports;
pub mod uris;

pub use lv2_raw::core as raw;
pub use lv2_raw::coreutils as raw_utils;

pub use feature::*;

pub trait Plugin {
    fn instantiate(rate: f64, bundle_path: &std::ffi::CStr, features: FeatureIterator) -> Self;

    fn connect_port(&mut self, port: u32, data: *mut ());

    fn activate(&mut self) {}

    fn run(&mut self, n_samples: u32);

    fn deactivate(&mut self) {}

    fn extension_data(_uri: &std::ffi::CStr) -> *const () {
        std::ptr::null()
    }
}

#[macro_export]
macro_rules! lv2_main {
    ($s:ident, $u:expr) => {
        const PLUGIN_URI: &'static [u8] = $u;

        extern "C" fn instantiate(
            descriptor: *const lv2_core::raw::LV2Descriptor,
            rate: f64,
            bundle_path: *const std::os::raw::c_char,
            features: *const *const lv2_core::Feature,
        ) -> lv2_core::raw::LV2Handle {
            use std::os::raw::c_char;

            let bundle_path = unsafe { std::ffi::CStr::from_ptr(bundle_path as *const c_char) };
            let features = lv2_core::FeatureIterator::new(features);

            let instance = Box::new($s::instantiate(rate, bundle_path, features));

            std::mem::forget(bundle_path);
            Box::leak(instance) as *const $s as lv2_core::raw::LV2Handle
        }

        extern "C" fn connect_port(
            instance: lv2_core::raw::LV2Handle,
            port: u32,
            data: *mut std::os::raw::c_void,
        ) {
            let instance = unsafe { (instance as *mut $s).as_mut() }.unwrap();
            instance.connect_port(port, data as *mut ());
        }

        extern "C" fn activate(instance: lv2_core::raw::LV2Handle) {
            let instance = unsafe { (instance as *mut $s).as_mut() }.unwrap();
            instance.activate();
        }

        extern "C" fn run(instance: lv2_core::raw::LV2Handle, n_samples: u32) {
            let instance = unsafe { (instance as *mut $s).as_mut() }.unwrap();
            instance.run(n_samples);
        }

        extern "C" fn deactivate(instance: lv2_core::raw::LV2Handle) {
            let instance = unsafe { (instance as *mut $s).as_mut() }.unwrap();
            instance.deactivate();
        }

        extern "C" fn cleanup(instance: lv2_core::raw::LV2Handle) {
            unsafe {
                core::ptr::drop_in_place(instance as *mut $s);
            }
        }

        extern "C" fn extension_data(
            uri: *const std::os::raw::c_char,
        ) -> *const std::os::raw::c_void {
            let uri = unsafe { std::ffi::CStr::from_ptr(uri as *const std::os::raw::c_char) };
            let result = $s::extension_data(uri);
            std::mem::forget(uri);
            result as *const std::os::raw::c_void
        }

        #[no_mangle]
        pub extern "C" fn lv2_descriptor(index: u32) -> *const lv2_core::raw::LV2Descriptor {
            if index == 0 {
                let descriptor = Box::new(lv2_core::raw::LV2Descriptor {
                    uri: PLUGIN_URI.as_ptr() as *const std::os::raw::c_char,
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
