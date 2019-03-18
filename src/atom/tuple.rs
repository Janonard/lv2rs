use crate::atom::array::ArrayAtomBody;
use crate::atom::{Atom, AtomBody, AtomHeader, Chunk};
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use urid::URID;

pub type Tuple = ArrayAtomBody<(), u8>;

impl AtomBody for Tuple {
    type InitializationParameter = ();

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::TUPLE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.tuple
    }

    fn initialize_body<'a, W>(writer: &mut W, parameter: &()) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, parameter)
    }

    unsafe fn widen_ref(header: &AtomHeader) -> Result<&Atom<Self>, ()> {
        Self::__widen_ref(header)
    }
}

pub struct Iter<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Atom<Chunk>;

    fn next(&mut self) -> Option<&'a Atom<Chunk>> {
        if self.position >= self.data.len() {
            return None;
        }

        let data = &self.data[self.position..];
        if data.len() < std::mem::size_of::<AtomHeader>() {
            return None;
        }
        let header = unsafe { (data.as_ptr() as *const AtomHeader).as_ref() }?;
        let chunk = unsafe { Chunk::widen_ref(header) }.unwrap();
        self.position += std::mem::size_of::<AtomHeader>() + (chunk.header.size as usize);
        self.position += self.position % 8; // padding

        Some(chunk)
    }
}

impl Atom<Tuple> {
    pub fn iter(&self) -> Iter {
        Iter {
            data: &self.body.data,
            position: 0,
        }
    }
}

pub trait TupleWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Tuple> {
    fn push_atom<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        parameter: &A::InitializationParameter,
        urids: &uris::MappedURIDs,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        let mut frame = unsafe { self.create_atom_frame::<A>(urids)? };
        A::initialize_body(&mut frame, parameter)?;
        Ok(frame)
    }
}

impl<'a, W> TupleWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Tuple> {}
