use crate::atom::{
    array::{ArrayAtomBody, ArrayAtomHeader},
    Atom, AtomBody, AtomHeader,
};
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use std::os::raw::*;
use urid::URID;

#[repr(C)]
pub struct LiteralHeader {
    pub datatype: c_uint,
    pub lang: c_uint,
}

pub type Literal = ArrayAtomBody<LiteralHeader, u8>;

impl ArrayAtomHeader for LiteralHeader {
    type InitializationParameter = URID;

    unsafe fn initialize<'a, W, T>(writer: &mut W, language: &URID) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
    {
        let header = LiteralHeader {
            datatype: 0,
            lang: *language,
        };
        writer.write_sized(&header)?;
        Ok(())
    }
}

impl AtomBody for Literal {
    type InitializationParameter = URID;

    type MappedURIDs = uris::MappedURIDs;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LITERAL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.literal
    }

    unsafe fn initialize_body<'a, W>(writer: &mut W, language: &URID) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, language)
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &uris::MappedURIDs,
    ) -> Result<&'a Atom<Self>, ()> {
        Self::__widen_ref(header, urids)
    }
}

impl Atom<Literal> {
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let bytes = &self.body.data;
        let bytes = &bytes[..bytes.len() - 1];
        std::str::from_utf8(bytes)
    }

    pub fn lang(&self) -> URID {
        self.body.header.lang
    }
}

#[derive(Debug)]
pub enum LiteralWritingError {
    InsufficientSpace,
    NotFirstCall,
}

pub trait LiteralWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Literal> {
    fn write_string(&mut self, string: &str) -> Result<(), LiteralWritingError> {
        if Literal::was_data_written(self) {
            return Err(LiteralWritingError::NotFirstCall);
        }

        unsafe { Literal::append(self, string.as_bytes()) }
            .map_err(|_| LiteralWritingError::InsufficientSpace)?;

        // Write the null terminator, as `string.as_bytes()` will never contain one.
        unsafe { Literal::push(self, 0) }.map_err(|_| LiteralWritingError::InsufficientSpace)
    }
}

impl<'a, W> LiteralWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Literal> {}
