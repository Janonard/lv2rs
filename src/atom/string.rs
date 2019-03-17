use crate::atom::AtomBody;
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use std::os::raw::*;
use urid::URID;

pub type AtomString = [c_char];

impl AtomBody for AtomString {
    type InitializationParameter = CStr;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::STRING_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.string
    }

    fn initialize_body<'a, W>(writer: &mut W, string: &CStr) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        unsafe { writer.write_raw(string.to_bytes(), false) }?;
        Ok(())
    }
}

pub trait AtomStringWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, AtomString> {
    fn append(&mut self, string: &CStr) -> Result<(), ()> {
        unsafe { self.write_raw(string.to_bytes(), false) }?;
        Ok(())
    }
}

impl<'a, W> AtomStringWritingFrame<'a> for W where
    W: WritingFrame<'a> + WritingFrameExt<'a, AtomString>
{
}
