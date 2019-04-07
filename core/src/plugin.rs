//! General Plugin-related traits and functions.
use crate::{Feature, FeaturesList};

use std::ffi::CStr;
use std::os::raw::*;

/**
   Plugin Instance Handle.
*/
pub type Handle = *mut c_void;

/**
   Plugin Descriptor.

   This structure provides the core functions necessary to instantiate and use
   a plugin.
*/
#[repr(C)]
pub struct Descriptor {
    /**
       A globally unique, case-sensitive identifier for this plugin.

       This MUST be a valid URI string as defined by RFC 3986.  All plugins with
       the same URI MUST be compatible to some degree, see
       http://lv2plug.in/ns/lv2core for details.
    */
    pub uri: *const c_char,

    /**
       Instantiate the plugin.

       Note that instance initialisation should generally occur in activate()
       rather than here. If a host calls instantiate(), it MUST call cleanup()
       at some point in the future.

       descriptor: Descriptor of the plugin to instantiate.

       sample_rate: Sample rate, in Hz, for the new plugin instance.

       bundle_path: Path to the LV2 bundle which contains this plugin
       binary. It MUST include the trailing directory separator (e.g. '/') so
       that simply appending a filename will yield the path to that file in the
       bundle.

       features: A NULL terminated array of LV2_Feature structs which
       represent the features the host supports. Plugins may refuse to
       instantiate if required features are not found here. However, hosts MUST
       NOT use this as a discovery mechanism: instead, use the RDF data to
       determine which features are required and do not attempt to instantiate
       unsupported plugins at all. This parameter MUST NOT be NULL, i.e. a host
       that supports no features MUST pass a single element array containing
       NULL.

       return value: A handle for the new plugin instance, or NULL if instantiation
       has failed.
    */
    pub instantiate: unsafe extern "C" fn(
        descriptor: *const Descriptor,
        sample_rate: f64,
        bundle_path: *const c_char,
        features: *const *const Feature,
    ) -> Handle,

    /**
       Connect a port on a plugin instance to a memory location.

       Plugin writers should be aware that the host may elect to use the same
       buffer for more than one port and even use the same buffer for both
       input and output (see lv2:inPlaceBroken in lv2.ttl).

       If the plugin has the feature lv2:hardRTCapable then there are various
       things that the plugin MUST NOT do within the connect_port() function;
       see lv2core.ttl for details.

       connect_port() MUST be called at least once for each port before run()
       is called, unless that port is lv2:connectionOptional. The plugin must
       pay careful attention to the block size passed to run() since the block
       allocated may only just be large enough to contain the data, and is not
       guaranteed to remain constant between run() calls.

       connect_port() may be called more than once for a plugin instance to
       allow the host to change the buffers that the plugin is reading or
       writing. These calls may be made before or after activate() or
       deactivate() calls.

       instance: Plugin instance containing the port.

       port: Index of the port to connect. The host MUST NOT try to
       connect a port index that is not defined in the plugin's RDF data. If
       it does, the plugin's behaviour is undefined (a crash is likely).

       data_location: Pointer to data of the type defined by the port
       type in the plugin's RDF data (e.g. an array of float for an
       lv2:AudioPort). This pointer must be stored by the plugin instance and
       used to read/write data when run() is called. Data present at the time
       of the connect_port() call MUST NOT be considered meaningful.
    */
    pub connect_port: unsafe extern "C" fn(instance: Handle, port: u32, data_location: *mut c_void),

    /**
       Initialise a plugin instance and activate it for use.

       This is separated from instantiate() to aid real-time support and so
       that hosts can reinitialise a plugin instance by calling deactivate()
       and then activate(). In this case the plugin instance MUST reset all
       state information dependent on the history of the plugin instance except
       for any data locations provided by connect_port(). If there is nothing
       for activate() to do then this field may be NULL.

       When present, hosts MUST call this function once before run() is called
       for the first time. This call SHOULD be made as close to the run() call
       as possible and indicates to real-time plugins that they are now live,
       however plugins MUST NOT rely on a prompt call to run() after
       activate().

       The host MUST NOT call activate() again until deactivate() has been
       called first. If a host calls activate(), it MUST call deactivate() at
       some point in the future. Note that connect_port() may be called before
       or after activate().
    */
    pub activate: unsafe extern "C" fn(instance: Handle),

    /**
       Run a plugin instance for a block.

       Note that if an activate() function exists then it must be called before
       run(). If deactivate() is called for a plugin instance then run() may
       not be called until activate() has been called again.

       If the plugin has the feature lv2:hardRTCapable then there are various
       things that the plugin MUST NOT do within the run() function (see
       lv2core.ttl for details).

       As a special case, when `sample_count` is 0, the plugin should update
       any output ports that represent a single instant in time (e.g. control
       ports, but not audio ports). This is particularly useful for latent
       plugins, which should update their latency output port so hosts can
       pre-roll plugins to compute latency. Plugins MUST NOT crash when
       `sample_count` is 0.

       instance: Instance to be run.

       sample_count: The block size (in samples) for which the plugin
       instance must run.
    */
    pub run: unsafe extern "C" fn(instance: Handle, n_samples: u32),

    /**
       Deactivate a plugin instance (counterpart to activate()).

       Hosts MUST deactivate all activated instances after they have been run()
       for the last time. This call SHOULD be made as close to the last run()
       call as possible and indicates to real-time plugins that they are no
       longer live, however plugins MUST NOT rely on prompt deactivation. If
       there is nothing for deactivate() to do then this field may be NULL

       Deactivation is not similar to pausing since the plugin instance will be
       reinitialised by activate(). However, deactivate() itself MUST NOT fully
       reset plugin state. For example, the host may deactivate a plugin, then
       store its state (using some extension to do so).

       Hosts MUST NOT call deactivate() unless activate() was previously
       called. Note that connect_port() may be called before or after
       deactivate().
    */
    pub deactivate: unsafe extern "C" fn(instance: Handle),

