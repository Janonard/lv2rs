use crate::{uris, Atom, AtomHeader};

use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use urid::URID;

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

impl Deref for AtomInt {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.body
    }
}

impl DerefMut for AtomInt {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.body
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

impl Deref for AtomLong {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.body
    }
}

impl DerefMut for AtomLong {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.body
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

impl Deref for AtomFloat {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.body
    }
}

impl DerefMut for AtomFloat {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.body
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

impl Deref for AtomDouble {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.body
    }
}

impl DerefMut for AtomDouble {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.body
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

impl Deref for AtomURID {
    type Target = URID;
    fn deref(&self) -> &URID {
        &self.body
    }
}

impl DerefMut for AtomURID {
    fn deref_mut(&mut self) -> &mut URID {
        &mut self.body
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

impl Deref for AtomString {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        use std::mem::size_of;
        use std::os::raw::c_char;

        let atom_pointer = self as *const Self;
        let atom_pointer = unsafe { atom_pointer.add(size_of::<AtomString>()) };
        unsafe { CStr::from_ptr(atom_pointer as *const c_char) }
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

impl AtomLiteral {
    pub fn try_to_str(&self) -> Option<&str> {
        use std::mem::size_of;

        let size = self.atom.size as usize - size_of::<AtomLiteralBody>();
        let str_ptr = unsafe { (self as *const AtomLiteral).add(1) } as *const u8;
        let slice = unsafe { std::slice::from_raw_parts(str_ptr, size) };

        match std::str::from_utf8(slice) {
            Ok(string) => Some(string),
            Err(_) => None,
        }
    }
}
