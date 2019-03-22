use crate::atom::{
    array::{ArrayAtomBody, ArrayAtomHeader},
    Atom, AtomBody, AtomHeader,
};
use crate::chunk::*;
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use urid::URID;

#[repr(C)]
pub struct PropertyHeader {
    key: URID,
    context: URID,
}

#[repr(C)]
pub struct ObjectHeader {
    id: URID,
    otype: URID,
}

impl ArrayAtomHeader for ObjectHeader {
    type InitializationParameter = (URID, URID);

    fn initialize<'a, W, T>(writer: &mut W, (id, otype): &(URID, URID)) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
    {
        let header = ObjectHeader {
            id: *id,
            otype: *otype,
        };
        unsafe { writer.write_sized(&header) }.map(|_| ())
    }
}

pub type Object = ArrayAtomBody<ObjectHeader, u8>;

impl AtomBody for Object {
    type InitializationParameter = (URID, URID);

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::OBJECT_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.object
    }

    unsafe fn initialize_body<'a, W>(writer: &mut W, (id, otype): &(URID, URID)) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, &(*id, *otype))
    }

    unsafe fn widen_ref(header: &AtomHeader) -> Result<&Atom<Self>, ()> {
        Self::__widen_ref(header)
    }
}

impl Atom<Object> {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a PropertyHeader, &'a Atom<Chunk>)> {
        ChunkIterator::<PropertyHeader>::new(&self.body.data)
    }
}

pub trait ObjectWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Object> {
    fn push_property<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        p_header: &PropertyHeader,
        parameter: &A::InitializationParameter,
        urids: &uris::MappedURIDs,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        unsafe {
            self.write_sized(p_header)?;
            let mut frame = self.create_atom_frame::<A>(urids)?;
            A::initialize_body(&mut frame, parameter)?;
            Ok(frame)
        }
    }
}

impl<'a, W> ObjectWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Object> {}
