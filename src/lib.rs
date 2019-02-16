extern crate lv2rs_core as core;

pub mod uris;

mod raw;

pub use raw::URID;

pub struct Map<'a> {
    raw: &'a mut raw::Map,
}

impl<'a> Map<'a> {
    pub fn try_from_feature(feature: &'a mut core::Feature) -> Option<Self> {
        match unsafe { feature.get_data_if_uri_matches(uris::MAP_URI) } {
            Some(map) => Some(Self { raw: map }),
            None => return None,
        }
    }

    pub fn map(&mut self, uri: &std::ffi::CStr) -> URID {
        (self.raw.map)(self.raw.handle, uri.as_ptr())
    }
}

pub struct Unmap<'a> {
    raw: &'a mut raw::Unmap,
}

impl<'a> Unmap<'a> {
    pub fn try_from_feature(feature: &'a mut core::Feature) -> Option<Self> {
        match unsafe { feature.get_data_if_uri_matches(uris::MAP_URI) } {
            Some(unmap) => Some(Self { raw: unmap }),
            None => return None,
        }
    }

    pub fn unmap(&mut self, urid: URID) -> &std::ffi::CStr {
        let uri = (self.raw.unmap)(self.raw.handle, urid);
        unsafe { std::ffi::CStr::from_ptr(uri) }
    }
}
