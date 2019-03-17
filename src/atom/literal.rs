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

pub enum LiteralWritingError {
    InsufficientSpace,
    NotFirstCall,
}

pub trait LiteralWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Literal> {
    fn write_string(&mut self, string: &str) -> Result<(), LiteralWritingError> {
        if self.get_header().size as usize > std::mem::size_of::<LiteralHeader>() {
            return Err(LiteralWritingError::NotFirstCall);
        }

        let bytes = string.as_bytes();
        match unsafe { self.write_raw(bytes, false) } {
            Ok(_) => (),
            Err(_) => return Err(LiteralWritingError::InsufficientSpace),
        }

        let termination_successfull = match bytes.last() {
            Some(byte) => {
                if *byte != 0 {
                    unsafe { self.write_sized(&0u8, true) }.is_ok()
                } else {
                    unsafe { self.write_sized(&(), true) }.is_ok()
                }
            }
            None => unsafe { self.write_sized(&0u8, true) }.is_ok(),
        };

        if termination_successfull {
            Ok(())
        } else {
            Err(LiteralWritingError::InsufficientSpace)
        }
    }
}

impl<'a, W> LiteralWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Literal> {}
