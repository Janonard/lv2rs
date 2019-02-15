use urid::URID;

/// The header of an atom:Atom.
#[repr(C)]
pub struct Atom {
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
    pub atom: Atom,
    /// Integer value.
    pub body: i32,
}

/// An atom:Long.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomLong {
    /// Atom header.
    pub atom: Atom,
    /// Integer value.
    pub body: i64,
}

/// An atom:Float.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomFloat {
    /// Atom header.
    pub atom: Atom,
    /// Floating point value.
    pub body: f32,
}

/// An atom:Double.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomDouble {
    /// Atom header.
    pub atom: Atom,
    /// Floating point value.
    pub body: f64,
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
    pub atom: Atom,
    /// URID.
    pub body: URID,
}

/// An atom:String.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomString {
    /// Atom header.
    pub atom: Atom,
    // Contents (a null-terminated UTF-8 string) follow here.
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
    pub atom: Atom,
    /// Body.
    pub body: AtomLiteralBody,
}

/// An atom:Tuple.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomTuple {
    /// Atom header.
    pub atom: Atom,
    // Contents (a series of complete atoms) follow here.
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
    pub atom: Atom,
    /// Body.
    pub body: AtomVectorBody,
}

/// The body of an atom:Property (e.g. in an atom:Object).
#[repr(C)]
pub struct AtomPropertyBody {
    /// Key (predicate) (mapped URI).
    pub key: URID,
    /// Context URID (may be, and generally is, 0).
    pub context: URID,
    /// Value atom header.
    pub value: Atom,
    // Value atom body follows here.
}

/// An atom:Property.
///
/// May be cast to Atom.
#[repr(C)]
pub struct AtomProperty {
    /// Atom header.
    pub atom: Atom,
    /// Body.
    pub body: AtomPropertyBody,
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
    pub atom: Atom,
    /// Body.
    pub body: AtomObjectBody,
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
    pub body: Atom,
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
    pub atom: Atom,
    /// Body.
    pub body: AtomSequenceBody,
}
