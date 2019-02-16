//! General Plugin-related traits and functions.
use crate::raw;

use std::ffi::CStr;
use std::os::raw::*;

/// LV2 plugin trait.
///
/// This trait helps you implementing plugins, since it requires you to implement all
/// necessary functions.
///
/// Almost every plugin function call from the host will be checked and "safed" before these trait
/// functions may be called. Therefore, most of them are safe, except for one:
/// [`connect_port`](#tymethod.connect_port). See it's documentations for more information on why it
/// is unsafe.
pub trait Plugin {
    /// Create a new instance of the plugin.
    ///
    /// Here, you should instantiate the plugin and supply it with general information. You can look
    /// at the plugin descriptor (although you shouldn't), the audio frame rate of the current
    /// session, the path from which the host has loaded the plugin and an iterator over features
    /// supported by the host.
    fn instantiate(
        descriptor: &raw::Descriptor,
        rate: f64,
        bundle_path: &CStr,
        features: FeatureIterator,
    ) -> Self;

    /// Set internal data pointers.
    ///
    /// This function will be called by the host when the location of a port has changed and the
    /// plugin should update it's internal pointers. This function is highly unsafe, since the type
    /// of the pointed data is generally unknown. The only thing that gives a clue on the type is
    /// the id of the port, which should match with the port specified in the plugin's turtle
    /// document.
    ///
    /// When this function is called, the data pointers may not be valid yet and therefore, you
    /// shouldn't use them. Also, if the host passes pointers that will never be valid, you cannot
    /// defend yourselves from undefined behaviour, and you should not, in any case, call this
    /// function on your own.
    unsafe fn connect_port(&mut self, port: u32, data: *mut ());

    /// Activate the plugin.
    ///
    /// If your plugin can be turned on or off, you should override this function and set the plugin
    /// up for active use.
    ///
    /// The default implementation does nothing.
    fn activate(&mut self) {}

    /// Run plugin specific operations.
    ///
    /// This is where the action happens! Here, you should execute the actions that make your plugin
    /// unique.
    ///
    /// Pointers, which were previously set by the [`connect_port`](#tyfunction.connect_port)
    /// function are guaranteed to be valid now. If they aren't, it's the host's fault, not yours.
    /// Also, sample arrays or atom sequence will have a length  of `n_samples` elements. This
    /// number may change during the life time of the plugin and therefore, you should not store
    /// it somewhere.
    fn run(&mut self, n_samples: u32);

    /// Deactivate the plugin.
    ///
    /// If your plugin can be turned on or off, you should override this function and destroy the
    /// plugins active state.
    ///
    /// The default implementation does nothing.
    fn deactivate(&mut self) {}

    /// Return extension specific data to the host.
    ///
    /// Some LV2 extensions require special data from a plugin in order to work. This is where you
    /// provide the data. The passed C string reference is the URI of the extension in question and
    /// you can return a static reference to some data. If you do not know the passed URI, you
    /// should return `None`.
    ///
    /// The return value must be a static reference since we don't know how long it needs to be
    /// alive; as stated in the [LV2 header](http://lv2plug.in/doc/html/group__core.html#ae907a7668d6579f099ac08c134b2e634),
    /// the host is not responsible for freeing the returned value. Therefore, the referenced data
    /// need to live for the entirety of the program.
    fn extension_data(_uri: &CStr) -> Option<&'static ExtensionData> {
        None
    }
}

/// Safe wrapper for a host feature.
pub struct Feature {
    feature: *mut raw::Feature,
}

impl Feature {
    fn new(feature: *mut raw::Feature) -> Self {
        Self { feature: feature }
    }

