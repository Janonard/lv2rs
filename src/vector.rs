use crate::atom::array::{ArrayAtomBody, ArrayAtomHeader};
use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::*;
use urid::URID;

#[repr(C)]
pub struct VectorHeader {
    child_size: c_uint,
    child_type: c_uint,
}

pub type Vector<T> = ArrayAtomBody<VectorHeader, T>;

impl ArrayAtomHeader for VectorHeader {
    type InitializationParameter = URID;

    unsafe fn initialize<'a, W, T>(writer: &mut W, child_type: &URID) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
    {
        let header = VectorHeader {
            child_size: size_of::<T>() as u32,
            child_type: *child_type,
        };
        writer.write_sized(&header)?;
        Ok(())
    }
}

impl<T> AtomBody for Vector<T>
where
    T: 'static + AtomBody + Sized + Copy,
{
    type InitializationParameter = T::MappedURIDs;

    type MappedURIDs = uris::MappedURIDs;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::VECTOR_TYPE_URI) }
    }

    fn get_urid(urids: &Self::MappedURIDs) -> URID {
        urids.vector
    }

    unsafe fn initialize_body<'a, W>(writer: &mut W, urids: &T::MappedURIDs) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, &T::get_urid(urids))
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &uris::MappedURIDs,
    ) -> Result<&'a Atom<Self>, ()> {
        Self::__widen_ref(header, urids)
    }
}

impl<T> Atom<Vector<T>>
where
    T: 'static + AtomBody + Sized + Copy,
{
    pub fn child_body_size(&self) -> usize {
        self.body.header.child_size as usize
    }

    pub fn child_body_type(&self) -> URID {
        self.body.header.child_type
    }

    pub fn as_slice(&self) -> &[T] {
        &self.body.data
    }
}

pub trait VectorWritingFrame<'a, T>
where
    T: 'static + AtomBody + Sized + Copy,
    Self: WritingFrame<'a> + WritingFrameExt<'a, Vector<T>>,
{
    fn push(&mut self, value: T) -> Result<(), ()> {
        unsafe { Vector::<T>::push(self, value) }
    }

    fn append(&mut self, slice: &[T]) -> Result<(), ()> {
        unsafe { Vector::<T>::append(self, slice) }
    }
}

impl<'a, T, F> VectorWritingFrame<'a, T> for F
where
    T: 'static + AtomBody + Sized + Copy,
    F: WritingFrame<'a> + WritingFrameExt<'a, Vector<T>>,
{
}
