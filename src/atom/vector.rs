use crate::atom::AtomBody;
use crate::frame::{CoreWriter, Writer};
use crate::uris::{MappedURIDs, VECTOR_TYPE_URI};
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
pub struct Vector<T: AtomBody + Sized> {
    header: VectorHeader,
    data: [T],
}

impl<T: AtomBody + Sized> AtomBody for Vector<T> {
    type InitializationParameter = MappedURIDs;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(VECTOR_TYPE_URI) }
    }

    fn get_urid(urids: &MappedURIDs) -> URID {
        urids.vector
    }

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        urids: &MappedURIDs,
    ) -> Result<(), ()> {
        let header = VectorHeader {
            child_size: size_of::<T>() as u32,
            // TODO: URID einfügen!
            child_type: T::get_urid(urids),
        };
        writer.write_sized(&header, true)?;
        Ok(())
    }
}
