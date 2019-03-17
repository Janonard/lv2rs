use crate::atom::AtomBody;
use crate::frame::{WritingFrame, WritingFrameExt};
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

#[repr(C)]
pub struct Vector<T>
where
    T: 'static + AtomBody + Sized + Copy,
{
    header: VectorHeader,
    data: [T],
}

impl<T> AtomBody for Vector<T>
where
    T: 'static + AtomBody + Sized + Copy,
{
    type InitializationParameter = uris::MappedURIDs;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::VECTOR_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.vector
    }

    fn initialize_body<'a, W>(writer: &mut W, urids: &uris::MappedURIDs) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        let header = VectorHeader {
            child_size: size_of::<T>() as u32,
            child_type: T::get_urid(urids),
        };
        unsafe { writer.write_sized(&header, true)? };
        Ok(())
    }
}

pub trait VectorWritingFrame<'a, T>
where
    T: 'static + AtomBody + Sized + Copy,
    Self: WritingFrame<'a> + WritingFrameExt<'a, Vector<T>>,
{
    fn push(&mut self, value: T) -> Result<(), ()> {
        unsafe { self.write_sized(&value, false)? };
        Ok(())
    }

    fn append(&mut self, slice: &[T]) -> Result<(), ()> {
        let data = unsafe {
            std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice))
        };
        unsafe { self.write_raw(data, false)? };
        Ok(())
    }
}

impl<'a, T, F> VectorWritingFrame<'a, T> for F
where
    T: 'static + AtomBody + Sized + Copy,
    F: WritingFrame<'a> + WritingFrameExt<'a, Vector<T>>,
{
}
