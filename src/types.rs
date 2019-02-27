use crate::{uris, Atom, AtomHeader};

use std::ffi::CStr;
use urid::URID;

/// An atom:Tuple.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomTuple {
    /// Atom header.
    pub atom: AtomHeader,
    // Contents (a series of complete atoms) follow here.
}

impl Atom for AtomTuple {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::TUPLE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.tuple
    }
}

/// The body of an atom:Vector.
#[repr(C)]
pub struct AtomVectorBody {
    /// The size of each element in the vector.
    pub child_size: u32,
    /// The type of each element in the vector.
    pub child_type: URID,
    // Contents (a series of packed atom bodies) follow here.
}

/// An atom:Vector.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomVector {
    /// Atom header.
    pub atom: AtomHeader,
    /// Body.
    pub body: AtomVectorBody,
}

impl Atom for AtomVector {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::VECTOR_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.vector
    }
}

/// The body of an atom:Property (e.g. in an atom:Object).
#[repr(C)]
pub struct AtomPropertyBody {
    /// Key (predicate) (mapped URI).
    pub key: URID,
    /// Context URID (may be, and generally is, 0).
    pub context: URID,
    /// Value atom header.
    pub value: AtomHeader,
    // Value atom body follows here.
}

/// An atom:Property.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomProperty {
    /// Atom header.
    pub atom: AtomHeader,
    /// Body.
    pub body: AtomPropertyBody,
}

impl Atom for AtomProperty {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::PROPERTY_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.property
    }
}

/// The body of an atom:Object.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomObjectBody {
    /// URID, or 0 for blank.
    pub id: URID,
    /// Type URID (same as rdf:type, for fast dispatch).
    pub otype: URID,
    // Contents (a series of property bodies) follow here.
}

/// An atom:Object.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomObject {
    /// Atom header.
    pub atom: AtomHeader,
    /// Body.
    pub body: AtomObjectBody,
}

impl Atom for AtomObject {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::OBJECT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.object
    }
}

/// The header of an atom:Event.
///
/// Note this type is NOT an Atom.
#[repr(C)]
pub struct AtomEvent {
    /// Time stamp. Which type is valid (i64 or f64) is determined by context:
    /// If given as i64, Time in audio frames,
    /// If given as f64, Time in beats.
    pub frames: [u8; 4],
    /// Event body atom header.
    pub body: AtomHeader,
    // Body atom contents follow here.
}

/// The body of an atom:Sequence (a sequence of events).
///
/// The unit field is either a URID that described an appropriate time stamp
/// type, or may be 0 where a default stamp type is known.  For
/// Descriptor::run(), the default stamp type is audio frames.
///
/// The contents of a sequence is a series of Atom_Event, each aligned
/// to 64-bits, e.g.:
/// <pre>
/// | Event 1 (size 6)                              | Event 2
/// |       |       |       |       |       |       |       |       |
/// | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | |
/// |FRAMES |SUBFRMS|TYPE   |SIZE   |DATADATADATAPAD|FRAMES |SUBFRMS|...
/// </pre>
#[repr(C)]
pub struct AtomSequenceBody {
    /// URID of unit of event time stamps.
    pub unit: URID,
    /// Currently unused.
    pub pad: URID,
    // Contents (a series of events) follow here.
}

/// An atom:Sequence.
#[repr(C)]
pub struct AtomSequence {
    /// Atom header.
    pub atom: AtomHeader,
    /// Body.
    pub body: AtomSequenceBody,
}

impl Atom for AtomSequence {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::SEQUENCE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.sequence
    }
}
