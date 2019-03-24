use crate::atom::{
    array::{ArrayAtomBody, ArrayAtomHeader},
    Atom, AtomBody, AtomHeader,
};
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::unknown::*;
use crate::uris;
use std::ffi::CStr;
use urid::URID;

#[repr(C)]
pub struct PropertyHeader {
    pub key: URID,
    pub context: URID,
}

#[repr(C)]
pub struct ObjectHeader {
    pub id: URID,
    pub otype: URID,
}

impl ArrayAtomHeader for ObjectHeader {
    type InitializationParameter = (URID, URID);

    unsafe fn initialize<'a, W, T>(
        writer: &mut W,
        (id, otype): &(URID, URID),
        _urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
    {
        let header = ObjectHeader {
            id: *id,
            otype: *otype,
        };
        writer.write_sized(&header).map(|_| ())
    }
}

pub type Object = ArrayAtomBody<ObjectHeader, u8>;

impl AtomBody for Object {
    type InitializationParameter = (URID, URID);

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::OBJECT_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        (id, otype): &(URID, URID),
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, &(*id, *otype), urids)
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, ()> {
        Self::__widen_ref(header, urids)
    }
}

impl Atom<Object> {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a PropertyHeader, &'a Atom<Unknown>)> {
        ChunkIterator::<PropertyHeader>::new(&self.body.data)
    }
}

pub trait ObjectWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Object> {
    fn push_property<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        p_header: &PropertyHeader,
        parameter: &A::InitializationParameter,
        urids: &mut urid::CachedMap,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        unsafe {
            self.write_sized(p_header)?;
            let mut frame = self.create_nested_frame::<A>(urids)?;
            A::initialize_body(&mut frame, parameter, urids)?;
            Ok(frame)
        }
    }
}

impl<'a, W> ObjectWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Object> {}
