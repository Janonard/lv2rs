/*
use crate::atom::{ArrayAtomBody, Atom, AtomBody};
use crate::frame::WritingFrame;
use crate::uris;
use std::ffi::CStr;
use std::os::raw::*;
use urid::URID;

#[repr(C)]
pub struct ObjectHeader {
    id: c_uint,
    otype: c_uint,
}

type Object = ArrayAtomBody<ObjectHeader, u8>;

impl AtomBody for Object {
    type ConstructionParameter = ObjectHeader;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::OBJECT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.object
    }

    fn write_body<F: WritingFrame>(
        frame: &mut F,
        parameter: ObjectHeader,
    ) -> Result<&mut Self, ()> {
        Self::__write_body(frame, parameter)
    }
}

#[repr(C)]
pub struct PropertyHeader {
    key: c_uint,
    context: c_uint,
}

#[repr(C)]
pub struct Property<A: AtomBody + Clone + ?Sized> {
    header: PropertyHeader,
    value: Atom<A>,
}

impl<A: AtomBody + Clone + ?Sized> AtomBody for Property<A> {
    type ConstructionParameter = PropertyHeader;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::PROPERTY_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.property
    }

    fn write_body<F: WritingFrame>(frame: &mut F, header: PropertyHeader) -> Result<&mut Self, ()> {
        let self_ptr: *mut PropertyHeader = frame.write_sized(&header, false)?.0;
    }
}
*/
