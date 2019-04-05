//! Fundamental type definitions.
use crate::frame::{WritingFrame, WritingFrameExt};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_int;
use urid::URID;

/// Generic type combining an atom header with a body.
///
/// This type is mostly used to interpret immutable references to atoms. Mutable references to atoms
/// appear rarely since one can set type and size to invalid numbers and owning atoms is even
/// rarer since most atoms are dynamically sized types (DSTs).
///
/// However, it is very handy for immutable references and many reading methods work directly on it.
/// The `Atom` struct is also used to interpret atom pointers from C, since it is  `repr(C)`
#[repr(C)]
pub struct Atom {
    pub size: c_int,
    pub atom_type: URID,
}

impl Atom {
    /// Return the size of the body, as noted in the header.
    pub fn body_size(&self) -> usize {
        self.size as usize
    }

    /// Return the type of the body, as noted in the header.
    pub fn body_type(&self) -> URID {
        self.atom_type
    }

    pub unsafe fn get_raw_body(&self) -> &[u8] {
        std::slice::from_raw_parts(
            (self as *const Atom).add(1) as *const u8,
            self.size as usize,
        )
    }

    pub unsafe fn get_body<A: AtomBody + ?Sized>(
        &self,
        urids: &mut urid::CachedMap,
    ) -> Result<&A, GetBodyError> {
        if self.atom_type != urids.map(A::get_uri()) {
            return Err(GetBodyError::WrongURID);
        }
        let raw_body = self.get_raw_body();
        A::create_ref(raw_body).map_err(|_| GetBodyError::MalformedAtom)
    }
}

/// Errors that may occur when calling [`AtomBody::widen_ref`](trait.AtomBody.html#tymethod.widen_ref).
#[derive(Debug)]
pub enum GetBodyError {
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

    /// Try to create a `Self` reference from an atom header.
    ///
    /// Also, this method is unsafe since you can not check if the memory space after the header is
    /// actually allocated. Therefore, this method could have undefined behaviour. However, you can
    /// assume that the size in the header is valid, since you cannot defend yourself from the
    /// opposite.
    unsafe fn create_ref<'a>(raw_body: &'a [u8]) -> Result<&'a Self, ()>;
}

/// Iterator over atoms.
///
/// This iterator takes a slice of bytes and tries to iterate over all atoms in this slice. If
/// there is an error while iterating, iteration will end.
pub struct AtomIterator<'a, H: 'static + Sized> {
    data: &'a [u8],
    position: usize,
    phantom: PhantomData<H>,
}

impl<'a, H: 'static + Sized> AtomIterator<'a, H> {
    /// Create a new chunk iterator.
    pub fn new(data: &'a [u8]) -> Self {
        AtomIterator {
            data: data,
            position: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, H: 'static + Sized> Iterator for AtomIterator<'a, H> {
    type Item = (&'a H, &'a Atom);

    fn next(&mut self) -> Option<(&'a H, &'a Atom)> {
        use std::mem::size_of;

        // pad to the next 64-bit aligned position, if nescessary.
        if self.position % 8 != 0 {
            self.position += 8 - self.position % 8;
        }
        if self.position >= self.data.len() {
            return None;
        }

        let data = &self.data[self.position..];
        if data.len() < size_of::<H>() + size_of::<Atom>() {
            return None;
        }

        let pre_header_ptr = data.as_ptr() as *const H;
        let pre_header = unsafe { pre_header_ptr.as_ref() }?;
        let atom_ptr = unsafe { pre_header_ptr.add(1) } as *const Atom;
        let atom = unsafe { atom_ptr.as_ref() }?;

        // Apply the package of pre-header, atom and data to our position in the array.
        self.position += size_of::<H>() + size_of::<Atom>() + atom.size as usize;

        if self.position <= self.data.len() {
            Some((pre_header, atom))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::atom::*;

    #[test]
    fn test_chunk_iterator() {
        struct TestPrefix {
            value: u64,
        }

        // ##################
        // creating the data.
        // ##################
        let mut data = Box::new([0u8; 256]);
        let ptr = data.as_mut().as_mut_ptr();

        // First prefix.
        let mut ptr = ptr as *mut TestPrefix;
        unsafe {
            let mut_ref = ptr.as_mut().unwrap();
            mut_ref.value = 650000;
            // No padding needed, TestPrefix is eight bytes long.
            ptr = ptr.add(1);
        }

        // First atom. We will fit a u8 after it, because it requires seven padding bytes, which
        // is an important edge case.
        let mut ptr = ptr as *mut Atom;
        unsafe {
            let mut_ref = ptr.as_mut().unwrap();
            mut_ref.atom_type = 42;
            mut_ref.size = 1;
            ptr = ptr.add(1);
        }
        let mut ptr = ptr as *mut u8;
        unsafe {
            let mut_ref = ptr.as_mut().unwrap();
            *mut_ref = 17;
            ptr = ptr.add(1);
        }

        // Padding and second prefix.
        let mut ptr = unsafe { ptr.add(7) } as *mut TestPrefix;
        unsafe {
            let mut_ref = ptr.as_mut().unwrap();
            mut_ref.value = 4711;
            // No padding needed, TestPrefix is eight bytes long.
            ptr = ptr.add(1);
        }

        // Second atom.
        let mut ptr = ptr as *mut Atom;
        unsafe {
            let mut_ref = ptr.as_mut().unwrap();
            mut_ref.atom_type = 10;
            mut_ref.size = 1;
            ptr = ptr.add(1);
        }
        let ptr = ptr as *mut u8;
        unsafe {
            let mut_ref = ptr.as_mut().unwrap();
            *mut_ref = 4;
        }

        // #####################
        // Testing the iterator.
        // #####################
        let mut iter: AtomIterator<TestPrefix> = AtomIterator::new(data.as_ref());

        // First atom
        let (prefix, atom) = iter.next().unwrap();
        assert_eq!(650000, prefix.value);
        assert_eq!(42, atom.atom_type);
        assert_eq!(1, atom.size);
        assert_eq!(17, unsafe { atom.get_raw_body() }[0]);

        // Second atom.
        let (prefix, atom) = iter.next().unwrap();
        assert_eq!(4711, prefix.value);
        assert_eq!(10, atom.atom_type);
        assert_eq!(1, atom.size);
        assert_eq!(4, unsafe { atom.get_raw_body() }[0]);
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

        /// Internal method to create an atom body reference.
        ///
        /// This method will check if the type URID is correct and if the size is big enough to
        /// contain the body header. The rest of the size will be interpreted as data.
        pub unsafe fn __create_ref<'a>(raw_data: &'a [u8]) -> Result<&'a Self, ()> {
            let array_header_size = size_of::<H>();
            if raw_data.len() < array_header_size {
                return Err(());
            }

            let tail_size = raw_data.len() - size_of::<H>();
            if tail_size % size_of::<T>() != 0 {
                return Err(());
            }
            let tail_len = tail_size / size_of::<T>();

            // This is were the unsafe things happen!
            // We know the length of the string, therefore we can create a fat pointer to the atom.
            let self_ptr: (*const u8, usize) = (raw_data.as_ptr(), tail_len);
            let self_ptr: *const Self = transmute(self_ptr);
            let self_ref: &Self = self_ptr.as_ref().unwrap();

            Ok(self_ref)
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
