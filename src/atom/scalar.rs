use crate::atom::AtomBody;
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
pub use std::os::raw::c_int;

pub trait ScalarAtomBody {
    fn get_uri() -> &'static CStr;
    fn get_urid(urids: &uris::MappedURIDs) -> URID;
}

impl<T> AtomBody for T
where
    T: 'static + ScalarAtomBody,
{
    type InitializationParameter = Self;

    fn get_uri() -> &'static CStr {
        T::get_uri()
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        T::get_urid(urids)
    }

    fn initialize_body<'a, W>(writer: &mut W, parameter: &Self) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        unsafe { writer.write_sized(parameter, true)? };
        Ok(())
    }
}

impl ScalarAtomBody for c_int {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::INT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.int
    }
}

pub use std::os::raw::c_long;

impl ScalarAtomBody for c_long {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LONG_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.long
    }
}

pub use std::os::raw::c_float;

impl ScalarAtomBody for c_float {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::FLOAT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.float
    }
}

pub use std::os::raw::c_double;

impl ScalarAtomBody for c_double {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::DOUBLE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.double
    }
}

pub use urid::URID;

impl ScalarAtomBody for URID {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::URID_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.urid
    }
}

impl ScalarAtomBody for bool {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::BOOL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.bool
    }
}
