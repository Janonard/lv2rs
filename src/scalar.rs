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
//!     extern crate lv2rs_atom;
//!     use lv2rs_atom::prelude::*;
//!     use lv2rs_atom::uris::MappedURIDs;
//!     use lv2rs_atom::ports::*;
//!
//!     pub struct Plugin {
//!         in_port: AtomInputPort<f32>,
//!         out_port: AtomOutputPort<f32>,
//!         urids: &'static MappedURIDs,
//!     }
//!
//!     impl Plugin {
//!         /// Simulated `run` method.
//!         fn run(&mut self) {
//!             // Writing.
//!             self.out_port.write_atom(&42.0f32, self.urids).unwrap();
//!
//!             // Reading.
//!             let atom = self.in_port.get_atom(self.urids).unwrap();
//!             assert_eq!(4, atom.body_size());
//!             assert_eq!(self.urids.float, atom.body_type());
//!             assert_eq!(42.0, **atom);
//!         }
//!     }
//!
//!     // Getting the default URID map.
//!     let urids = unsafe {MappedURIDs::get_map()};
//!
//!     // Creating the plugin.
//!     let mut plugin = Plugin {
//!         in_port: AtomInputPort::new(urids),
//!         out_port: AtomOutputPort::new(),
//!         urids: urids,
//!     };
//!
//!     // Creating the atom space.
//!     let mut atom_space = vec![0u8; 256];
//!     let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
//!     atom.size = 256 - 8;
//!
//!     // Connecting the ports.
//!     plugin.in_port.connect_port(atom as &AtomHeader);
//!     plugin.out_port.connect_port(atom);
//!
//!     // Calling `run`.
//!     plugin.run();
use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;

/// Abstraction over scalar (number-like) atoms.
///
/// See the [module documentation](index.html) for more information.
pub trait ScalarAtomBody {
    fn get_uri() -> &'static CStr;
    fn get_urid(urids: &uris::MappedURIDs) -> URID;
}

impl<T> AtomBody for T
where
    T: 'static + Sized + ScalarAtomBody,
{
    type InitializationParameter = Self;

    type MappedURIDs = uris::MappedURIDs;

    fn get_uri() -> &'static CStr {
        T::get_uri()
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        T::get_urid(urids)
    }

    unsafe fn initialize_body<'a, W>(writer: &mut W, parameter: &Self) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        writer.write_sized(parameter)?;
        Ok(())
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &uris::MappedURIDs,
    ) -> Result<&'a Atom<Self>, ()> {
        if header.atom_type == T::get_urid(urids)
            && header.size as usize == std::mem::size_of::<Self>()
        {
            Ok((header as *const AtomHeader as *const Atom<Self>)
                .as_ref()
                .unwrap())
        } else {
            Err(())
        }
    }
}

pub use std::os::raw::c_int;

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
