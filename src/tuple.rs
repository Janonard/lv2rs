//! Heterogenous array of sized and unsized atoms.
//!
//! A tuple is a pretty simple collection of different atoms: It basically a chunk containing an
//! arbitrary amount of atoms, aligned to 64-bit.
//!
//! When initialized, a tuple does not contain any atoms. These have to be pushed to the tuple using
//! the [`TupleWritingFrame`](trait.TupleWritingFrame.html) trait. Every
//! writing frame implements this trait via a blanket implementation and the trait is included in
//! the crate's prelude. You can, therefore, act as if the extended method were normal methods of a
//! writing frame.
//!
//! Reading atoms is done by iterating through all atoms one by one. Iterators are produced by the
//! [`iter`](../atom/struct.Atom.html#method.iter-2) method.
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
//!         in_port: AtomInputPort<Tuple>,
//!         out_port: AtomOutputPort<Tuple>,
//!         urids: CachedMap,
//!     }
//!
//!     impl Plugin {
//!         /// Simulated `run` method.
//!         fn run(&mut self) {
//!             // Writing
//!             {
//!                 let mut frame =
//!                     unsafe { self.out_port.write_atom(&(), &mut self.urids) }.unwrap();
//!                 frame.push_atom::<i32>(&42, &mut self.urids).unwrap();
//!                 frame.push_atom::<f32>(&17.0, &mut self.urids).unwrap();
//!             }
//!
//!             let i32_urid = self.urids.map(<i32 as AtomBody>::get_uri());
//!             let f32_urid = self.urids.map(<f32 as AtomBody>::get_uri());
//!
//!             // Reading.
//!             let atom = unsafe { self.in_port.get_atom(&mut self.urids) }.unwrap();
//!             for sub_atom in atom.iter() {
//!                 if sub_atom.body_type() == i32_urid {
//!                     let integer = sub_atom.cast::<i32>(&mut self.urids).unwrap();
//!                     assert_eq!(42, **integer);
//!                 } else if sub_atom.body_type() == f32_urid {
//!                     let float = sub_atom.cast::<f32>(&mut self.urids).unwrap();
//!                     assert_eq!(17.0, **float);
//!                 } else {
//!                     panic!("Unknown property in object!");
//!                 }
//!             }
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
use crate::atom::{array::ArrayAtomBody, Atom, AtomBody, AtomHeader};
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::unknown::*;
use crate::uris;
use std::ffi::CStr;

/// Heterogenous array of sized and unsized atoms.
///
/// See the [module documentation](index.html) for more information.
pub type Tuple = ArrayAtomBody<(), u8>;

impl AtomBody for Tuple {
    type InitializationParameter = ();

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::TUPLE_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        parameter: &(),
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, parameter, urids)
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, ()> {
        Self::__widen_ref(header, urids)
    }
}

impl Atom<Tuple> {
    /// Create an iterator over all properties of the object.
    ///
    /// This iterator is based on the [`ChunkIterator`](../unknown/struct.ChunkIterator.html).
    pub fn iter(&self) -> impl Iterator<Item = &Atom<Unknown>> {
        ChunkIterator::<()>::new(&self.body.data).map(|(_, chunk): (&(), &Atom<Unknown>)| chunk)
    }
}

/// Extension for [`WritingFrame`](../frame/trait.WritingFrame.html) and
/// [`WritingFrameExt`](../frame/trait.WritingFrameExt.html) for vectors.
///
/// See the [module documentation](index.html) for more information.
pub trait TupleWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Tuple> {
    /// Add a new atom to the tuple.
    ///
    /// This method acts just like an output port's
    /// [`write_atom`](../ports/struct.AtomOutputPort.html#method.write_atom): It receives the
    /// initialization parameter of a atom, creates a new writing frame, initializes the atom and
    /// returns the frame.
    fn push_atom<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        parameter: &A::InitializationParameter,
        urids: &mut urid::CachedMap,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        unsafe {
            let mut frame = self.create_nested_frame::<A>(urids)?;
            A::initialize_body(&mut frame, parameter, urids)?;
            Ok(frame)
        }
    }
}

impl<'a, W> TupleWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Tuple> {}
