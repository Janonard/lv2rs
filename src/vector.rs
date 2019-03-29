//! Homogenous array of sized atoms.
//!
//! A [vector](type.Vector.html) is the LV2 equivalent of a slice: It has a variable length, but it
//! does only contain one type of item, which has to be sized.
//!
//! When initialized, a vector does not contain any items. These items have to be pushed or appended
//! to the vector using the [`VectorWritingFrame`](trait.VectorWritingFrame.html) trait. Every
//! writing frame implements this trait via a blanket implementation and the trait is included in
//! the crate's prelude. You can, therefore, act as if the extended method were normal methods of a
//! writing frame.
//!
//! Reading the vector is done using special methods for the [`Atom`](../atom/struct.Atom.html)
//! struct:
//! * [`child_body_size`](../atom/struct.Atom.html#method.child_body_size)
//! * [`child_body_type`](../atom/struct.Atom.html#method.child_body_type)
//! * [`as_slice`](../atom/struct.Atom.html#method.as_slice)
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
//!         in_port: AtomInputPort<Vector<f32>>,
//!         out_port: AtomOutputPort<Vector<f32>>,
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
//!                 frame.push(0.0).unwrap();
//!                 frame.append(&[1.0, 2.0, 3.0, 4.0]).unwrap();
//!             }
//!
//!             // Reading.
//!             let atom = unsafe { self.in_port.get_atom(&mut self.urids) }.unwrap();
//!             let data = atom.as_slice();
//!             assert_eq!([0.0, 1.0, 2.0, 3.0, 4.0], data);
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
use crate::atom::array::{ArrayAtomBody, ArrayAtomHeader};
use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::*;
use urid::URID;

/// The body header of a vector.
///
/// It contains the size of the child type (which has to be static) and the child type itself.
/// This struct is also `repr(C)` and is used to interpret raw atom data.
#[repr(C)]
pub struct VectorHeader {
    pub child_size: c_uint,
    pub child_type: c_uint,
}

/// A homogenous array of sized atoms.
///
/// See the [module documentation](index.html) for more information.
pub type Vector<T> = ArrayAtomBody<VectorHeader, T>;

impl ArrayAtomHeader for VectorHeader {
    type InitializationParameter = URID;

    unsafe fn initialize<'a, W, T>(
        writer: &mut W,
        child_type: &URID,
        _urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
    {
        let header = VectorHeader {
            child_size: size_of::<T>() as u32,
            child_type: *child_type,
        };
        writer.write_sized(&header)?;
        Ok(())
    }
}

impl<T> AtomBody for Vector<T>
where
    T: 'static + AtomBody + Sized + Copy,
{
    type InitializationParameter = ();

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::VECTOR_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        _: &(),
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, &urids.map(T::get_uri()), urids)
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, ()> {
        Self::__widen_ref(header, urids)
    }
}

impl<T> Atom<Vector<T>>
where
    T: 'static + AtomBody + Sized + Copy,
{
    /// Return the size of the child type, according to the vector's body header.
    pub fn child_body_size(&self) -> usize {
        self.body.header.child_size as usize
    }

    /// Return the type of the child, according to the vector's body header.
    pub fn child_body_type(&self) -> URID {
        self.body.header.child_type
    }

    /// Return a slice containing all items in the vector.
    ///
    /// No allocation is done; This method simply borrows the data of the vector.
    pub fn as_slice(&self) -> &[T] {
        &self.body.data
    }
}

/// Extension for [`WritingFrame`](../frame/trait.WritingFrame.html) and
/// [`WritingFrameExt`](../frame/trait.WritingFrameExt.html) for vectors.
///
/// See the [module documentation](index.html) for more information.
pub trait VectorWritingFrame<'a, T>
where
    T: 'static + AtomBody + Sized + Copy,
    Self: WritingFrame<'a> + WritingFrameExt<'a, Vector<T>>,
{
    /// Push a value to the end of the vector.
    fn push(&mut self, value: T) -> Result<(), ()> {
        unsafe { Vector::<T>::push(self, value) }
    }

    /// Append a slice of values to the end of the vector.
    fn append(&mut self, slice: &[T]) -> Result<(), ()> {
        unsafe { Vector::<T>::append(self, slice) }
    }
}

impl<'a, T, F> VectorWritingFrame<'a, T> for F
where
    T: 'static + AtomBody + Sized + Copy,
    F: WritingFrame<'a> + WritingFrameExt<'a, Vector<T>>,
{
}
