use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use std::marker::PhantomData;
use urid::URID;

pub type Chunk = [u8];

impl Atom<Chunk> {
    pub fn cast<A: AtomBody + ?Sized>(&self, urids: &uris::MappedURIDs) -> Result<&Atom<A>, ()> {
        if self.header.atom_type == A::get_urid(urids) {
            unsafe { A::widen_ref(&self.header) }
        } else {
            Err(())
        }
    }
}

impl AtomBody for Chunk {
    type InitializationParameter = [u8];

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::CHUNK_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.chunk
    }

    unsafe fn initialize_body<'a, W>(writer: &mut W, data: &[u8]) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        writer.write_raw(data).map(|_| ())
    }

    unsafe fn widen_ref(header: &AtomHeader) -> Result<&Atom<Self>, ()> {
        let size = header.size as usize;

        // This is were the unsafe things happen!
        // We know the length of the string, therefore we can create a fat pointer to the atom.
        let fat_ptr: (*const AtomHeader, usize) = (header as *const AtomHeader, size);
        let fat_ptr: *const Atom<Self> = std::mem::transmute(fat_ptr);
        let atom_ref: &Atom<Self> = fat_ptr.as_ref().unwrap();

        Ok(atom_ref)
    }
}

pub struct ChunkIterator<'a, H: 'static + Sized> {
    data: &'a [u8],
    position: usize,
    phantom: PhantomData<H>,
}

impl<'a, H: 'static + Sized> ChunkIterator<'a, H> {
    pub fn new(data: &'a [u8]) -> Self {
        ChunkIterator {
            data: data,
            position: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, H: 'static + Sized> Iterator for ChunkIterator<'a, H> {
    type Item = (&'a H, &'a Atom<Chunk>);

    fn next(&mut self) -> Option<(&'a H, &'a Atom<Chunk>)> {
        use std::mem::size_of;

        if self.position >= self.data.len() {
            return None;
        }

        let data = &self.data[self.position..];
        if data.len() < size_of::<H>() + size_of::<AtomHeader>() {
            return None;
        }

        let pre_header = unsafe { (data.as_ptr() as *const H).as_ref() }?;
        let data = &data[size_of::<H>()..];
        let header = unsafe { (data.as_ptr() as *const AtomHeader).as_ref() }?;
        let chunk = match unsafe { Chunk::widen_ref(header) } {
            Ok(chunk) => chunk,
            Err(_) => return None,
        };
        self.position += size_of::<H>() + size_of::<AtomHeader>() + (chunk.header.size as usize);
        self.position += self.position % 8; // padding

        Some((pre_header, chunk))
    }
}
