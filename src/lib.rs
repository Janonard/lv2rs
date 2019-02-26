extern crate lv2rs_core as core;

pub mod uris;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::*;

pub type MapHandle = *mut c_void;
pub type UnmapHandle = *mut c_void;
pub type URID = u32;

#[repr(C)]
pub struct Map {
    pub handle: MapHandle,
    pub map: extern "C" fn(handle: MapHandle, uri: *const c_char) -> URID,
}

impl Map {
    pub fn try_from_features<'a>(features: &'a HashMap<&CStr, *mut ()>) -> Option<&'a mut Self> {
        match features.get(unsafe { CStr::from_bytes_with_nul_unchecked(uris::MAP_URI) }) {
            Some(data) => Some(unsafe { (*data as *mut Self).as_mut() }.unwrap()),
            None => None,
        }
    }

    pub fn map(&mut self, uri: &CStr) -> URID {
        (self.map)(self.handle, uri.as_ptr())
    }
}

#[repr(C)]
pub struct Unmap {
    pub handle: MapHandle,
    pub unmap: extern "C" fn(handle: UnmapHandle, urid: URID) -> *const c_char,
}

impl Unmap {
    pub fn try_from_features<'a>(features: &'a HashMap<&CStr, *mut ()>) -> Option<&'a mut Self> {
        match features.get(unsafe { CStr::from_bytes_with_nul_unchecked(uris::UNMAP_URI) }) {
            Some(data) => Some(unsafe { (*data as *mut Self).as_mut() }.unwrap()),
            None => None,
        }
    }

    pub fn unmap(&mut self, urid: URID) -> &CStr {
        let uri = (self.unmap)(self.handle, urid);
        unsafe { CStr::from_ptr(uri) }
    }
}

pub struct CachedMap<'a> {
    raw: &'a mut Map,
    cache: HashMap<CString, URID>,
}

impl<'a> CachedMap<'a> {
    pub fn new(raw: &'a mut Map) -> CachedMap<'a> {
        Self {
            raw: raw,
            cache: HashMap::new(),
        }
    }

    pub fn try_from_features(features: &'a HashMap<&CStr, *mut ()>) -> Option<Self> {
        let raw_map = Map::try_from_features(features);
        if raw_map.is_none() {
            return None;
        } else {
            let raw_map = raw_map.unwrap();
            Some(Self::new(raw_map))
        }
    }

    pub fn cache(&self) -> &HashMap<CString, URID> {
        &self.cache
    }

    pub fn map(&mut self, uri: &CString) -> URID {
        if !self.cache.contains_key(uri) {
            let urid = self.raw.map(uri.as_c_str());
            self.cache.insert(uri.clone(), urid);
        }
        *(self.cache.get(uri).unwrap())
    }
}

pub struct CachedUnmap<'a> {
    raw: &'a mut Unmap,
    cache: HashMap<URID, CString>,
}

impl<'a> CachedUnmap<'a> {
    pub fn new(raw_map: &'a mut Unmap) -> Self {
        Self {
            raw: raw_map,
            cache: HashMap::new(),
        }
    }

    pub fn try_from_features(features: &'a HashMap<&CStr, *mut ()>) -> Option<Self> {
        let raw_unmap = Unmap::try_from_features(features);
        if raw_unmap.is_none() {
            return None;
        } else {
            let raw_unmap = raw_unmap.unwrap();
            Some(Self::new(raw_unmap))
        }
    }

    pub fn cache(&self) -> &HashMap<URID, CString> {
        &self.cache
    }

    pub fn unmap(&mut self, urid: URID) -> &CString {
        if !self.cache.contains_key(&urid) {
            let uri = self.raw.unmap(urid);
            let uri = CString::from(uri);
            self.cache.insert(urid, uri);
        }
        self.cache.get(&urid).unwrap()
    }
}
