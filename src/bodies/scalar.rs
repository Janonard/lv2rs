use crate::atom::AtomBody;
use crate::frame::WritingFrame;
use crate::uris;
use std::ffi::CStr;

pub use std::os::raw::c_int;

impl AtomBody for c_int {
    type ConstructionParameter = i32;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::INT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.int
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, value: &i32) -> Result<&'a mut Self, ()> {
        let object = frame.write_sized(value, true)?;
        Ok(object.0)
    }
}

pub use std::os::raw::c_long;

impl AtomBody for c_long {
    type ConstructionParameter = i64;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LONG_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.long
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, value: &i64) -> Result<&'a mut Self, ()> {
        let object = frame.write_sized(value, true)?;
        Ok(object.0)
    }
}

pub use std::os::raw::c_float;

impl AtomBody for c_float {
    type ConstructionParameter = f32;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::FLOAT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.float
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, value: &f32) -> Result<&'a mut Self, ()> {
        let object = frame.write_sized(value, true)?;
        Ok(object.0)
    }
}

pub use std::os::raw::c_double;

impl AtomBody for c_double {
    type ConstructionParameter = f64;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::DOUBLE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.double
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, value: &f64) -> Result<&'a mut Self, ()> {
        let object = frame.write_sized(value, true)?;
        Ok(object.0)
    }
}

pub use urid::URID;

impl AtomBody for URID {
    type ConstructionParameter = URID;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::URID_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.urid
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, value: &URID) -> Result<&'a mut Self, ()> {
        let object = frame.write_sized(value, true)?;
        Ok(object.0)
    }
}

impl AtomBody for bool {
    type ConstructionParameter = bool;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::BOOL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.bool
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, value: &bool) -> Result<&'a mut Self, ()> {
        let object = frame.write_sized(value, true)?;
        Ok(object.0)
    }
}
