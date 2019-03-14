use crate::atom::AtomBody;
use crate::frame::{CoreWriter, Writer};
use crate::uris;
use std::ffi::CStr;
use std::mem::size_of_val;
use std::os::raw::*;
use urid::URID;

impl AtomBody for CStr {
    type InitializationParameter = ();

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::STRING_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.string
    }

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        string: &(),
    ) -> Result<(), ()> {
        Ok(())
    }
}

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

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        language: &URID,
    ) -> Result<(), ()> {
        let header = LiteralHeader {
            datatype: 0,
            lang: *language,
        };
        writer.write_sized(&header, true)?;
        Ok(())
    }
}
