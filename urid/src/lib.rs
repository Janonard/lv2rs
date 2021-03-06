//! A Rust re-implementation of the LV2 URID library.
//!
//! This LV2 feature enables you to map URIs to numbers and reverse.
//! 
//! This is a frozen prototype and therefore, development of this crate will not continue here. Further
//! development continues as [rust-lv2](https://github.com/rust-dsp/rust-lv2).
//!
//! ## Use
//!
//! URID mapping is only possible in the `instantiate` function of a plugin since there is no
//! guarantee that the required pointers live longer than the `instantiate` function call. Here is
//! an example:
//!
//!     // import the required crates.
//!     extern crate lv2rs_core as core;
//!     extern crate lv2rs_urid as urid;
//!     use std::ffi::CStr;
//!     
//!     // A dummy plugin that doesn't actually do anything.
//!     struct UridPlugin {}
//!
//!     impl core::Plugin for UridPlugin {
//!         fn instantiate(
//!             descriptor: &core::Descriptor,
//!             rate: f64,
//!             bundle_path: &CStr,
//!             features: Option<&core::FeaturesList>
//!         ) -> Option<Self> where Self: Sized {
//!
//!             // Return `None` if there are no features.
//!             let features = features?;
//!
//!             // Try to get the mapper and the un-mapper from the features list.
//!             let map = urid::Map::try_from_features(features)?;
//!             let unmap = urid::Unmap::try_from_features(features)?;
//!
//!             // Create a URI, map it, and un-map it.
//!             let github_uri = CStr::from_bytes_with_nul(b"https://github.com\0").unwrap();
//!             let github_urid = map.map(github_uri);
//!             let github_uri = unmap.unmap(github_urid);
//!
//!             Some(Self {})
//!         }
//!
//!         // Blank implementations to keep the compiler quiet.
//!         fn connect_port(&mut self, _port: u32, _data: *mut ()) {}
//!         fn run(&mut self, _n_samples: u32) {}
//!     }
extern crate lv2rs_core as core;

pub mod debug;
pub mod uris;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::*;

/// Type to describe pointers to map handles.
pub type MapHandle = *mut c_void;

/// Type to describe pointers to unmap handles.
pub type UnmapHandle = *mut c_void;

/// Type for describing URIDs.
pub type URID = u32;

/// Struct for mapping URIs to URIDs.
#[repr(C)]
pub struct Map {
    /// Pointer to a host-specific handle to map URIs to URIDs.
    pub handle: MapHandle,
    /// Function that maps a URI to a URID.
    pub map: extern "C" fn(handle: MapHandle, uri: *const c_char) -> URID,
}

impl Map {
    /// Try to find the mapping feature in the features map.
    ///
    /// If this function returns None if the host does not support mapping.
    pub fn try_from_features<'a>(features: &core::FeaturesList) -> Option<&'static mut Self> {
        unsafe {
            core::Feature::get_feature::<Self>(
                features,
                CStr::from_bytes_with_nul(uris::MAP_URI).unwrap(),
            )
        }
    }

    /// Map a URI to a URID.
    ///
    /// If the host is properly implemented, this should be an injective function: Every URI should
    /// be mapped to a unique URID.
    pub fn map<'a, S>(&mut self, uri: S) -> URID
    where
        &'a CStr: From<S>,
    {
        let uri: &CStr = uri.into();
        (self.map)(self.handle, uri.as_ptr())
    }
}

/// Struct for mapping URIDs to URIs.
#[repr(C)]
pub struct Unmap {
    /// Pointer to a host-specific handle to map URIDs to URIs.
    pub handle: MapHandle,
    /// Function that maps a URID to a URI.
    pub unmap: extern "C" fn(handle: UnmapHandle, urid: URID) -> *const c_char,
}

impl Unmap {
    /// Try to find the unmapping feature in the features map.
    ///
    /// If this function returns None if the host does not support unmapping.
    pub fn try_from_features(features: &core::FeaturesList) -> Option<&'static mut Self> {
        unsafe {
            core::Feature::get_feature::<Self>(
                features,
                CStr::from_bytes_with_nul(uris::UNMAP_URI).unwrap(),
            )
        }
    }

    /// Try to unmap a URID to a URI.
    ///
    /// Since mapping URIs to URIDs may not be a surjective function, unmapping may be a partial
    /// function: Not every URID is necessarily mapped to a URI. Therefore, this function returns
    /// `None` if the given URID is not mapped.
    pub fn unmap(&mut self, urid: URID) -> Option<&CStr> {
        let uri = (self.unmap)(self.handle, urid);
        if uri.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(uri) })
        }
    }
}

/// Cached version of [Map](struct.Map.html)
pub struct CachedMap {
    raw: &'static mut Map,
    cache: HashMap<&'static CStr, URID>,
}

impl CachedMap {
    /// Create a new cached map from a mutable map reference.
    pub fn new(raw: &'static mut Map) -> CachedMap {
        Self {
            raw: raw,
            cache: HashMap::new(),
        }
    }

    /// Try to find the mapping feature in the features map.
    ///
    /// If this function returns `None` if the host does not support mapping.
    pub fn try_from_features(features: &core::FeaturesList) -> Option<Self> {
        let raw_map = Map::try_from_features(features)?;
        Some(Self::new(raw_map))
    }

    /// Return a reference to the cache.
    pub fn cache(&self) -> &HashMap<&'static CStr, URID> {
        &self.cache
    }

    /// Map a URI to a URID.
    ///
    /// The same rules from [Map.map](struct.Map.html#method.map) apply. Additionally, this function
    /// will cache the mappings and short-cut if a requested mapping is already cached.
    pub fn map(&mut self, uri: &'static CStr) -> URID {
        if !self.cache.contains_key(&uri) {
            let urid = self.raw.map(uri);
            self.cache.insert(uri.clone(), urid);
        }
        *(self.cache.get(&uri).unwrap())
    }
}
/// Cached version of [Unmap](struct.Unmap.html)
pub struct CachedUnmap {
    raw: &'static mut Unmap,
    cache: HashMap<URID, CString>,
}

impl CachedUnmap {
    /// Create a new cached unmap from a mutable unmap reference.
    pub fn new(raw_map: &'static mut Unmap) -> Self {
        Self {
            raw: raw_map,
            cache: HashMap::new(),
        }
    }

    /// Try to find the unmapping feature in the features map.
    ///
    /// If this function returns `None` if the host does not support unmapping.
    pub fn try_from_features(features: &core::FeaturesList) -> Option<Self> {
        let raw_unmap = Unmap::try_from_features(features)?;
        Some(Self::new(raw_unmap))
    }

    /// Return a reference to the cache.
    pub fn cache(&self) -> &HashMap<URID, CString> {
        &self.cache
    }

    /// Try to map a URID to a URI.
    ///
    /// The same rules from [Unmap.unmap](struct.Unmap.html#method.unmap) apply. Additionally, this
    /// function will cache the mappings and short-cut if a requested mapping is already cached.
    pub fn unmap(&mut self, urid: URID) -> Option<&CString> {
        if !self.cache.contains_key(&urid) {
            let uri = self.raw.unmap(urid);
            match uri {
                Some(uri) => {
                    let uri = CString::from(uri);
                    self.cache.insert(urid, uri);
                }
                None => return None,
            }
        }
        Some(self.cache.get(&urid).unwrap())
    }
}
