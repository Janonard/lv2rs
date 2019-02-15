extern crate lv2rs_core as core;

pub mod uris;

mod raw;

pub use raw::URID;

pub struct Map {
    raw: &'static raw::Map,
}

impl Map {
    pub fn try_from_feature(feature: &core::feature::Feature) -> Result<Self, ()> {
        let feature_uri = feature.get_uri()?;
        if *(feature_uri.to_bytes()) == *(uris::MAP_URI) {
            match unsafe { (feature.get_data() as *const raw::Map).as_ref() } {
                Some(map) => Ok(Self { raw: map }),
                None => Err(()),
            }
        } else {
            Err(())
        }
    }

    pub fn map(&self, uri: &std::ffi::CStr) -> URID {
        (self.raw.map)(self.raw.handle, uri.as_ptr())
    }
}

pub struct Unmap {
    raw: &'static raw::Unmap,
}

impl Unmap {
    pub fn try_from_feature(feature: &core::feature::Feature) -> Result<Self, ()> {
        let feature_uri = feature.get_uri()?;
        if *(feature_uri.to_bytes()) == *(uris::UNMAP_URI) {
            match unsafe { (feature.get_data() as *const raw::Unmap).as_ref() } {
                Some(unmap) => Ok(Self { raw: unmap }),
                None => Err(()),
            }
        } else {
            Err(())
        }
    }

    pub fn unmap(&self, urid: URID) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr((self.raw.unmap)(self.raw.handle, urid)) }
    }
}
