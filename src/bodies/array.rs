use crate::atom::AtomBody;
use crate::uris;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::os::raw::*;
use urid::URID;

impl AtomBody for CStr {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::STRING_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.string
    }
}

#[repr(C)]
pub struct Literal {
    datatype: c_uint,
    lang: c_uint,
    data: str,
}

impl AtomBody for Literal {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LITERAL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.literal
    }
}

impl Deref for Literal {
    type Target = str;
    fn deref(&self) -> &str {
        &self.data
    }
}

impl DerefMut for Literal {
    fn deref_mut(&mut self) -> &mut str {
        &mut self.data
    }
}

#[repr(C)]
pub struct Vector<T: AtomBody> {
    child_size: c_uint,
    child_type: c_uint,
    data: [T],
}

impl<T: AtomBody> AtomBody for Vector<T> {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::VECTOR_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.vector
    }
}

impl<T: AtomBody> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.data
    }
}

impl<T: AtomBody> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
