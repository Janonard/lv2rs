//! Raw body for un- or not-yet identified atoms.
//!
//! Sometimes you need to use a full reference to an atom, but you can't or don't know what type
//! it is. In this case, you can use an `Atom<Unknown>`. [`Unknown`](type.Unknown.html) simply is an
//! alias for a byte slice and has a full [`AtomBody`](../atom/trait.AtomBody.html) implementation.
//! Also, you can always widen an `&AtomHeader` to a `&Atom<Unknown>` using the
//! [`widen_ref_unknown`](../atom/struct.AtomHeader.html#method.widen_ref_unknown) method and try to
//! cast an `&Atom<Unknown>` to any atom using the [`cast`](../atom/struct.Atom.html#method.cast).
//!
//! Another feature of this module is the [`ChunkIterator`](struct.ChunkIterator.html). It takes
//! a byte slice and iterates through all atoms in this slice. It makes the use of compound atoms
//! like [`Sequence`s](../sequence/index.html) or [`Object`s](../object/index.html)
//! even possible.
use crate::atom::*;
use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use std::marker::PhantomData;

/// Raw body for un- or not-yet identified atoms.
///
/// See the [module documentation](index.html) for more information.
pub type Unknown = [u8];

impl Atom<Unknown> {
    /// Try to cast the the `Atom<Unknown>` reference into a proper atom references.
    pub fn cast<A: AtomBody + ?Sized>(
        &self,
        urids: &mut urid::CachedMap,
    ) -> Result<&Atom<A>, WidenRefError> {
        unsafe { A::widen_ref(&self.header, urids) }
    }
}

impl AtomHeader {
    /// Widen the header reference to a `AtomUnknown` reference.
    ///
    /// This method simply takes the size noted in the header and creates a fat pointer to the
    /// atom.
    ///
    /// This method is unsafe since the allocated space behind the header could be samller than
    /// tthe size noted in the header. Therefore, further use of the atom reference could lead to
    /// undefined behaviour, although there is no way to be sure about the allocated space.
    pub unsafe fn widen_ref_unknown(&self) -> &Atom<Unknown> {
        let size = self.size as usize;

        // This is were the unsafe things happen!
        // We know the length of the string, therefore we can create a fat pointer to the atom.
        let fat_ptr: (*const AtomHeader, usize) = (self as *const AtomHeader, size);
        let fat_ptr: *const Atom<Unknown> = std::mem::transmute(fat_ptr);
        let atom_ref: &Atom<Unknown> = fat_ptr.as_ref().unwrap();

        atom_ref
    }
}

impl AtomBody for Unknown {
    type InitializationParameter = [u8];

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::CHUNK_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        data: &[u8],
        _urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        writer.write_raw(data).map(|_| ())
    }

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, WidenRefError> {
        let atom_ref = AtomHeader::widen_ref_unknown(header);
        if atom_ref.header.atom_type
            == urids.map(CStr::from_bytes_with_nul_unchecked(uris::CHUNK_TYPE_URI))
        {
            Ok(atom_ref)
        } else {
            Err(WidenRefError::WrongURID)
        }
    }
}

/// Iterator over atoms.
///
/// This iterator takes a slice of bytes and tries to iterate over all atoms in this slice. If
/// there is an error while iterating, iteration will end.
pub struct ChunkIterator<'a, H: 'static + Sized> {
    data: &'a [u8],
    position: usize,
    phantom: PhantomData<H>,
}

impl<'a, H: 'static + Sized> ChunkIterator<'a, H> {
    /// Create a new chunk iterator.
    pub fn new(data: &'a [u8]) -> Self {
        ChunkIterator {
            data: data,
            position: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, H: 'static + Sized> Iterator for ChunkIterator<'a, H> {
    type Item = (&'a H, &'a Atom<Unknown>);

    fn next(&mut self) -> Option<(&'a H, &'a Atom<Unknown>)> {
        use std::mem::size_of;

        // pad to the next 64-bit aligned position.
        self.position += self.position % 8;
        if self.position >= self.data.len() {
            return None;
        }

        let data = &self.data[self.position..];
        if data.len() < size_of::<H>() + size_of::<AtomHeader>() {
            return None;
        }

        let pre_header = unsafe { (data.as_ptr() as *const H).as_ref() }?;
        let data = &data[size_of::<H>()..];
        let atom_header = unsafe { (data.as_ptr() as *const AtomHeader).as_ref() }?;
        let item_size = size_of::<H>() + size_of::<AtomHeader>() + atom_header.size as usize;

        // Check if the atom actually fits.
        if self.position + item_size > self.data.len() {
            return None;
        }

        // Widen the header ref.
        let chunk = unsafe { AtomHeader::widen_ref_unknown(atom_header) };

        // Apply the bodies' size.
        self.position += item_size;

        Some((pre_header, chunk))
    }
}
