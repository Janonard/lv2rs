use crate::frame::{WritingFrame, WritingFrameExt};
use crate::uris::MappedURIDs;
use std::ffi::CStr;
use std::os::raw::c_int;
use urid::URID;

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

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        parameter: &Self::InitializationParameter,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>;

    unsafe fn widen_ref(header: &AtomHeader) -> Result<&Atom<Self>, ()>;
}

#[derive(Clone)]
#[repr(C)]
pub struct Atom<A: AtomBody + ?Sized> {
    pub header: AtomHeader,
    pub body: A,
}

impl<A: AtomBody + ?Sized> Atom<A> {
    pub fn body_size(&self) -> usize {
        self.header.size as usize
    }

    pub fn body_type(&self) -> URID {
        self.header.atom_type
    }
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

pub mod array {
    use crate::atom::{Atom, AtomBody, AtomHeader};
    use crate::frame::{WritingFrame, WritingFrameExt};
    use std::mem::{size_of, transmute};

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

    impl ArrayAtomHeader for () {
        type InitializationParameter = ();

        fn initialize<'a, W, T>(_: &mut W, _: &()) -> Result<(), ()>
        where
            T: 'static + Sized + Copy,
            ArrayAtomBody<Self, T>: AtomBody,
            W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
        {
            Ok(())
        }
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
            unsafe { writer.write_sized(&value)? };
            Ok(())
        }

        pub fn append<'a, W>(writer: &mut W, slice: &[T]) -> Result<(), ()>
        where
            W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
        {
            let data = unsafe {
                std::slice::from_raw_parts(
                    slice.as_ptr() as *const u8,
                    std::mem::size_of_val(slice),
                )
            };
            unsafe { writer.write_raw(data)? };
            Ok(())
        }
    }
}
