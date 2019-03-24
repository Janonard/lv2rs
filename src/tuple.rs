use crate::atom::{array::ArrayAtomBody, Atom, AtomBody, AtomHeader};
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::unknown::*;
use crate::uris;
use std::ffi::CStr;

pub type Tuple = ArrayAtomBody<(), u8>;

impl AtomBody for Tuple {
    type InitializationParameter = ();

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::TUPLE_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        parameter: &(),
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, parameter, urids)
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, ()> {
        Self::__widen_ref(header, urids)
    }
}

impl Atom<Tuple> {
    pub fn iter(&self) -> impl Iterator<Item = &Atom<Unknown>> {
        ChunkIterator::<()>::new(&self.body.data).map(|(_, chunk): (&(), &Atom<Unknown>)| chunk)
    }
}

pub trait TupleWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Tuple> {
    fn push_atom<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        parameter: &A::InitializationParameter,
        urids: &mut urid::CachedMap,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        unsafe {
            let mut frame = self.create_nested_frame::<A>(urids)?;
            A::initialize_body(&mut frame, parameter, urids)?;
            Ok(frame)
        }
    }
}

impl<'a, W> TupleWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Tuple> {}
