use crate::atom::{Atom, AtomBody};
use crate::uris;
use std::ffi::CStr;
use std::os::raw::*;
use urid::URID;

#[repr(C)]
pub struct Object {
    id: c_uint,
    otype: c_uint,
    data: [u8],
}

impl AtomBody for Object {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::OBJECT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.object
    }
}

#[repr(C)]
pub struct Property<A: AtomBody + Clone + ?Sized> {
    key: c_uint,
    context: c_uint,
    value: Atom<A>,
}

impl<A: AtomBody + Clone + ?Sized> AtomBody for Property<A> {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::PROPERTY_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.property
    }
}
