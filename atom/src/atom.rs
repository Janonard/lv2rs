//! Fundamental type definitions.
use crate::frame::{WritingFrame, WritingFrameExt};
use std::ffi::CStr;
use std::os::raw::c_int;
use urid::URID;

/// General atom type information container.
///
/// An atom header contains the size and the type URID of the atom. You can use this struct to
/// interpret an atom or atom header pointer from C since it is `repr(C)`.
#[derive(Clone)]
#[repr(C)]
pub struct AtomHeader {
    pub size: c_int,
    pub atom_type: URID,
}

impl AtomHeader {
    /// Try to widen a `AtomHeader` reference to a `Atom` reference.
    ///
    /// Also, this method is unsafe since it can not check if the memory space after the header is
    /// actually allocated. Therefore, this method could have undefined behaviour.
    pub unsafe fn widen_ref<A: AtomBody + ?Sized>(
        &self,
        urids: &mut urid::CachedMap,
    ) -> Result<&Atom<A>, WidenRefError> {
        A::widen_ref(self, urids)
    }
}

/// Errors that may occur when calling [`AtomBody::widen_ref`](trait.AtomBody.html#tymethod.widen_ref).
#[derive(Debug)]
pub enum WidenRefError {
    /// The URID noted in the atom header is wrong.
    ///
    /// Maybe you tried to use the wrong atom type?
    WrongURID,
    /// The atom is malformed.
    ///
    /// You can't do much about it; This is the fault of other plugins.
    MalformedAtom,
}

/// Abstraction of atom bodies.
///
/// Atom bodies can be very different in size and shape and therefore, this trait contains only a
/// small set of things atoms are capable of.
///
/// ## Implementing your own atom bodies.
///
/// First of all, you shouldn't. The set of included atom bodies is enough to express almost any
/// information. On the other hand, if you really need to implement a new atom body, just implement
/// this trait. This will give you most of the features you will need. However, this trait only
/// lets you initialize a body. It does not give you means to extend it afterwards. If you want to
/// do that, you should create an extension trait for [`WritingFrame`s](../frame/trait.WritingFrame.html),
/// just like the [`TupleWritingFrame`](../tuple/trait.TupleWritingFrame.html).
pub trait AtomBody {
    /// The type of the parameter for [`initialize_body`](#tymethod.initialize_body)
    ///
    /// Since Rust does not support generic associated types yet, you can not use references here.
    /// However, since `initialize_body` will receive a reference of this type, you can place
    /// unsized types in here, like slices.
    type InitializationParameter: ?Sized;

    /// Return the URI of the atom type.
    fn get_uri() -> &'static CStr;

    /// Write out a basic but valid atom body.
    ///
    /// Implementors should use the writing frame to write out general information about the atom,
    /// like body-specific headers or, in the case of scalars, the value itself. Please note that
    /// * The [`AtomHeader`](struct.AtomHeader.html) was already written, you do not need to write
    /// it yourself.
    /// * You cannot alter the data after it was written. Once this method call is over, you only have
    /// reading access to it by using the [`get_atom`](../frame/trait.WritingFrameExt.html#method.get_atom)
    /// method of the writing frame.
    /// * The result must be a valid atom. You may not rely on future calls to make it valid.
    /// * In most cases, you don't need to include padding. If padding is required, the writer will
    /// include it when it is dropped.
    /// * You do not need (and definitely should not try) to update the atom header for the new
    /// size. The writer will keep track of that.
    /// * Your implementation should be implemented in a way that it can only return `Err` in cases
    /// of insufficient memory.
    ///
    /// This method is unsafe since it can tamper if the integrity of the atom structure, for example
    /// if called twice.
    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        parameter: &Self::InitializationParameter,
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>;

    /// Try to widen the header reference to a atom reference.
    ///
    /// Also, this method is unsafe since you can not check if the memory space after the header is
    /// actually allocated. Therefore, this method could have undefined behaviour. However, you can
    /// assume that the size in the header is valid, since you cannot defend yourself from the
    /// opposite.
    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, WidenRefError>;
}

/// Generic type combining an atom header with a body.
///
/// This type is mostly used to interpret immutable references to atoms. Mutable references to atoms
/// appear rarely since one can set type and size to invalid numbers and owning atoms is even
/// rarer since most atoms are dynamically sized types (DSTs).
///
/// However, it is very handy for immutable references and many reading methods work directly on it.
/// The `Atom` struct is also used to interpret atom pointers from C, since it is  `repr(C)`
#[repr(C)]
pub struct Atom<A: AtomBody + ?Sized> {
    pub header: AtomHeader,
    pub body: A,
}

