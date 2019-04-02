//! Object- or Map-style atom container.
//!
//! An [object](type.Object.html) contains a series of atoms which are tagged with a key. This way,
//! one can view an atom in an object either as a property of an object (just like in
//! object-oriented programming) or as an entry in a URID->Atom map.
//!
//! When initialized, an object does not contain any items. Instead, you have to push them to the
//! end of the object using the [`ObjectWritingFrame`](trait.ObjectWritingFrame.html) trait. Every
//! writing frame implements this trait via a blanket implementation and the trait is included in
//! the crate's prelude. You can, therefore, act as if the extended method were normal methods of a
//! writing frame.
//!
//! Reading an object is accomplished by creating an iterator over the properties with the
//! [`iter`](../atom/struct.Atom.html#method.iter) method.
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
//!     use std::ffi::{CString, CStr};
//!
//!     pub struct Plugin {
//!         in_port: AtomInputPort<Object>,
//!         out_port: AtomOutputPort<Object>,
//!         urids: CachedMap,
//!     }
//!
//!     impl Plugin {
//!         /// Simulated `run` method.
//!         fn run(&mut self) {
//!             let my_class_urid = self.urids.map(
//!                 CStr::from_bytes_with_nul(b"https://example.org#MyClass\0").unwrap()
//!             );
//!             let a_urid = self.urids.map(
//!                 CStr::from_bytes_with_nul(b"https://example.org#a\0").unwrap()
//!             );
//!             let b_urid = self.urids.map(
//!                 CStr::from_bytes_with_nul(b"https://example.org#b\0").unwrap()
//!             );
//!
//!             // Writing
//!             {
//!                 // We are writing an object that is an instance of `MyClass`. This information
//!                 // is expressed by passing the URID of `MyClass` as the second parameter. In
//!                 // real plugins, you would describe `MyClass` in a Turtle document.
//!                 let mut frame =
//!                     unsafe {
//!                         self.out_port.write_atom(
//!                             &(0, my_class_urid),
//!                             &mut self.urids
//!                         )
//!                     }.unwrap();
//!
//!                 // Pushing a property requires a key, a context, the parameter for the atom type
//!                 // and a mut reference to the URID map.
//!                 frame.push_property::<i32>(a_urid, 0, &42, &mut self.urids).unwrap();
//!                 frame.push_property::<f32>(b_urid, 0, &17.0, &mut self.urids).unwrap();
//!             }
//!
//!             // Reading
//!             let atom = unsafe { self.in_port.get_atom(&mut self.urids) }.unwrap();
//!             // We're iterating through the properties. If a property matches our known key,
//!             // We assert that it has the right value.
//!             for (header, property) in atom.iter() {
//!                 if header.key == a_urid {
//!                     let a = property.cast::<i32>(&mut self.urids).unwrap();
//!                     assert_eq!(42, **a);
//!                 } else if header.key == b_urid {
//!                     let b = property.cast::<f32>(&mut self.urids).unwrap();
//!                     assert_eq!(17.0, **b);
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
use crate::atom::{array::*, *};
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::unknown::*;
use crate::uris;
use std::ffi::CStr;
use urid::URID;

/// The header of an object's property.
///
/// In original LV2, a property is a standalone atom, but since it is only useful within objects,
/// which don't need the atom properties of a property, it is not an atom.
///
/// The `key` represents the name of the property. The `context` is described by the standard as
/// "Context URID (may be, and generally is, 0)". It does not really say what it is used for,
/// but since it says that it may be 0, you should set it to 0.
///
/// This struct is also `repr(C)` and is used to interpret objects from raw data.
#[repr(C)]
pub struct PropertyHeader {
    pub key: URID,
    pub context: URID,
}

/// Header of an object.
///
/// The important field is `otype`, which contains the URID of the class this object is an instance
/// of. However, the `id` is only described as "URID, or 0 for blank" by the standard and therefore
/// should be set to zero.
///
/// This struct is also `repr(C)` and is used to interpret objects from raw data.
#[repr(C)]
pub struct ObjectHeader {
    pub id: URID,
    pub otype: URID,
}

impl ArrayAtomHeader for ObjectHeader {
    type InitializationParameter = (URID, URID);

    unsafe fn initialize<'a, W, T>(
        writer: &mut W,
        (id, otype): &(URID, URID),
        _urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
    {
        let header = ObjectHeader {
            id: *id,
            otype: *otype,
        };
        writer.write_sized(&header).map(|_| ())
    }
}

/// Object- or Map-style atom container.
///
/// See the [module documentation](index.html) for more information.
pub type Object = ArrayAtomBody<ObjectHeader, u8>;

impl AtomBody for Object {
    type InitializationParameter = (URID, URID);

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::OBJECT_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        (id, otype): &(URID, URID),
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, &(*id, *otype), urids)
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, WidenRefError> {
        Self::__widen_ref(header, urids)
    }
}

impl Atom<Object> {
    /// Create an iterator over all properties of the object.
    ///
    /// This iterator is based on the [`ChunkIterator`](../unknown/struct.ChunkIterator.html).
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a PropertyHeader, &'a Atom<Unknown>)> {
        ChunkIterator::<PropertyHeader>::new(&self.body.data)
    }
}

/// Extension for [`WritingFrame`](../frame/trait.WritingFrame.html) and
/// [`WritingFrameExt`](../frame/trait.WritingFrameExt.html) for vectors.
///
/// See the [module documentation](index.html) for more information.
pub trait ObjectWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Object> {
    /// Add a property to the object.
    ///
    /// The `key` and the `context` are the same as in the
    /// [`PropertyHeader`](struct.PropertyHeader.html): The key represents the name of the property
    /// and the `context`'s purpose is unknown and should be set to zero.
    fn push_property<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        key: URID,
        context: URID,
        parameter: &A::InitializationParameter,
        urids: &mut urid::CachedMap,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        let p_header = PropertyHeader {
            key: key,
            context: context,
        };
        unsafe {
            self.write_sized(&p_header)?;
            let mut frame = self.create_nested_frame::<A>(urids)?;
            A::initialize_body(&mut frame, parameter, urids)?;
            Ok(frame)
        }
    }
}

impl<'a, W> ObjectWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Object> {}
