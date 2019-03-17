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
        let bytes = string.to_bytes();
        unsafe { writer.write_raw(bytes, false) }?;

        // Write the string terminator, if not included in the string.
        match bytes.last() {
            Some(byte) => {
                if *byte != 0 {
                    unsafe { writer.write_sized(&0u8, true) }?;
                }
            }
            None => {
                unsafe { writer.write_sized(&0u8, true) }?;
            }
        }
        Ok(())
    }
}
