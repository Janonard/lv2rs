use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::*;

/**
   Feature.

   Features allow hosts to make additional functionality available to plugins
   without requiring modification to the LV2 API.  Extensions may define new
   features and specify the `uri` and `data` to be used if necessary.
   Some features, such as lv2:isLive, do not require the host to pass data.
*/
#[repr(C)]
pub struct Feature {
    /**
       A globally unique, case-sensitive identifier (URI) for this feature.

       This MUST be a valid URI string as defined by RFC 3986.
    */
    uri: *const c_char,

    /**
       Pointer to arbitrary data.

       The format of this data is defined by the extension which describes the
       feature with the given `URI`.
    */
    data: *mut c_void,
}

impl Feature {
    /// Try to get the URI of the feature.
    ///
    /// None if  the URI is pointing to null.
    pub fn uri(&self) -> Option<&CStr> {
        if !self.uri.is_null() {
            Some(unsafe { CStr::from_ptr(self.uri) })
        } else {
            None
        }
    }

    /// Try to get a pointer to the feature's data.
    ///
    /// None if the internal feature is pointing to null.
    ///
    /// This function is unsafe, since we don't know if the data really has type T. It's your
    /// responsibility to ensure that the data has the correct type.
    pub unsafe fn data<T>(&mut self) -> Option<&mut T> {
        (self.data as *mut T).as_mut()
    }

    /// Walk through the provided features and collect them in a map.
    ///
    /// This is usually the first call to discover a host's features: When a plugin's
    /// [`instantiate`](trait.Plugin.html#tymethod.instantiate) function is called, you pass the
    /// given `Feature` array pointer to this function, which creates a map that maps every
    /// feature's URI to it's data. However, since the type of the data is not known, only a raw
    /// pointer is included, you have to deref it yourself.
    pub unsafe fn map_features(features: &[*mut Feature]) -> HashMap<&CStr, *mut ()> {
        let mut map = HashMap::new();
        for feature in features {
            match feature.as_mut() {
                Some(feature) => {
                    map.insert(CStr::from_ptr(feature.uri), feature.data as *mut ());
                }
                None => break,
            }
        }
        map
    }
}

#[cfg(test)]
#[test]
fn test_map_features() {
    let feature_0_uri = Box::new(b"http://example.org/Feature0\0");
    let mut feature_0_data = 42.0;
    let mut feature_0 = Feature {
        uri: feature_0_uri.as_ptr() as *const c_char,
        data: &mut feature_0_data as *mut f64 as *mut c_void,
    };

    let feature_1_uri = Box::new(b"http://example.org/Feature1\0");
    let mut feature_1_data = 17.0;
    let mut feature_1 = Feature {
        uri: feature_1_uri.as_ptr() as *const c_char,
        data: &mut feature_1_data as *mut f64 as *mut c_void,
    };

    let features: [*mut Feature; 3] = [&mut feature_0, &mut feature_1, std::ptr::null_mut()];
    let features_map = unsafe { Feature::map_features(&features) };
    assert_eq!(features_map.len(), 2);

    unsafe {
        // Create a clone of the uri. We want the HashMap to compare the content, not the addresses!
        let feature_0_uri = feature_0_uri.clone();
        let feature_0_uri = CStr::from_bytes_with_nul_unchecked(*feature_0_uri);
        let mapped_f0_data = (features_map[&feature_0_uri] as *const f64)
            .as_ref()
            .unwrap();
        assert_eq!(*mapped_f0_data, feature_0_data);

        let feature_1_uri = feature_1_uri.clone();
        let feature_1_uri = CStr::from_bytes_with_nul_unchecked(*feature_1_uri);
        let mapped_f1_data = (features_map[&feature_1_uri] as *const f64)
            .as_ref()
            .unwrap();
        assert_eq!(*mapped_f1_data, feature_1_data);
    }
}
