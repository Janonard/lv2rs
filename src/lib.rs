pub mod feature;
pub mod plugin;
pub mod ports;
pub mod raw;
pub mod uris;

pub use plugin::Plugin;

#[macro_export]
macro_rules! lv2_main {
    ($c:ident, $s:ty, $u:expr) => {
        const PLUGIN_URI: &'static [u8] = $u;

        extern "C" fn instantiate(
            descriptor: *const $c::raw::Descriptor,
            rate: f64,
            bundle_path: *const std::os::raw::c_char,
            features: *const *const $c::raw::Feature,
        ) -> $c::raw::Handle {
            $c::plugin::instantiate::<$s>(rate, bundle_path, features)
        }

        extern "C" fn connect_port(
            instance: $c::raw::Handle,
            port: u32,
            data: *mut std::os::raw::c_void,
        ) {
            $c::plugin::connect_port::<$s>(instance, port, data);
        }

        extern "C" fn activate(instance: $c::raw::Handle) {
            $c::plugin::activate::<$s>(instance);
        }

        extern "C" fn run(instance: $c::raw::Handle, n_samples: u32) {
            $c::plugin::run::<$s>(instance, n_samples);
        }

        extern "C" fn deactivate(instance: $c::raw::Handle) {
            $c::plugin::deactivate::<$s>(instance);
        }

        extern "C" fn cleanup(instance: $c::raw::Handle) {
            $c::plugin::cleanup::<$s>(instance);
        }

        extern "C" fn extension_data(
            uri: *const std::os::raw::c_char,
        ) -> *const std::os::raw::c_void {
            $c::plugin::extension_data::<$s>(uri)
        }

        #[no_mangle]
        pub extern "C" fn lv2_descriptor(index: u32) -> *const $c::raw::Descriptor {
            if index == 0 {
                let descriptor = Box::new($c::raw::Descriptor {
                    uri: PLUGIN_URI.as_ptr() as *const std::os::raw::c_char,
                    instantiate: instantiate,
                    connect_port: connect_port,
                    activate: activate,
                    run: run,
                    deactivate: deactivate,
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
