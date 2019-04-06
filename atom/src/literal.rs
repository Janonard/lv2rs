//! UTF-8-encoded string.
//!
//! This string atom corresponds to Rust's normal `str` and `String` types, since it is
//! UTF-8-encoded. A literal also contains, apart from the string, the URID of it's language.
//!
//! When initialized, a literal does not contain any text. Every text has to be appended to the
//! literal using the [`LiteralWritingFrame`](trait.LiteralWritingFrame.html) trait. Every
//! writing frame implements this trait via a blanket implementation and the trait is included in
//! the crate's prelude. You can, therefore, act as if the extended methods were normal methods of a
//! writing frame.
//!
//! You can aquire a literal's data using the [`lang` method](type.Literal.html#method.lang)
//! and the [`as_str` method](type.Literal.html#method.as_str).
//!
//! An example:
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
//!         in_port: AtomInputPort<Literal>,
//!         out_port: AtomOutputPort<Literal>,
//!         urids: CachedMap,
//!     }
//!
//!     impl Plugin {
//!         /// Simulated `run` method.
//!         fn run(&mut self) {
//!             // Writing
//!             {
//!                 let mut frame =
//!                     unsafe { self.out_port.write_atom_body(&0, &mut self.urids) }.unwrap();
//!                 frame.append_string("Hello World!");
//!             }
//!
//!             // Reading.
//!             let literal = unsafe { self.in_port.get_atom_body(&mut self.urids) }.unwrap();
//!             let message = literal.as_str().unwrap();
//!             assert_eq!("Hello World!", message);
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
use crate::atom::{array::*, *};
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use urid::URID;

/// The body header of a literal.
///
/// It contains the URID of the datatype (some obscure RDF feature) and of the language. It is also
/// `repr(C)` and is used to interpret raw atoms.
#[repr(C)]
pub struct LiteralHeader {
    pub datatype: URID,
    pub lang: URID,
}

/// UTF-8 encoded string.
///
/// See the [module documentation](index.html) for more information.
pub type Literal = ArrayAtomBody<LiteralHeader, u8>;

impl ArrayAtomHeader for LiteralHeader {
    type InitializationParameter = URID;

    unsafe fn initialize<'a, W, T>(
        writer: &mut W,
        language: &URID,
        _urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
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

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LITERAL_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        language: &URID,
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, language, urids)
    }

    fn create_ref<'a>(raw_data: &'a [u8]) -> Result<&'a Self, ()> {
        Self::__create_ref(raw_data)
    }
}

impl Literal {
    /// Try to parse the literal data as a `&str`
    ///
    /// Parsing errors are forwarded.
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let bytes = &self.data;
        std::str::from_utf8(bytes)
    }

    /// Return the language of the literal.
    pub fn lang(&self) -> URID {
        self.header.lang
    }
}

pub trait LiteralWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Literal> {
    /// Append a string to the literal.
    ///
    /// In case of insufficient memory, `Err` is returned.
    fn append_string(&mut self, string: &str) -> Result<(), ()> {
        unsafe { Literal::append(self, string.as_bytes()) }
    }
}

impl<'a, W> LiteralWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Literal> {}
