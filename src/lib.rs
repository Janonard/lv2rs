extern crate lv2rs_core as core;
extern crate lv2rs_urid as urid;

/// The header of an atom:Atom.
#[repr(C)]
pub struct AtomHeader {
    /// Size in bytes, not including type and size.
    pub size: u32,
    /// Type of this atom (mapped URI).
    pub atom_type: urid::URID,
}

pub trait Atom {
    fn get_uri<'a>() -> &'a std::ffi::CStr;
    fn get_urid(urids: &uris::MappedURIDs) -> urid::URID;
}

mod port;
mod scalar;
mod types;

pub mod uris;

pub use port::*;
pub use scalar::*;
pub use types::*;
