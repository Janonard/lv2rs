use crate::uris;
use std::ffi::CStr;
use urid::URID;

pub trait Atom {
    fn get_uri<'a>() -> &'a std::ffi::CStr;
    fn get_urid(urids: &uris::MappedURIDs) -> URID;
}

/// The header of an atom:Atom.
#[repr(C)]
pub struct AtomHeader {
    /// Size in bytes, not including type and size.
    pub size: u32,
    /// Type of this atom (mapped URI).
    pub atom_type: URID,
}

/// An atom:Int or atom:Bool.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomInt {
    /// Atom header.
    pub atom: AtomHeader,
    /// Integer value.
    pub body: i32,
}

impl Atom for AtomInt {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::INT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.int
    }
}

/// An atom:Long.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomLong {
    /// Atom header.
    pub atom: AtomHeader,
    /// Integer value.
    pub body: i64,
}

impl Atom for AtomLong {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LONG_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.long
    }
}

/// An atom:Float.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomFloat {
    /// Atom header.
    pub atom: AtomHeader,
    /// Floating point value.
    pub body: f32,
}

impl Atom for AtomFloat {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::FLOAT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.float
    }
}

/// An atom:Double.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomDouble {
    /// Atom header.
    pub atom: AtomHeader,
    /// Floating point value.
    pub body: f64,
}

impl Atom for AtomDouble {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::DOUBLE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.double
    }
}

/// An atom:Bool.
///
/// May be cast to Atom.
pub type AtomBool = AtomInt;

/// An atom:URID.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomURID {
    /// Atom header.
    pub atom: AtomHeader,
    /// URID.
    pub body: URID,
}

impl Atom for AtomURID {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::URID_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.urid
    }
}

/// An atom:String.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomString {
    /// Atom header.
    pub atom: AtomHeader,
    // Contents (a null-terminated UTF-8 string) follow here.
}

impl Atom for AtomString {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::STRING_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.string
    }
}

/// The body of an atom:Literal.
#[repr(C)]
pub struct AtomLiteralBody {
    /// Datatype URID.
    pub datatype: URID,
    /// Language URID.
    pub lang: URID,
    // Contents (a null-terminated UTF-8 string) follow here.
}

/// An atom:Literal.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomLiteral {
    /// Atom header.
    pub atom: AtomHeader,
    /// Body.
    pub body: AtomLiteralBody,
}

impl Atom for AtomLiteral {
    fn get_uri<'a>() -> &'a CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::LITERAL_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.literal
    }
}

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
    pub frames: i64,
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
