use crate::atom::AtomBody;
use crate::frame::{CoreWriter, Writer};
use crate::uris;
use std::ffi::CStr;

pub use std::os::raw::c_int;

impl AtomBody for c_int {
    type InitializationParameter = i32;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::INT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.int
    }

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        parameter: &i32,
    ) -> Result<(), ()> {
        writer.write_sized(parameter, true)?;
        Ok(())
    }
}

pub use std::os::raw::c_long;

impl AtomBody for c_long {
    type InitializationParameter = i64;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LONG_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.long
    }

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        parameter: &i64,
    ) -> Result<(), ()> {
        writer.write_sized(parameter, true)?;
        Ok(())
    }
}

pub use std::os::raw::c_float;

impl AtomBody for c_float {
    type InitializationParameter = f32;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::FLOAT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.float
    }

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        parameter: &f32,
    ) -> Result<(), ()> {
        writer.write_sized(parameter, true)?;
        Ok(())
    }
}

pub use std::os::raw::c_double;

impl AtomBody for c_double {
    type InitializationParameter = f64;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::DOUBLE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.double
    }

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        parameter: &f64,
    ) -> Result<(), ()> {
        writer.write_sized(parameter, true)?;
        Ok(())
    }
}

pub use urid::URID;

impl AtomBody for URID {
    type InitializationParameter = URID;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::URID_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.urid
    }

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        parameter: &URID,
    ) -> Result<(), ()> {
        writer.write_sized(parameter, true)?;
        Ok(())
    }
}

impl AtomBody for bool {
    type InitializationParameter = bool;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::BOOL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.bool
    }

    fn initialize_body<'a, W: Writer<'a> + CoreWriter<'a>>(
        writer: &mut W,
        parameter: &bool,
    ) -> Result<(), ()> {
        writer.write_sized(parameter, true)?;
        Ok(())
    }
}
