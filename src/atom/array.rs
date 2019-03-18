use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::frame::{WritingFrame, WritingFrameExt};
use std::mem::{size_of, size_of_val, transmute};

pub trait ArrayAtomHeader: Sized {
    type InitializationParameter: ?Sized;

    fn initialize<'a, W, T>(
        writer: &mut W,
        parameter: &Self::InitializationParameter,
    ) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>;
}

#[repr(C)]
pub struct ArrayAtomBody<H, T>
where
    H: ArrayAtomHeader,
    T: 'static + Sized + Copy,
{
    pub header: H,
    pub data: [T],
}

impl<H, T> ArrayAtomBody<H, T>
where
    Self: AtomBody,
    H: ArrayAtomHeader,
    T: 'static + Sized + Copy,
{
    pub fn __initialize_body<'a, W>(
        writer: &mut W,
        parameter: &H::InitializationParameter,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        H::initialize(writer, parameter)
    }

    pub unsafe fn __widen_ref(header: &AtomHeader) -> Result<&Atom<Self>, ()> {
        let body_size = header.size as usize;
        let array_header_size = size_of::<H>();
        if body_size < array_header_size {
            return Err(());
        }

        let body_size = body_size - array_header_size;
        if body_size % size_of::<T>() != 0 {
            return Err(());
        }
        let vector_len: usize = body_size / size_of::<T>();

        // This is were the unsafe things happen!
        // We know the length of the string, therefore we can create a fat pointer to the atom.
        let fat_ptr: (*const AtomHeader, usize) = (header as *const AtomHeader, vector_len);
        let fat_ptr: *const Atom<Self> = transmute(fat_ptr);
        let atom_ref: &Atom<Self> = fat_ptr.as_ref().unwrap();

        Ok(atom_ref)
    }

    pub fn was_data_written<'a, W>(writer: &mut W) -> bool
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        (writer.get_header().size as usize) > size_of::<H>()
    }

    pub fn push<'a, W>(writer: &mut W, value: T) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        unsafe { writer.write_sized(&value, false)? };
        Ok(())
    }

    pub fn append<'a, W>(writer: &mut W, slice: &[T]) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        let data = unsafe {
            std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice))
        };
        unsafe { writer.write_raw(data, false)? };
        Ok(())
    }
}
