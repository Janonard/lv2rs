//! The purpose of this crate is to provide safe, idiomatic and easy-to-use means to use the type
//! system introduced by the LV2 atom library. This type system is (relatively) portable and can be
//! used to exchange information of arbitrary type among LV2 plugins.
//! 
//! This is a frozen prototype and therefore, development of this crate will not continue here. Further
//! development continues as [rust-lv2](https://github.com/rust-dsp/rust-lv2).
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
//! * [`Literal`](literal/index.html): A proper UTF-8 string.
//! * [`Object`](object/index.html): Compound type similar to tuples that maps URIDs to atoms.
//! * [`Sequence`](sequence/index.html): Tuple with additional time stamps for every atom.
//! Usually used for frame-perfect, event-based data.
//! * [`AtomString`](string/index.html): An old-school ASCII string, also used for URIs.
//! * [`Tuple`](tuple/index.html): Heterogenous array of atoms, including dynamically sized
//! ones.
//! * [`Vector`](vector/index.html): Homogenous array of sized atoms, like numbers.
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
//! [`write_atom_body`](ports/struct.AtomOutputPort.html#method.write_atom_body) method.
//!
//! Such [writing frames](frame/trait.WritingFrame.html) are able to write data to the provided
//! chunk. However, most of their methods are unsafe since they do not check the resulting output
//! for meaningfulness. Instead, you should use the safe methods provided by the writing frame
//! extensions, which are tailored for specific atoms and guarantee the consistency of the resulting
//! output. You can read more about them in their specific module descriptions.
extern crate lv2rs_urid as urid;

mod atom;
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

pub use atom::*;

/// Re-exportation module that contains all traits necessary to use lv2rs-atom.
pub mod prelude {
    pub use crate::frame::{WritingFrame, WritingFrameExt};
    pub use crate::scalar::ScalarAtomBody;

    // Atom bodies.
    pub use crate::atom::{Atom, AtomBody};
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
