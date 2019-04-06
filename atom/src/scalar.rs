//! Scalar (number-like) atoms
//!
//! There are several scalar atoms:
//! * `i32`
//! * `i64`
//! * `f32`
//! * `f64`
//! * `bool`
//! * `URID`
//!
//! They all have in common that they are statically sized (which is something special among atoms)
//! and that they can be written in one piece; Once they are initialized, they are completed and
//! need no further amendments. Therefore, their behaviour is abstracted to another trait,
//! [`ScalarAtomBody`](trait.ScalarAtomBody.html), which features a standard implementation of
//! [`AtomBody`](../atom/trait.AtomBody.html) for every type that implements it. Therefore, writing
//! and reading scalar atoms is pretty straight foreward:
//!
//!     extern crate lv2rs_atom as atom;
//!     extern crate lv2rs_urid as urid;
//!
//!     use atom::prelude::*;
//!     use atom::ports::*;
//!     use urid::{CachedMap, debug::DebugMap};
//!     use std::ffi::CStr;
//!
//!     pub struct Plugin {
//!         in_port: AtomInputPort<f32>,
//!         out_port: AtomOutputPort<f32>,
//!         urids: CachedMap,
//!     }
//!
//!     impl Plugin {
//!         /// Simulated `run` method.
//!         fn run(&mut self) {
//!             // Writing.
//!             unsafe { self.out_port.write_atom_body(&42.0f32, &mut self.urids) }.unwrap();
//!
//!             // Reading.
//!             let float = unsafe { self.in_port.get_atom_body(&mut self.urids) }.unwrap();
//!             assert_eq!(42.0, *float);
//!         }
//!     }
//!
//!     // Getting a debug URID map.
//!     let mut debug_map = DebugMap::new();
//!     let mut urids = unsafe {debug_map.create_cached_map()};
//!
//!     // Creating the plugin.
//!     let mut plugin = Plugin {
//!         in_port: AtomInputPort::new(),
//!         out_port: AtomOutputPort::new(),
//!         urids: urids,
//!     };
//!
//!     // Creating the atom space.
//!     let mut atom_space = vec![0u8; 256];
//!     let atom = unsafe { (atom_space.as_mut_ptr() as *mut Atom).as_mut() }.unwrap();
//!     *(atom.mut_size()) = 256 - 8;
//!
//!     // Connecting the ports.
//!     plugin.in_port.connect_port(atom as &Atom);
//!     plugin.out_port.connect_port(atom);
//!
//!     // Calling `run`.
//!     plugin.run();
use crate::atom::*;
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;

/// Abstraction over scalar (number-like) atoms.
///
/// See the [module documentation](index.html) for more information.
pub trait ScalarAtomBody {
    fn get_uri() -> &'static CStr;
}

impl<T> AtomBody for T
where
    T: 'static + Sized + ScalarAtomBody,
{
    type InitializationParameter = Self;

    fn get_uri() -> &'static CStr {
        T::get_uri()
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        parameter: &Self,
        _urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        writer.write_sized(parameter)?;
        Ok(())
    }

    fn create_ref<'a>(raw_body: &'a [u8]) -> Result<&'a Self, ()> {
        if raw_body.len() == std::mem::size_of::<Self>() {
            let ptr = raw_body.as_ptr() as *const Self;
            Ok(unsafe { ptr.as_ref() }.unwrap())
        } else {
            Err(())
        }
    }
}

impl ScalarAtomBody for i32 {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::INT_TYPE_URI) }
    }
}

impl ScalarAtomBody for i64 {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LONG_TYPE_URI) }
    }
}

impl ScalarAtomBody for f32 {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::FLOAT_TYPE_URI) }
    }
}

impl ScalarAtomBody for f64 {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::DOUBLE_TYPE_URI) }
    }
}

pub use urid::URID;

impl ScalarAtomBody for URID {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::URID_TYPE_URI) }
    }
}

impl ScalarAtomBody for bool {
    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::BOOL_TYPE_URI) }
    }
}