    /// Try to get the URI of the feature.
    ///
    /// None if the internal feature or the the URI is pointing to null.
    pub fn get_uri(&self) -> Option<&CStr> {
        match unsafe { self.feature.as_ref() } {
            Some(feature) => {
                if !feature.uri.is_null() {
                    Some(unsafe { CStr::from_ptr(feature.uri) })
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Try to get a pointer to the feature's data.
    ///
    /// None if the internal feature is pointing to null.
    pub fn get_data(&mut self) -> Option<*mut ()> {
        match unsafe { self.feature.as_mut() } {
            Some(feature) => Some(feature.data as *mut ()),
            None => None,
        }
    }

    /// Return a casted data pointer if the given URI matches the feature's URI.
    ///
    /// No checks are made if the pointer is valid or the returned value is actually of type T.
    /// Therefore, this function is unsafe.
    ///
    /// Also, this function returns None if the URI didn't match or the internal pointers are
    /// invalid.
    pub unsafe fn get_data_if_uri_matches<T>(&mut self, uri: &[u8]) -> Option<&mut T> {
        let feature_uri = match self.get_uri() {
            Some(uri) => uri,
            None => return None,
        };
        if *(feature_uri.to_bytes()) == *(uri) {
            let data_ptr = match self.get_data() {
                Some(data) => data as *mut T,
                None => return None,
            };
            data_ptr.as_mut()
        } else {
            None
        }
    }
}

/// An iterator over raw host features.
pub struct FeatureIterator {
    raw: *const *const raw::Feature,
}

impl FeatureIterator {
    fn new(raw: *const *const raw::Feature) -> Self {
        Self { raw: raw }
    }
}

impl std::iter::Iterator for FeatureIterator {
    type Item = Feature;

    fn next(&mut self) -> Option<Self::Item> {
        let feature_ptr = match unsafe { self.raw.as_ref() } {
            // From a Rust perspective, this is an odd thing: The feature pointers are `const`,
            // but the data pointer is `mut`. C is a strange language.
            Some(feature) => (*feature) as *mut raw::Feature,
            None => return None,
        };
        if !feature_ptr.is_null() {
            self.raw = unsafe { self.raw.add(1) };
            Some(Feature::new(feature_ptr))
        } else {
            None
        }
    }
}

/// Marker trait for extension data.
///
/// This trait was introduced in order to make a [`Plugin`](trait.Plugin.html)'s
/// [`extension_data`](trait.Plugin.html#method.extension_data) function more dynamic.
/// Apart from that, it has absolutely no meaning.
pub trait ExtensionData {}

/// "Saver" function for the `instantiate` plugin call.
///
/// This function takes the raw parameters provided by the C API and turns them into safe Rust data
/// types. Only functions generated by the `lv2_main` should call the function any other should not.
pub unsafe fn instantiate<P: Plugin>(
    descriptor: *const raw::Descriptor,
    rate: f64,
    bundle_path: *const c_char,
    features: *const *const raw::Feature,
) -> crate::raw::Handle {
    let descriptor = descriptor.as_ref().unwrap();
    let bundle_path = CStr::from_ptr(bundle_path as *const c_char);
    let features = FeatureIterator::new(features);

    let instance = Box::new(P::instantiate(descriptor, rate, bundle_path, features));

    std::mem::forget(bundle_path);
    Box::leak(instance) as *const P as raw::Handle
}

/// "Saver" function for the `connect_port` plugin call.
///
/// This function takes the raw parameters provided by the C API and turns them into safe Rust data
/// types. Only functions generated by the `lv2_main` should call the function any other should not.
pub unsafe fn connect_port<P: Plugin>(instance: raw::Handle, port: u32, data: *mut c_void) {
    let instance = (instance as *mut P).as_mut().unwrap();
    instance.connect_port(port, data as *mut ());
}

/// "Saver" function for the `activate` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn activate<P: Plugin>(instance: raw::Handle) {
    let instance = (instance as *mut P).as_mut().unwrap();
    instance.activate();
}

/// "Saver" function for the `run` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn run<P: Plugin>(instance: raw::Handle, n_samples: u32) {
    let instance = (instance as *mut P).as_mut().unwrap();
    instance.run(n_samples);
}

/// "Saver" function for the `deactivate` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn deactivate<P: Plugin>(instance: raw::Handle) {
    let instance = (instance as *mut P).as_mut().unwrap();
    instance.deactivate();
}

/// "Saver" function for the `cleanup` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn cleanup<P: Plugin>(instance: raw::Handle) {
    core::ptr::drop_in_place(instance as *mut P);
}

/// "Saver" function for the `extension_data` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn extension_data<P: Plugin>(uri: *const c_char) -> *const c_void {
    let uri = CStr::from_ptr(uri);
    let result = P::extension_data(uri);
    std::mem::forget(uri);
    match result {
        Some(ext_data) => ext_data as *const ExtensionData as *const c_void,
        None => std::ptr::null(),
    }
}
