//! Placeholder URID mapper for debugging purposes.
//!
//! Many LV2 libraries depend on URID mapping and therefore, a mapper is needed when debugging and
//! testing. However, testing can (and should) not be done within a running host. This is where
//! these utilities come in hand: They map URIs to unique URIDs and backwards without needing an
//! external host.
use crate::{MapHandle, URID};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::marker::PhantomPinned;
use std::os::raw::*;

/// Container holding both the mapping feature and a storage for URID mappings.
///
/// Since the mapping feature contains a raw pointer to the storage, this struct must be pinned.
/// This means that it cannot be moved.
pub struct DebugMap {
    storage: HashMap<CString, URID>,
    feature: crate::Map,
    _pin: PhantomPinned,
}

extern "C" fn mapping_fn(handle: MapHandle, uri: *const c_char) -> URID {
    let map = unsafe { (handle as *mut HashMap<CString, URID>).as_mut() }.unwrap();
    let uri = unsafe { CStr::from_ptr(uri) }.to_owned();

    if !map.contains_key(&uri) {
        let biggest_urid = map.values().map(|urid: &URID| -> URID { *urid }).max();
        let new_urid = match biggest_urid {
            Some(urid) => urid + 1,
            None => 0,
        };
        map.insert(uri.clone(), new_urid);
    }

    map[&uri]
}

impl DebugMap {
    /// Create a new debug map in a box.
    pub fn new() -> Box<Self> {
        let mut debug_map = Box::new(Self {
            storage: HashMap::new(),
            feature: crate::Map {
                handle: std::ptr::null_mut(),
                map: mapping_fn,
            },
            _pin: PhantomPinned,
        });
        debug_map.feature.handle =
            &mut debug_map.storage as *mut HashMap<CString, URID> as *mut c_void;
        debug_map
    }

    /// Return a reference to the mapping feature.
    pub fn get_map_ref(&self) -> &crate::Map {
        &self.feature
    }

    /// Return a mutable reference to the mapping feature.
    pub fn get_map_mut(&mut self) -> &mut crate::Map {
        &mut self.feature
    }

    /// Create a cached map.
    ///
    /// Technically, this is useless since this debug map already contains the URIDs in a `HashMap`,
    /// but many LV2 libraries use the `CachedMap` and therefore need such an object.
    ///
    /// This method is unsafe since it has to fake the lifetime of the mapping feature. In reality,
    /// this mapping feature only lives as long as the `DebugMap` exists, but the `CachedMap`
    /// expects the mapping feature to come from a host. This implies that the mapping feature will
    /// live for the whole lifetime of the plugin and therefore is static.
    pub unsafe fn create_cached_map(&mut self) -> crate::CachedMap {
        let faked_map: &'static mut crate::Map =
            (self.get_map_mut() as *mut crate::Map).as_mut().unwrap();
        crate::CachedMap::new(faked_map)
    }
}
#[cfg(test)]
mod test {
    use crate::debug::*;

    const GITHUB_URI: &[u8] = b"https://github.com\0";
    const GITLAB_URI: &[u8] = b"https://gitlab.com\0";

    #[test]
    fn test_mapping() {
        let mut debug_map = DebugMap::new();
        let map = debug_map.get_map_mut();

        let github_urid = map.map(CStr::from_bytes_with_nul(GITHUB_URI).unwrap());
        let gitlab_urid = map.map(CStr::from_bytes_with_nul(GITLAB_URI).unwrap());

        assert_ne!(github_urid, gitlab_urid);
    }

    #[test]
    fn test_cached_mapping() {
        let mut debug_map = DebugMap::new();
        let mut cached_map = unsafe { debug_map.create_cached_map() };

        let github_urid = cached_map.map(CStr::from_bytes_with_nul(GITHUB_URI).unwrap());
        let gitlab_urid = cached_map.map(CStr::from_bytes_with_nul(GITLAB_URI).unwrap());

        assert_ne!(github_urid, gitlab_urid);
    }
}
