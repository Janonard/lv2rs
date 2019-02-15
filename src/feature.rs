use crate::raw;
use std::ffi::CStr;

#[derive(Clone)]
pub struct Feature {
    feature: &'static raw::Feature,
}

impl Feature {
    fn new(feature: &'static raw::Feature) -> Self {
        Self { feature: feature }
    }

    pub fn get_uri(&self) -> Result<&'static CStr, ()> {
        match unsafe { self.feature.uri.as_ref() } {
            Some(uri) => Ok(unsafe { CStr::from_ptr(uri) }),
            None => Err(()),
        }
    }

    pub fn get_data(&self) -> *mut () {
        self.feature.data as *mut ()
    }
}

#[derive(Clone)]
pub struct FeatureIterator {
    raw: *const *const raw::Feature,
}

impl FeatureIterator {
    pub fn new(raw: *const *const raw::Feature) -> Self {
        Self { raw: raw }
    }
}

impl std::iter::Iterator for FeatureIterator {
    type Item = Feature;

    fn next(&mut self) -> Option<Self::Item> {
        if self.raw.is_null() {
            None
        } else {
            match unsafe { (*self.raw).as_ref() } {
                Some(feature) => {
                    self.raw = unsafe { self.raw.add(1) };
                    Some(Feature::new(feature))
                }
                None => None,
            }
        }
    }
}
