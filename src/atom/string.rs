use crate::atom::array::ArrayAtomBody;
use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use urid::URID;

pub type AtomString = ArrayAtomBody<(), i8>;

impl AtomBody for AtomString {
    type InitializationParameter = ();

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::STRING_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.string
    }

    fn initialize_body<'a, W>(writer: &mut W, parameter: &()) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, parameter)
    }

    unsafe fn widen_ref(header: &AtomHeader) -> Result<&Atom<Self>, ()> {
        Self::__widen_ref(header)
    }
}

impl Atom<AtomString> {
    pub fn as_cstr(&self) -> Result<&CStr, std::ffi::FromBytesWithNulError> {
        CStr::from_bytes_with_nul(unsafe { std::mem::transmute::<&[i8], &[u8]>(&self.body.data) })
    }
}

pub enum AtomStringWritingError {
    InsufficientSpace,
    NotFirstCall,
}

pub trait AtomStringWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, AtomString> {
    fn write_string(&mut self, string: &CStr) -> Result<(), AtomStringWritingError> {
        if AtomString::was_data_written(self) {
            return Err(AtomStringWritingError::NotFirstCall);
        }

        AtomString::append(self, unsafe {
            std::mem::transmute::<&[u8], &[i8]>(string.to_bytes())
        })
        .map_err(|_| AtomStringWritingError::InsufficientSpace)?;

        // Write the null terminator, as `string.as_bytes()` will never contain one.
        AtomString::push(self, 0).map_err(|_| AtomStringWritingError::InsufficientSpace)
    }
}

impl<'a, W> AtomStringWritingFrame<'a> for W where
    W: WritingFrame<'a> + WritingFrameExt<'a, AtomString>
{
}
