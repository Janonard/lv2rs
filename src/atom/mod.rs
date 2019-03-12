use crate::uris::MappedURIDs;
use std::ffi::CStr;
use std::mem::size_of_val;
use urid::URID;

//mod object;
mod scalar;
//mod sequence;
//mod string;
//mod vector;

//pub use object::*;
pub use scalar::*;
//pub use sequence::*;
//pub use string::*;
//pub use vector::*;

#[derive(Clone)]
#[repr(C)]
pub struct AtomHeader {
    pub size: c_int,
    pub atom_type: URID,
}

pub trait AtomBody {
    fn get_uri() -> &'static CStr;

    fn get_urid(urids: &MappedURIDs) -> URID;
}

#[derive(Clone)]
#[repr(C)]
pub struct Atom<A: AtomBody + Clone + ?Sized> {
    pub header: AtomHeader,
    pub body: A,
}

impl<A: AtomBody + Clone + ?Sized> Atom<A> {
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

impl<A: AtomBody + Clone + ?Sized> std::ops::Deref for Atom<A> {
    type Target = A;
    fn deref(&self) -> &A {
        &self.body
    }
}

impl<A: AtomBody + Clone + ?Sized> std::ops::DerefMut for Atom<A> {
    fn deref_mut(&mut self) -> &mut A {
        &mut self.body
    }
}

impl<'a, A: AtomBody + Clone + ?Sized> From<&'a Atom<A>> for &'a AtomHeader {
    fn from(atom: &'a Atom<A>) -> &'a AtomHeader {
        unsafe { (atom as *const Atom<A> as *const AtomHeader).as_ref() }.unwrap()
    }
}

impl<'a, A: AtomBody + Clone + ?Sized> From<&'a mut Atom<A>> for &'a mut AtomHeader {
    fn from(atom: &'a mut Atom<A>) -> &'a mut AtomHeader {
        unsafe { (atom as *mut Atom<A> as *mut AtomHeader).as_mut() }.unwrap()
    }
}
