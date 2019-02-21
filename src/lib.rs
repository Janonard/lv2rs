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
        match features.get(unsafe { CStr::from_bytes_with_nul_unchecked(uris::MAP_URI) }) {
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
    raw_map: &'a mut Map,
    raw_unmap: &'a mut Unmap,
    map_cache: HashMap<CString, URID>,
    unmap_cache: HashMap<URID, CString>,
}

impl<'a> CachedMap<'a> {
    pub fn new(raw_map: &'a mut Map, raw_unmap: &'a mut Unmap) -> CachedMap<'a> {
        Self {
            raw_map: raw_map,
            raw_unmap: raw_unmap,
            map_cache: HashMap::new(),
            unmap_cache: HashMap::new(),
        }
    }

    pub fn try_from_features(features: &'a HashMap<&CStr, *mut ()>) -> Option<Self> {
        let raw_map = Map::try_from_features(features);
        let raw_unmap = Unmap::try_from_features(features);
        if raw_map.is_none() | raw_unmap.is_none() {
            return None;
        } else {
            let raw_map = raw_map.unwrap();
            let raw_unmap = raw_unmap.unwrap();
            Some(Self::new(raw_map, raw_unmap))
        }
    }

    pub fn map_cache(&self) -> &HashMap<CString, URID> {
        &self.map_cache
    }

    pub fn unmap_cache(&self) -> &HashMap<URID, CString> {
        &self.unmap_cache
    }

    pub fn map(&mut self, uri: &CString) -> Result<URID, ()> {
        if !self.map_cache.contains_key(uri) {
            let urid = self.raw_map.map(uri.as_c_str());
            // check for inconsistencies.
            match self.unmap_cache.get(&urid) {
                Some(unmapped_uri) => {
                    if *uri != *unmapped_uri {
                        return Err(());
                    }
                }
                None => (),
            }
            self.map_cache.insert(uri.clone(), urid);
        }
        Ok(*(self.map_cache.get(uri).unwrap()))
    }

    pub fn unmap(&mut self, urid: URID) -> Result<&CString, ()> {
        if !self.unmap_cache.contains_key(&urid) {
            let uri = self.raw_unmap.unmap(urid);
            let uri = CString::from(uri);
            // check for inconsistencies.
            match self.map_cache.get(&uri) {
                Some(mapped_urid) => {
                    if urid != *mapped_urid {
                        return Err(());
                    }
                }
                None => (),
            }
            self.unmap_cache.insert(urid, uri);
        }
        Ok(self.unmap_cache.get(&urid).unwrap())
    }
}
