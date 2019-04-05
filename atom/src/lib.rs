//! A rust re-implementation of the LV2 atom library.
//!
//! The purpose of this crate is to provide safe, idiomatic and easy-to-use means to use the type
//! system introduced by the LV2 atom library. This type system is (relatively) portable and can be
//! used to exchange information of arbitrary type among LV2 plugins.
//!
//! ## What are atoms?
//!
//! On an abstract level, every atom consist of a header, which contains the URID of the atom type
//! and a size in bytes, and body, a chunk of memory with the specified size. The interpretation of
//! this body is dependent on the atom type and one of the features of this crate. Since this data is
//! supposed to be "plain old data" and therefore must not contain references to other objects,
//! the host does not need to "understand" the atoms; It simply copies the data.
//!
//! There are several types of atoms which can be used to express almost any data:
//!
//! * Numbers: All types that implement the [`ScalarAtomBody`](scalar/trait.ScalarAtomBody.html)
//! trait:
//!     * `f32`
//!     * `f64`
//!     * `i32`
//!     * `i64`
//!     * `bool`
//!     * `URID`
//! * [`Literal`](literal/type.Literal.html): A proper UTF-8 string.
//! * [`AtomString`](string/type.AtomString.html): An old-school ASCII string, also used for URIs.
//! * [`Vector`](vector/type.Vector.html): Homogenous array of sized atoms, like numbers.
//! * [`Tuple`](tuple/type.Tuple.html): Heterogenous array of atoms, including dynamically sized
//! ones.
//! * [`Sequence`](sequence/type.Sequence.html): Tuple with additional time stamps for every atom.
//! Usually used for frame-perfect, event-based data.
//! * [`Object`](object/type.Object.html): Compound type similar to tuples that maps URIDs to atoms.
//! * [`Chunk`](chunk/type.Chunk.html): Simple chunk of bytes. Often used for unknown or
//! not-yet-known atoms.
//!
//! The purpose of this crate is to provide means to correctly read and construct said objects.
//!
//! ## How does it work?
//!
//! ### Reading
//!
//! In vanilla LV2, everything is expressed with floating-point numbers. The host passes a pointer
//! to a floating pointer number to the plugin and the plugin uses the number the pointer points to.
//! Reading atoms is similiar, but not completely the same:
//!
//! The host passes a pointer to an atom header to the plugin. Now, the plugin has to interpret this
//! header. It looks at the size and the type noted in the header and tries to "widen" the reference
//! to a fully-grown atom reference. Since this might be a delicate task, it is done by the
//! [`AtomInputPort`](ports/struct.AtomInputPort.html). Some atoms are easy to use, for example
//! floats, but some require special methods to be usefull.
//!
//! ### Writing
//!
//! If a plugin has to write information to an atom port, the host provides the plugin with a pointer
//! to a chunk of free space it can write to. An [`AtomOutputPort`](ports/struct.AtomOutputPort.html)
//! can interpret this pointer and creates a writing frame for it using the
//! [`write_atom`](ports/struct.AtomOutputPort.html#method.write_atom) method.
//!
//! Such [writing frames](frame/trait.WritingFrame.html) are able to write data to the provided
//! chunk. However, most of their methods are unsafe since they do not check the resulting output
//! for meaningfulness. Instead, you should use the safe methods provided by the writing frame
//! extensions, which are tailored for specific atoms and guarantee the consistency of the resulting
//! output.
extern crate lv2rs_urid as urid;

pub mod atom;
pub mod frame;
pub mod literal;
pub mod object;
pub mod ports;
pub mod scalar;
pub mod sequence;
pub mod string;
pub mod tuple;
pub mod uris;
pub mod vector;

/// Re-exportation module that contains all traits necessary to use lv2rs-atom.
pub mod prelude {
    pub use crate::frame::{WritingFrame, WritingFrameExt};
    pub use crate::scalar::ScalarAtomBody;

    // Atom bodies.
    pub use crate::{
        literal::Literal, object::Object, sequence::Sequence, string::AtomString, tuple::Tuple,
        vector::Vector,
    };

    // Writing frame extensions
    pub use crate::{
        literal::LiteralWritingFrame, object::ObjectWritingFrame, sequence::SequenceWritingFrame,
        tuple::TupleWritingFrame, vector::VectorWritingFrame,
    };
}
