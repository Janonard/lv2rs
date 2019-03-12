use crate::atom::{ArrayAtomBody, AtomBody};
use crate::frame::WritingFrame;
use crate::uris;
use std::ffi::CStr;
use std::os::raw::*;
use urid::URID;

pub type String = ArrayAtomBody<(), u8>;

impl AtomBody for String {
    type ConstructionParameter = CStr;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::STRING_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.string
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, data: &CStr) -> Result<&'a mut Self, ()> {
        Self::__write_body(frame, &(), data.to_bytes())
    }
}

#[repr(C)]
pub struct LiteralHeader {
    datatype: c_uint,
    lang: c_uint,
}

pub type Literal = ArrayAtomBody<LiteralHeader, u8>;

impl AtomBody for Literal {
    type ConstructionParameter = LiteralHeader;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LITERAL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.literal
    }

    fn write_body<'a, F: WritingFrame>(
        frame: &'a mut F,
        header: &LiteralHeader,
    ) -> Result<&'a mut Self, ()> {
        Self::__write_body(frame, header, &[0; 0])
    }
}
