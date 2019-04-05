//! Raw writing access to chunks of memory.
//!
//! This module contains the basic handles for writing atoms to a chunk of memory. This is done
//! using a framing approach: Every atom is managed by a [`WritingFrame`](trait.WritingFrame.html)
//! which is able to write data into memory and update the size noted in the atom header.
//! Nested atoms can also be created by every writing frame using the
//! [`create_nested_frame`](trait.WritingFrameExt.html#method.create_nested_frame) method.
//! These nested frames additionally account for padding when dropped.
use crate::atom::*;
use std::marker::PhantomData;
use std::mem::size_of;

/// Basic functionality of a writing frame.
///
/// A writing frame manages an atom header and is able to append raw data to the atom. Additional
/// methods are out-sourced to the [`WritingFrameExt`](trait.WritingFrameExt.html) trait since it
/// contains generic methods and therefore can not be turned into trait objects. But don't worry,
/// every type that implements the `WritingFrame` trait automatically implements the
/// `WritingFrameExt` too.
pub trait WritingFrame<'a> {
    /// Try to write out a slice of bytes into the atom space.
    ///
    /// The data will be written directly after the previously written data and no padding will be
    /// applied.
    ///
    /// If writing was successfull, a slice with the written data is returned. In case
    /// of insufficient atom space, this function will return an error.
    ///
    /// Also, this function is unsafe since it does not check the resulting atom for consistency.
    /// You have to know what you are doing!
    unsafe fn write_raw(&mut self, data: &[u8]) -> Result<&'a mut [u8], ()>;

    /// Return an immutable reference to the managed atom header.
    fn get_atom(&self) -> &Atom;
}

/// Extended functionality for writing frames.
///
/// This extension trait makes [`WritingFrame`s](trait.WritingFrame.html) really usefull. First of
/// all, this trait is generic for the managed atom type. This means that one can implement
/// additional functions for writing frames that manage a specific type of atom body.
///
/// Also, you can write sized objects to the atom using the [`write_sized`](#method.write_sized)
/// method without needing to turn a reference to a `u8` slice. Last but not least, it can create
/// new, nested writing frames using [`create_nested_frame`](#method.create_nested_frame).
pub trait WritingFrameExt<'a, A: AtomBody + ?Sized>: WritingFrame<'a> + Sized {
    /// Try to write a sized object into the atom space.
    ///
    /// The data will be written directly after the previously written data and no padding will be
    /// applied.
    ///
    /// If writing was successfull, a reference to the writen object is returned. In case
    /// of insufficient atom space, this function will return an error.
    ///
    /// Also, this function is unsafe since it does not check the resulting structure for consistency.
    /// You have to know what you are doing!
    unsafe fn write_sized<T: Sized>(&mut self, object: &T) -> Result<&'a mut T, ()> {
        let data: &[u8] =
            std::slice::from_raw_parts(object as *const T as *const u8, size_of::<T>());
        match self.write_raw(data) {
            Ok(data) => Ok((data.as_mut_ptr() as *mut T).as_mut().unwrap()),
            Err(_) => Err(()),
        }
    }

    /// Create a new atom header and return a nested writing frame for it.
    ///
    /// This function can be used for container atoms. Please note that this function only writes
    /// the atom header and does not initialize the atom body.
    ///
    /// Also, this function is unsafe since one can mess up atom structures.
    unsafe fn create_nested_frame<'b, C: AtomBody + ?Sized>(
        &'b mut self,
        urids: &mut urid::CachedMap,
    ) -> Result<NestedFrame<'b, 'a, C>, ()> {
        let writer = NestedFrame {
            atom: Atom::write_empty_header(self, urids.map(C::get_uri()))?,
            parent: self,
            phantom: PhantomData,
        };

        Ok(writer)
    }

    /// Try to get a reference to the body from our atom header.
    ///
    /// This is just a shortcut for `A::widen_ref(frame.get_header(), urids)`.
    unsafe fn get_atom_body<'b>(
        &'b self,
        urids: &mut urid::CachedMap,
    ) -> Result<&'b A, GetBodyError> {
        self.get_atom().get_body(urids)
    }
}

/// The ground level writing frame.
///
/// This writing frame manages the first atom and actually has access to the data. It is created by
/// the [`AtomOutputPort`](../ports/struct.AtomOutputPort.html) and simply acts like any
/// [`WritingFrame`](trait.WritingFrame.html).
pub struct RootFrame<'a, A: AtomBody + ?Sized> {
    atom: &'a mut Atom,
    free_data: &'a mut [u8],
    phantom: PhantomData<A>,
}

