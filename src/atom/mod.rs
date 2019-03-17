use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris::MappedURIDs;
use std::ffi::CStr;
use std::os::raw::c_int;
use urid::URID;

pub mod literal;
pub mod scalar;
pub mod string;
pub mod vector;

#[derive(Clone)]
#[repr(C)]
pub struct AtomHeader {
    pub size: c_int,
    pub atom_type: URID,
}

pub trait AtomBody {
    type InitializationParameter: ?Sized;

    fn get_uri() -> &'static CStr;

    fn get_urid(urids: &MappedURIDs) -> URID;

    fn initialize_body<'a, W>(
        writer: &mut W,
        parameter: &Self::InitializationParameter,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>;
}

#[derive(Clone)]
#[repr(C)]
pub struct Atom<A: AtomBody + ?Sized> {
    pub header: AtomHeader,
    pub body: A,
}

impl<A: AtomBody + ?Sized> std::ops::Deref for Atom<A> {
    type Target = A;
    fn deref(&self) -> &A {
        &self.body
    }
}

impl<A: AtomBody + ?Sized> std::ops::DerefMut for Atom<A> {
    fn deref_mut(&mut self) -> &mut A {
        &mut self.body
    }
}

impl<'a, A: AtomBody + ?Sized> From<&'a Atom<A>> for &'a AtomHeader {
    fn from(atom: &'a Atom<A>) -> &'a AtomHeader {
        unsafe { (atom as *const Atom<A> as *const AtomHeader).as_ref() }.unwrap()
    }
}

impl<'a, A: AtomBody + ?Sized> From<&'a mut Atom<A>> for &'a mut AtomHeader {
    fn from(atom: &'a mut Atom<A>) -> &'a mut AtomHeader {
        unsafe { (atom as *mut Atom<A> as *mut AtomHeader).as_mut() }.unwrap()
    }
}
