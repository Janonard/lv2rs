extern crate lv2rs_core as core;
extern crate lv2rs_urid as urid;

pub mod atom;
pub mod frame;
pub mod ports;
pub mod uris;

pub mod prelude {
    pub use crate::atom::scalar::ScalarAtomBody;
    pub use crate::atom::{Atom, AtomBody, AtomHeader};
    pub use crate::frame::{WritingFrame, WritingFrameExt};

    // Atom bodies.
    pub use crate::atom::{
        chunk::Chunk, literal::Literal, object::Object, sequence::Sequence, string::AtomString,
        tuple::Tuple, vector::Vector,
    };

    // Writing frame extensions
    pub use crate::atom::literal::LiteralWritingFrame;
    pub use crate::atom::object::ObjectWritingFrame;
    pub use crate::atom::sequence::SequenceWritingFrame;
    pub use crate::atom::string::AtomStringWritingFrame;
    pub use crate::atom::tuple::TupleWritingFrame;
    pub use crate::atom::vector::VectorWritingFrame;
}
