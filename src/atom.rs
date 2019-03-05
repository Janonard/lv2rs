use crate::uris::MappedURIDs;
use std::ffi::CStr;
use std::mem::size_of_val;
use std::os::raw::*;
use urid::URID;

#[derive(Clone)]
#[repr(C)]
pub struct AtomHeader {
    pub size: c_int,
    pub atom_type: URID,
}

#[derive(Clone)]
#[repr(C)]
pub struct Atom<A: AtomBody>
where
    A: Clone,
{
    pub header: AtomHeader,
    pub body: A,
}

impl<A: AtomBody> Atom<A>
where
    A: Clone,
{
    pub fn from_body(body: A, urids: &MappedURIDs) -> Self {
        let header = AtomHeader {
            size: size_of_val(&body) as c_int,
            atom_type: A::get_urid(urids),
        };
        Self {
            header: header,
            body: body,
        }
    }
}

impl<'a, A: AtomBody + Clone> From<&'a Atom<A>> for &'a AtomHeader {
    fn from(atom: &'a Atom<A>) -> &'a AtomHeader {
        unsafe { (atom as *const Atom<A> as *const AtomHeader).as_ref() }.unwrap()
    }
}

impl<'a, A: AtomBody + Clone> From<&'a mut Atom<A>> for &'a mut AtomHeader {
    fn from(atom: &'a mut Atom<A>) -> &'a mut AtomHeader {
        unsafe { (atom as *mut Atom<A> as *mut AtomHeader).as_mut() }.unwrap()
    }
}

pub trait AtomBody {
    fn get_uri() -> &'static CStr;

    fn get_urid(urids: &MappedURIDs) -> URID;

    type WritingParameter;

    type WritingHandle: Default;
}
