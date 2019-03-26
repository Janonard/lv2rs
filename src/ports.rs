//! Safer wrappers for raw atom IO.
//!
//! The wrappers provided by this module increase the safety and usability of atom IO.
use crate::atom::*;
use crate::frame::RootFrame;
use std::marker::PhantomData;
use std::ptr::{null, null_mut};
use urid::URID;

/// Wrapper for atom writing operations.
pub struct AtomOutputPort<A: AtomBody + ?Sized> {
    atom: *mut AtomHeader,
    phantom: PhantomData<A>,
}

impl<A: AtomBody + ?Sized> AtomOutputPort<A> {
    /// Create a new port.
    ///
    /// Please note that the newly created port wil point to null and therefore,
    /// [`write_atom`](#method.write_atom) will yield undefined behaviour.
    pub fn new() -> Self {
        Self {
            atom: null_mut(),
            phantom: PhantomData,
        }
    }

    /// Set the internal atom pointer.
    ///
    /// As implied by the name, this method should be called by an atom's `connect_port`. However,
    /// you have to cast the passed pointer to the correct type!
    pub fn connect_port(&mut self, atom: *mut AtomHeader) {
        self.atom = atom;
    }

    /// Write an atom to the internal atom pointer.
    ///
    /// This method will create a [`RootFrame`](../frame/struct.RootFrame.html) and initialize the
    /// body a `Atom<A>`. For [scalar atoms](../scalar/index.html), this is all you can and need to
    /// do. For all other atoms, you can write additional data using the `RootFrame`.
    ///
    /// If the host doesn't provide enough space to write the atom, an `Err` will be returned.
    ///
    /// This method is unsafe since it dereferences the raw, internal pointer and therefore could
    /// yield undefined behaviour. Make sure that your plugin's `connect_port` method calls this
    /// port's [`connect_port`](#method.connect_port) method correctly!
    pub unsafe fn write_atom<'a>(
        &'a mut self,
        parameter: &A::InitializationParameter,
        urids: &mut urid::CachedMap,
    ) -> Result<RootFrame<'a, A>, ()> {
        let header = match self.atom.as_mut() {
            Some(header) => header,
            None => return Err(()),
        };
        let data = std::slice::from_raw_parts_mut(self.atom as *mut u8, header.size as usize);
        let mut frame = RootFrame::new(data, urids)?;
        A::initialize_body(&mut frame, parameter, urids)?;
        Ok(frame)
    }
}

/// Wrapper for atom reading operations.
pub struct AtomInputPort<A: AtomBody + ?Sized> {
    atom: *const AtomHeader,
    type_urid: URID,
    phantom: PhantomData<A>,
}

impl<A: AtomBody + ?Sized> AtomInputPort<A> {
    /// Create a new port.
    ///
    /// Please note that the newly created port wil point to null and therefore,
    /// [`write_atom`](#method.write_atom) will yield undefined behaviour.
    pub fn new(urids: &mut urid::CachedMap) -> Self {
        Self {
            atom: null(),
            type_urid: urids.map(A::get_uri()),
            phantom: PhantomData,
        }
    }

    /// Set the internal atom pointer.
    ///
    /// As implied by the name, this method should be called by an atom's `connect_port`. However,
    /// you have to cast the passed pointer to the correct type!
    pub fn connect_port(&mut self, atom: *const AtomHeader) {
        self.atom = atom;
    }

    /// Dereference the internal raw pointer to a atom reference.
    ///
    /// If the internal pointer points to null or if the atom is illformed, this method will return
    /// an `Err`.
    ///
    /// This method is unsafe since it dereferences the raw, internal pointer and therefore could
    /// yield undefined behaviour. Make sure that your plugin's `connect_port` method calls this
    /// port's [`connect_port`](#method.connect_port) method correctly!
    pub unsafe fn get_atom(&self, urids: &mut urid::CachedMap) -> Result<&Atom<A>, ()> {
        let atom = match self.atom.as_ref() {
            Some(atom) => atom,
            None => return Err(()),
        };
        if atom.atom_type != self.type_urid {
            return Err(());
        }
        A::widen_ref(atom, urids)
    }
}