impl<'a, A: AtomBody + ?Sized> RootFrame<'a, A> {
    /// Try to create a new root frame.
    ///
    /// All you need to create a root frame is a slice of writable memory and a way to retrieve
    /// the URID of the managed atom. Then, this function will initialize the header in the
    /// beginning of the slice and create the frame.
    ///
    /// If the slice is not big enough to hold the atom header, this function returns an `Err`.
    pub fn new(free_space: &'a mut [u8], urids: &mut urid::CachedMap) -> Result<Self, ()> {
        let atom_size = std::mem::size_of::<Atom>();
        if free_space.len() < atom_size {
            return Err(());
        }

        let atom_ptr = free_space.as_mut_ptr() as *mut Atom;
        let atom = unsafe { atom_ptr.as_mut() }.unwrap();
        let data = unsafe {
            std::slice::from_raw_parts_mut(atom_ptr.add(1) as *mut u8, free_space.len() - atom_size)
        };

        *(atom.mut_atom_type()) = urids.map(A::get_uri());
        *(atom.mut_size()) = 0;
        Ok(RootFrame {
            atom: atom,
            free_data: data,
            phantom: PhantomData,
        })
    }
}

impl<'a, A: AtomBody + ?Sized> WritingFrame<'a> for RootFrame<'a, A> {
    unsafe fn write_raw(&mut self, data: &[u8]) -> Result<&'a mut [u8], ()> {
        if data.len() > self.free_data.len() {
            return Err(());
        }

        let data_ptr = self.free_data.as_mut_ptr();
        let n_free_bytes = self.free_data.len() - data.len();

        let target_data = std::slice::from_raw_parts_mut(data_ptr, data.len());
        let free_data = std::slice::from_raw_parts_mut(data_ptr.add(data.len()), n_free_bytes);

        target_data.copy_from_slice(data);
        self.free_data = free_data;
        *(self.atom.mut_size()) += data.len() as i32;

        // Construct a reference to the newly written atom.
        Ok(target_data)
    }

    fn get_atom(&self) -> &Atom {
        self.atom
    }
}

impl<'a, A: AtomBody + ?Sized> WritingFrameExt<'a, A> for RootFrame<'a, A> {}

/// A writing frame managing nested atoms.
///
/// Unlike the [`RootFrame`](struct.RootFrame.html), which really manages memory, this frame only
/// forwards writing calls to the next frame in the hierarchy and updates the atom header
/// accordingly. Additionally, this frame will assure for padding when dropped. These padding bytes
/// will not be included in the top level header, only in the surrounding ones.
///
/// Nested frames can only be created with the
/// [`create_nested_frame`](trait.WritingFrameExt.html#method.create_nested_frame) method.
pub struct NestedFrame<'a, 'b, A>
where
    A: AtomBody + ?Sized,
{
    atom: &'b mut Atom,
    parent: &'a mut WritingFrame<'b>,
    phantom: PhantomData<A>,
}

impl<'a, 'b, A> Drop for NestedFrame<'a, 'b, A>
where
    A: AtomBody + ?Sized,
{
    fn drop(&mut self) {
        let pad: &[u8] = match 8 - (self.parent.get_atom().size() % 8) {
            1 => &[0; 1],
            2 => &[0; 2],
            3 => &[0; 3],
            4 => &[0; 4],
            5 => &[0; 5],
            6 => &[0; 6],
            7 => &[0; 7],
            8 => &[0; 0],
            _ => panic!("invalid pad size"),
        };
        unsafe { self.parent.write_raw(pad).unwrap() };
    }
}

impl<'a, 'b, A> WritingFrame<'b> for NestedFrame<'a, 'b, A>
where
    A: AtomBody + ?Sized,
{
    unsafe fn write_raw(&mut self, data: &[u8]) -> Result<&'b mut [u8], ()> {
        let data = self.parent.write_raw(data)?;
        *(self.atom.mut_size()) += data.len() as i32;
        Ok(data)
    }

    fn get_atom(&self) -> &Atom {
        self.atom
    }
}

impl<'a, 'b, A: AtomBody + ?Sized> WritingFrameExt<'b, A> for NestedFrame<'a, 'b, A> {}