impl<A: AtomBody + ?Sized> Atom<A> {
    /// Return the size of the body, as noted in the header.
    pub fn body_size(&self) -> usize {
        self.header.size as usize
    }

    /// Return the type of the body, as noted in the header.
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

/// Special templates for dynamically sized atoms.
pub mod array {
    use crate::atom::*;
    use crate::frame::{WritingFrame, WritingFrameExt};
    use std::mem::{size_of, transmute};

    /// A header of an `ArrayAtomBody`.
    ///
    /// Many atoms have an additional body header and this trait represents said headers.
    pub trait ArrayAtomHeader: Sized {
        /// Type of the parameter for [`initialize`](#tymethod.initialize).
        type InitializationParameter: ?Sized;

        /// Write out the array atom header.
        ///
        /// The same rules from
        /// [`AtomBody::initialize_body`](../trait.AtomBody.html#tymethod.initialize_body) apply.
        unsafe fn initialize<'a, W, T>(
            writer: &mut W,
            parameter: &Self::InitializationParameter,
            urids: &mut urid::CachedMap,
        ) -> Result<(), ()>
        where
            T: 'static + Sized + Copy,
            ArrayAtomBody<Self, T>: AtomBody,
            W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>;
    }

    impl ArrayAtomHeader for () {
        type InitializationParameter = ();

        unsafe fn initialize<'a, W, T>(_: &mut W, _: &(), _: &mut urid::CachedMap) -> Result<(), ()>
        where
            T: 'static + Sized + Copy,
            ArrayAtomBody<Self, T>: AtomBody,
            W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
        {
            Ok(())
        }
    }

    /// Abstract type for dynamically sized atoms.
    ///
    /// Many dynamically sized atoms share a lot of their behaviour as well as their raw
    /// representation. Therefore, they are abstracted to this struct that contains a header and
    /// an array of sized items.
    ///
    /// If you don't want to have a header, you can use `()` instead.
    ///
    /// Not all combinations of header and data items are atom bodies, but many methods rely on
    /// the combination being an atom body.
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
        /// Internal method to initialize the body.
        ///
        /// It simply calls the initialization method of the header, the data array will be left
        /// empty.
        pub unsafe fn __initialize_body<'a, W>(
            writer: &mut W,
            parameter: &H::InitializationParameter,
            urids: &mut urid::CachedMap,
        ) -> Result<(), ()>
        where
            W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
        {
            H::initialize(writer, parameter, urids)
        }

        /// Internal method to widen an atom header reference.
        ///
        /// This method will check if the type URID is correct and if the size is big enough to
        /// contain the body header. The rest of the size will be interpreted as data.
        pub unsafe fn __widen_ref<'a>(
            header: &'a AtomHeader,
            urids: &mut urid::CachedMap,
        ) -> Result<&'a Atom<Self>, WidenRefError> {
            if header.atom_type != urids.map(Self::get_uri()) {
                return Err(WidenRefError::WrongURID);
            }

            let body_size = header.size as usize;
            let array_header_size = size_of::<H>();
            if body_size < array_header_size {
                return Err(WidenRefError::MalformedAtom);
            }

            let body_size = body_size - array_header_size;
            if body_size % size_of::<T>() != 0 {
                return Err(WidenRefError::MalformedAtom);
            }
            let vector_len: usize = body_size / size_of::<T>();

            // This is were the unsafe things happen!
            // We know the length of the string, therefore we can create a fat pointer to the atom.
            let fat_ptr: (*const AtomHeader, usize) = (header as *const AtomHeader, vector_len);
            let fat_ptr: *const Atom<Self> = transmute(fat_ptr);
            let atom_ref: &Atom<Self> = fat_ptr.as_ref().unwrap();

            Ok(atom_ref)
        }

        /// Push another value to the data array.
        ///
        /// In case of insufficient memory, an `Err` is returned.
        ///
        /// This method assumes that the atom was already initialized, but since can't be checked,
        /// this method is unsafe.
        pub unsafe fn push<'a, W>(writer: &mut W, value: T) -> Result<(), ()>
        where
            W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
        {
            writer.write_sized(&value)?;
            Ok(())
        }

        /// Append a `T` slice to the data.
        ///
        /// In case of insufficient memory, an `Err` is returned.
        ///
        /// This method assumes that the atom was already initialized, but since can't be checked,
        /// this method is unsafe.
        pub unsafe fn append<'a, W>(writer: &mut W, slice: &[T]) -> Result<(), ()>
        where
            W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
        {
            let data = std::slice::from_raw_parts(
                slice.as_ptr() as *const u8,
                std::mem::size_of_val(slice),
            );
            writer.write_raw(data)?;
            Ok(())
        }
    }
}