    /**
       Clean up a plugin instance (counterpart to instantiate()).

       Once an instance of a plugin has been finished with it must be deleted
       using this function. The instance handle passed ceases to be valid after
       this call.

       If activate() was called for a plugin instance then a corresponding call
       to deactivate() MUST be made before cleanup() is called. Hosts MUST NOT
       call cleanup() unless instantiate() was previously called.
    */
    pub cleanup: unsafe extern "C" fn(instance: Handle),

    /**
       Return additional plugin data defined by some extenion.

       A typical use of this facility is to return a struct containing function
       pointers to extend the LV2_Descriptor API.

       The actual type and meaning of the returned object MUST be specified
       precisely by the extension. This function MUST return NULL for any
       unsupported URI. If a plugin does not support any extension data, this
       field may be NULL.

       The host is never responsible for freeing the returned value.
    */
    pub extension_data: unsafe extern "C" fn(uri: *const c_char) -> *const c_void,
}

/// LV2 plugin trait.
///
/// This trait helps you implementing plugins, since it requires you to implement all
/// necessary functions.
///
/// Almost every plugin function call from the host will be checked and "safed" before these trait
/// functions are called. Therefore, all of them are safe.
pub trait Plugin {
    /// Create a new instance of the plugin.
    ///
    /// Here, you should instantiate the plugin and supply it with general information. You can look
    /// at the plugin descriptor, the audio frame rate of the current session, the path from which
    /// the host has loaded the plugin and an slice of references to the features supported by the
    /// host. If, for one reason or another, you find yourself in a situation where you can't
    /// create a plugin instance, you can return `None`.
    fn instantiate(
        descriptor: &Descriptor,
        rate: f64,
        bundle_path: &CStr,
        features: Option<&FeaturesList>,
    ) -> Option<Self>
    where
        Self: Sized;

    /// Set internal data pointers.
    ///
    /// This function will be called by the host when the location of a port has changed and the
    /// plugin should update it's internal pointers.
    ///
    /// When this function is called, the data pointers may not be valid yet and therefore, you
    /// shouldn't use them.
    fn connect_port(&mut self, port: u32, data: *mut ());

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

/// Marker trait for extension data.
///
/// This trait was introduced in order to make a [`Plugin`](trait.Plugin.html)'s
/// [`extension_data`](trait.Plugin.html#method.extension_data) function more dynamic.
/// Apart from that, it has absolutely no meaning.
pub trait ExtensionData {}

/// Helper function for the `instantiate` plugin call.
///
/// This function takes the raw parameters provided by the C API and turns them into safe Rust data
/// types. Only functions generated by the `lv2_main` should call the function any other should not.
pub unsafe fn instantiate<P: Plugin>(
    descriptor: *const Descriptor,
    rate: f64,
    bundle_path: *const c_char,
    features: *const *const Feature,
) -> Handle {
    let descriptor = match descriptor.as_ref() {
        Some(desc) => desc,
        None => return std::ptr::null_mut(),
    };
    let bundle_path = if bundle_path.is_null() {
        return std::ptr::null_mut();
    } else {
        CStr::from_ptr(bundle_path as *const c_char)
    };

    let features = {
        if features.is_null() {
            None
        } else {
            let mut length = 0;
            let mut features_cp = features;
            loop {
                match features_cp.as_ref() {
                    Some(feature) => {
                        if feature.is_null() {
                            break;
                        } else {
                            length += 1;
                            features_cp = features_cp.add(1);
                        }
                    }
                    None => return std::ptr::null_mut(),
                }
            }
            Some(std::slice::from_raw_parts(
                features as *const &'static Feature,
                length,
            ))
        }
    };

    match P::instantiate(descriptor, rate, bundle_path, features) {
        Some(instance) => {
            let instance = Box::new(instance);
            Box::leak(instance) as *const P as Handle
        }
        None => std::ptr::null_mut(),
    }
}

/// Helper function for the `connect_port` plugin call.
///
/// This function takes the raw parameters provided by the C API and turns them into safe Rust data
/// types. Only functions generated by the `lv2_main` should call the function any other should not.
pub unsafe fn connect_port<P: Plugin>(instance: Handle, port: u32, data: *mut c_void) {
    let instance = (instance as *mut P).as_mut().unwrap();
    instance.connect_port(port, data as *mut ());
}

/// Helper function for the `activate` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn activate<P: Plugin>(instance: Handle) {
    let instance = (instance as *mut P).as_mut().unwrap();
    instance.activate();
}

/// Helper function for the `run` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn run<P: Plugin>(instance: Handle, n_samples: u32) {
    let instance = (instance as *mut P).as_mut().unwrap();
    instance.run(n_samples);
}

/// Helper function for the `deactivate` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn deactivate<P: Plugin>(instance: Handle) {
    let instance = (instance as *mut P).as_mut().unwrap();
    instance.deactivate();
}

/// Helper function for the `cleanup` plugin call.
///
/// This function takes the raw parameters provided by the C API, turns them into safe Rust data
/// types, and calls the trait's function. Only functions generated by the `lv2_main` should call
/// this function, any other must not.
pub unsafe fn cleanup<P: Plugin>(instance: Handle) {
    core::ptr::drop_in_place(instance as *mut P);
}

/// Helper function for the `extension_data` plugin call.
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
