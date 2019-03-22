extern crate lv2rs_core as core;
extern crate lv2rs_urid as urid;

pub mod atom;
pub mod chunk;
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

pub mod prelude {
    pub use crate::atom::{Atom, AtomBody, AtomHeader};
    pub use crate::frame::{WritingFrame, WritingFrameExt};
    pub use crate::scalar::ScalarAtomBody;

    // Atom bodies.
    pub use crate::{
        chunk::Chunk, literal::Literal, object::Object, sequence::Sequence, string::AtomString,
        tuple::Tuple, vector::Vector,
    };

    // Writing frame extensions
    pub use crate::{
        literal::LiteralWritingFrame, object::ObjectWritingFrame, sequence::SequenceWritingFrame,
        string::AtomStringWritingFrame, tuple::TupleWritingFrame, vector::VectorWritingFrame,
    };
}
