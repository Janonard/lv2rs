use crate::atom::AtomBody;
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use std::os::raw::*;
use urid::URID;

#[repr(C)]
pub struct LiteralHeader {
    datatype: c_uint,
    lang: c_uint,
}

#[repr(C)]
pub struct Literal {
    header: LiteralHeader,
    string: [u8],
}

impl AtomBody for Literal {
    type InitializationParameter = URID;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LITERAL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.literal
    }

    fn initialize_body<'a, W>(writer: &mut W, language: &URID) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        let header = LiteralHeader {
            datatype: 0,
            lang: *language,
        };
        unsafe { writer.write_sized(&header, true)? };
        Ok(())
    }
}

pub trait LiteralWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Literal> {
    fn append(&mut self, string: &str) -> Result<(), ()> {
        unsafe { self.write_raw(string.as_bytes(), false) }?;
        Ok(())
    }
}

impl<'a, W> LiteralWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Literal> {}
