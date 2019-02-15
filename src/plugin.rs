use crate::feature::FeatureIterator;
use crate::raw;

use std::os::raw::*;
use std::ffi::CStr;

pub trait Plugin {
    fn instantiate(rate: f64, bundle_path: &CStr, features: FeatureIterator) -> Self;

    fn connect_port(&mut self, port: u32, data: *mut ());

    fn activate(&mut self) {}

    fn run(&mut self, n_samples: u32);

    fn deactivate(&mut self) {}

    fn extension_data(_uri: &CStr) -> *const () {
        std::ptr::null()
    }
}

pub fn instantiate<P: Plugin>(
    rate: f64,
    bundle_path: *const c_char,
    features: *const *const raw::Feature,
) -> crate::raw::Handle {
    let bundle_path = unsafe { CStr::from_ptr(bundle_path as *const c_char) };
    let features = FeatureIterator::new(features);

    let instance = Box::new(P::instantiate(rate, bundle_path, features));

    std::mem::forget(bundle_path);
    Box::leak(instance) as *const P as raw::Handle
}

pub fn connect_port<P: Plugin>(instance: raw::Handle, port: u32, data: *mut c_void) {
    let instance = unsafe { (instance as *mut P).as_mut() }.unwrap();
    instance.connect_port(port, data as *mut ());
}

pub fn activate<P: Plugin>(instance: raw::Handle) {
    let instance = unsafe { (instance as *mut P).as_mut() }.unwrap();
    instance.activate();
}

pub fn run<P: Plugin>(instance: raw::Handle, n_samples: u32) {
    let instance = unsafe { (instance as *mut P).as_mut() }.unwrap();
    instance.run(n_samples);
}

pub fn deactivate<P: Plugin>(instance: raw::Handle) {
    let instance = unsafe { (instance as *mut P).as_mut() }.unwrap();
    instance.deactivate();
}

pub fn cleanup<P: Plugin>(instance: raw::Handle) {
    unsafe {
        core::ptr::drop_in_place(instance as *mut P);
    }
}

pub fn extension_data<P: Plugin>(uri: *const c_char) -> *const c_void {
    let uri = unsafe { CStr::from_ptr(uri) };
    let result = P::extension_data(uri);
    std::mem::forget(uri);
    result as *const c_void
}
