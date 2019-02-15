use std::os::raw::*;

pub type MapHandle = *mut c_void;
pub type UnmapHandle = *mut c_void;
pub type URID = u32;

#[repr(C)]
pub struct Map {
    pub handle: MapHandle,
    pub map: extern "C" fn(handle: MapHandle, uri: *const c_char) -> URID,
}

#[repr(C)]
pub struct Unmap {
    pub handle: MapHandle,
    pub unmap: extern "C" fn(handle: UnmapHandle, urid: URID) -> *const c_char,
}
