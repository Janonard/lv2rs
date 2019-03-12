use crate::atom::{ArrayAtomBody, Atom, AtomBody};
use crate::frame::WritingFrame;
use crate::uris;
use std::ffi::CStr;
use std::os::raw::*;
use urid::URID;

pub enum Unit {
    Frames,
    Beats,
}

#[repr(C)]
pub struct SequenceHeader {
    unit: c_uint,
    pad: c_uint,
}

pub type Sequence = ArrayAtomBody<SequenceHeader, u8>;

impl AtomBody for Sequence {
    type ConstructionParameter = Unit;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::SEQUENCE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.sequence
    }

    fn write_body<'a, F: WritingFrame>(frame: &'a mut F, unit: &Unit) -> Result<&'a mut Self, ()> {
        let header = SequenceHeader {
            // TODO: URIDs einfügen!
            unit: match unit {
                Unit::Frames => 0,
                Unit::Beats => 1,
            },
            pad: 0,
        };
        Self::__write_body(frame, &header, &[0; 0])
    }
}

#[repr(C)]
pub union Time {
    frames: c_long,
    beats: c_double,
}

#[repr(C)]
pub struct Event<A: AtomBody + Clone + ?Sized> {
    time: Time,
    body: Atom<A>,
}
