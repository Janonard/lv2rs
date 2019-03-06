use crate::atom::{ArrayAtomBody, AtomBody};
use crate::frame::WritingFrame;
use crate::uris;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::*;
use urid::URID;

#[repr(C)]
pub struct VectorHeader {
    child_size: c_uint,
    child_type: c_uint,
}

pub type Vector<T> = ArrayAtomBody<VectorHeader, T>;

impl<T: AtomBody + Default> AtomBody for Vector<T> {
    type ConstructionParameter = ();

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::VECTOR_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.vector
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, _: &()) -> Result<&'a mut Self, ()> {
        let header = VectorHeader {
            child_size: size_of::<T>() as u32,
            // TODO: URID einfügen!
            child_type: 0,
        };
        Self::__write_body(frame, &header, &[T::default(); 0])
    }
}
