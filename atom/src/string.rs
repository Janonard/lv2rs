//! ASCII string.
//!
//! This module contains the [`AtomString`](type.AtomString.html), an atom representing standard
//! ASCII strings.
//!
//! Atom strings can only be written once: The `write_atom` call expects a CStr from which it can
//! copy the data and after that call, the string can't be modified.
//!
//! This module contains also a special method for `Atom<AtomString>`:
//! [`as_cstr`](../atom/struct.Atom.html#method.as_cstr). It let's you access the string quickly!
//!
//! An example:
//!
//!     extern crate lv2rs_atom as atom;
//!     extern crate lv2rs_urid as urid;
//!
//!     use atom::prelude::*;
//!     use atom::ports::*;
//!     use atom::atom::*;
//!     use urid::{CachedMap, debug::DebugMap};
//!     use std::ffi::CStr;
//!
//!     pub struct Plugin {
//!         in_port: AtomInputPort<AtomString>,
//!         out_port: AtomOutputPort<AtomString>,
//!         urids: CachedMap,
//!     }
//!
//!     impl Plugin {
//!         /// Simulated `run` method.
//!         fn run(&mut self) {
//!             let message: &str = "Hello World!\0";
//!             let c_message = CStr::from_bytes_with_nul(message.as_bytes()).unwrap();
//!
//!             // Writing.
//!             unsafe { self.out_port.write_atom(c_message, &mut self.urids).unwrap() };
//!
//!             // Reading.
//!             let atom = unsafe { self.in_port.get_atom(&mut self.urids) }.unwrap();
//!             let str = atom.as_cstr().unwrap().to_str().unwrap();
//!             assert_eq!("Hello World!", str);
//!         }
//!     }
//!
//!     // Getting a debug URID map.
//!     let mut debug_map = DebugMap::new();
//!     let mut urids = unsafe {debug_map.create_cached_map()};
//!
//!     // Creating the plugin.
//!     let mut plugin = Plugin {
//!         in_port: AtomInputPort::new(&mut urids),
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
use crate::atom::{array::*, *};
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
/// ASCII String.
///
/// See the [module documentation](index.html) for more information.
pub type AtomString = ArrayAtomBody<(), i8>;

impl AtomBody for AtomString {
    type InitializationParameter = CStr;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::STRING_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        string: &CStr,
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, &(), urids)?;

        writer.write_raw(string.to_bytes())?;
        // Write the null terminator since `string.to_bytes()` will never contain one.
        writer.write_sized(&0u8)?;

        Ok(())
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, WidenRefError> {
        Self::__widen_ref(header, urids)
    }
}

impl Atom<AtomString> {
    /// Try to wrap the string into a `CStr` reference.
    ///
    /// This function returns an error if the internal conversion fails.
    pub fn as_cstr(&self) -> Result<&CStr, std::ffi::FromBytesWithNulError> {
        CStr::from_bytes_with_nul(unsafe { std::mem::transmute::<&[i8], &[u8]>(&self.body.data) })
    }
}
