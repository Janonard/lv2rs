use crate::frame::WritingFrame;
use crate::uris::MappedURIDs;
use std::ffi::CStr;
use std::mem::{size_of_val, transmute};
use std::ops::{Deref, DerefMut};
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

pub trait AtomBody {
    type ConstructionParameter: ?Sized;

    fn get_uri() -> &'static CStr;

    fn get_urid(urids: &MappedURIDs) -> URID;

    /*
    fn write_body<'a, F: WritingFrame>(
        frame: &'a mut F,
        parameter: &Self::ConstructionParameter,
    ) -> Result<&'a mut Self, ()>;*/
}

/*
#[repr(C)]
pub struct ArrayAtomBody<H, T> {
    header: H,
    body: [T],
}

impl<H, T> ArrayAtomBody<H, T>
where
    Self: AtomBody,
{
    pub fn __write_body<'a, F: WritingFrame>(
        frame: &'a mut F,
        header: &H,
        data: &[T],
    ) -> Result<&'a mut Self, ()> {
        // Writing the custom header.
        let ptr: *mut H = frame.write_sized(header, false)?.0;

        // Writing the data.
        let raw_data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, size_of_val(data)) };
        frame.write_raw(raw_data, true)?;

        // Creating a reference to the body.
        let ptr = unsafe { transmute::<(*mut H, usize), *mut Self>((ptr, 0)) };
        Ok(unsafe { ptr.as_mut() }.unwrap())
    }
}

impl<H, T> Deref for ArrayAtomBody<H, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.body
    }
}

impl<H, T> DerefMut for ArrayAtomBody<H, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.body
    }
}
*/
