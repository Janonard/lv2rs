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

/// The slice that contains the feature references.
pub type FeaturesList = [&'static Feature];

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

    /// Try to find a feature in the features list by it's URI.
    ///
    /// This function is safe, since the data pointer is not touched at all and therefore, no UB
    /// can be triggered.
    pub fn get_feature_raw(features: &FeaturesList, uri: &CStr) -> Option<*mut c_void> {
        Some(
            (features
                .iter()
                .find(|feature| unsafe { CStr::from_ptr(feature.uri) } == uri)?)
            .data,
        )
    }

    /// Try to find a feature in the features list and cast the data.
    ///
    /// This function in unsafe, since it can not check if the data is of type T. It is your
    /// responsibility to ensure the soundness of the cast.
    pub unsafe fn get_feature<T>(features: &FeaturesList, uri: &CStr) -> Option<&'static mut T> {
        let feature = Self::get_feature_raw(features, uri)?;
        (feature as *mut T).as_mut()
    }
}

#[cfg(test)]
#[test]
fn test_map_features() {
    const FEATURE_0_URI: &[u8] = b"http://example.org/Feature0\0";
    const FEATURE_0_DATA: f64 = 42.0;
    const FEATURE_0: Feature = Feature {
        uri: FEATURE_0_URI.as_ptr() as *const c_char,
        data: &FEATURE_0_DATA as *const f64 as *mut f64 as *mut c_void,
    };

    const FEATURE_1_URI: &[u8] = b"http://example.org/Feature1\0";
    const FEATURE_1_DATA: f64 = 17.0;
    const FEATURE_1: Feature = Feature {
        uri: FEATURE_1_URI.as_ptr() as *const c_char,
        data: &FEATURE_1_DATA as *const f64 as *mut f64 as *mut c_void,
    };

    const FEATURES: [&Feature; 2] = [&FEATURE_0, &FEATURE_1];

    unsafe {
        let feature_0_data = Feature::get_feature::<f64>(
            &FEATURES,
            CStr::from_bytes_with_nul(FEATURE_0_URI).unwrap(),
        )
        .unwrap();
        assert_eq!(42.0, *feature_0_data);

        let feature_1_data = Feature::get_feature::<f64>(
            &FEATURES,
            CStr::from_bytes_with_nul(FEATURE_1_URI).unwrap(),
        )
        .unwrap();
        assert_eq!(17.0, *feature_1_data);
    }
}
