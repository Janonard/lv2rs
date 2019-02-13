extern crate lv2_core;
extern crate lv2_raw;
pub mod uris;

pub use lv2_raw::urid::LV2Urid as urid;

pub struct Map {
    raw: &'static lv2_raw::urid::LV2UridMap,
}

impl Map {
    pub fn from_features_iter(iter: lv2_core::FeatureIterator) -> Result<Self, ()> {
        let feature_uri = std::ffi::CStr::from_bytes_with_nul(uris::MAP_URI).unwrap();

        for feature in iter {
            let uri = match feature.get_uri() {
                Some(uri) => uri,
                None => continue,
            };

            if uri != feature_uri {
                continue;
            }

            let map = feature.get_data() as *const lv2_raw::urid::LV2UridMap;
            match unsafe { map.as_ref() } {
                Some(map) => return Ok(Self { raw: map }),
                None => return Err(()),
            }
        }
        Err(())
    }

    pub fn map(&self, uri: &std::ffi::CStr) -> urid {
        (self.raw.map)(self.raw.handle, uri.as_ptr())
    }
}
