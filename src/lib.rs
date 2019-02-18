//! A rust re-implementation of the LV2 core library.
//!
//! This crate contains the complete contents of the
//! [LV2 core library](http://lv2plug.in/ns/lv2core/lv2core.html) with additional constructions
//! to make the use of LV2 as idiomatic and safe as possible.
//!
//! ## Getting started
//!
//! From Rust's side of view, creating an LV2 plugin is fairly simple:
//! * Create a new library crate
//! * Add the following entry to your `Cargo.toml` in order to build a dynamic library:
//!
//!   [lib]
//!   
//!   crate-type = ["dylib"]
//!
//! * Add lv2rs-core as a dependency both in your `Cargo.toml` as well as in your `lib.rs`
//! * Create a struct and implement [`Plugin`](trait.Plugin.html) for it.
//! * Use the [`lv2_main!`](macro.lv2_main.html) macro to export the required symbols.
//!
//! That's it! Although, if you really want to get started, you should check out the
//! [tutorial](http://lv2plug.in/book/) at LV2's website before.
mod plugin;
pub mod ports;
pub mod raw;
pub mod uris;

pub use plugin::*;
pub use raw::{Descriptor, Feature};

/// Create  lv2 export functions.
///
/// This macro takes a struct that implements [`Plugin`](trait.Plugin.html) and crates the required
/// functions a plugin needs to export in order to be found and used by plugin hosts.
///
/// In order to properly work, it needs three arguments:
/// * The namespace of the `lv2rs-core` crate: You may use this crate via re-exports and
/// therefore, the name of the namespace is needed in order to call the appropiate functions.
/// * The struct type that should be used as the Plugin implementation.
/// * The URI of the plugin. Please note that the URI needs to be a bytes-array and null-terminated,
/// since the C world has to interact with it.
///
/// Example usage:
///     
///     extern crate lv2rs_core as core;
///     use std::ffi::CStr;
///
///     struct MyPlugin {}
///
///     impl core::Plugin for MyPlugin {
///         fn instantiate(
///             _descriptor: &core::Descriptor,
///             _rate: f64,
///             _bundle_path: &CStr,
///             _features: Vec<&mut core::Feature>
///         ) -> Self {
///             Self {}
///         }
///
///         unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {}
///
///         fn run(&mut self, _n_samples: u32) {}
///     }
///
///     core::lv2_main!(core, MyPlugin, b"http://example.org/Dummy\0");
///
#[macro_export]
macro_rules! lv2_main {
    ($c:ident, $s:ty, $u:expr) => {
        const PLUGIN_URI: &'static [u8] = $u;
        const PLUGIN_DESCRIPTOR: $c::raw::Descriptor = $c::raw::Descriptor {
            uri: PLUGIN_URI.as_ptr() as *const std::os::raw::c_char,
            instantiate: instantiate,
            connect_port: connect_port,
            activate: activate,
            run: run,
            deactivate: deactivate,
            cleanup: cleanup,
            extension_data: extension_data,
        };

        unsafe extern "C" fn instantiate(
            descriptor: *const $c::raw::Descriptor,
            rate: f64,
            bundle_path: *const std::os::raw::c_char,
            features: *const *const $c::raw::Feature,
        ) -> $c::raw::Handle {
            $c::instantiate::<$s>(descriptor, rate, bundle_path, features)
        }

        unsafe extern "C" fn connect_port(
            instance: $c::raw::Handle,
            port: u32,
            data: *mut std::os::raw::c_void,
        ) {
            $c::connect_port::<$s>(instance, port, data);
        }

        unsafe extern "C" fn activate(instance: $c::raw::Handle) {
            $c::activate::<$s>(instance);
        }

        unsafe extern "C" fn run(instance: $c::raw::Handle, n_samples: u32) {
            $c::run::<$s>(instance, n_samples);
        }

        unsafe extern "C" fn deactivate(instance: $c::raw::Handle) {
            $c::deactivate::<$s>(instance);
        }

        unsafe extern "C" fn cleanup(instance: $c::raw::Handle) {
            $c::cleanup::<$s>(instance);
        }

        unsafe extern "C" fn extension_data(
            uri: *const std::os::raw::c_char,
        ) -> *const std::os::raw::c_void {
            $c::extension_data::<$s>(uri)
        }

        #[no_mangle]
        pub unsafe extern "C" fn lv2_descriptor(index: u32) -> *const $c::raw::Descriptor {
            if index == 0 {
                &PLUGIN_DESCRIPTOR
            } else {
                std::ptr::null()
            }
        }
    };
}
