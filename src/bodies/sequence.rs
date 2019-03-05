use crate::atom::{Atom, AtomBody};
use crate::uris;
use std::ffi::CStr;
use std::os::raw::*;
use urid::URID;

#[repr(C)]
pub struct Sequence {
    unit: c_uint,
    pad: c_uint,
    data: [u8],
}

impl AtomBody for Sequence {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::SEQUENCE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.sequence
    }
}

#[repr(C)]
union Time {
    frames: c_long,
    beats: c_double,
}

#[repr(C)]
struct Event<A: AtomBody + Clone + ?Sized> {
    time: Time,
    body: Atom<A>,
}
