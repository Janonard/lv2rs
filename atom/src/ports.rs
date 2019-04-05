//! Safer wrappers for raw atom IO.
//!
//! The wrappers provided by this module increase the safety and usability of atom IO.
use crate::atom::*;
use crate::frame::RootFrame;
use std::marker::PhantomData;
use std::ptr::{null, null_mut};

/// Wrapper for atom writing operations.
pub struct AtomOutputPort<A: AtomBody + ?Sized> {
    atom: *mut Atom,
    phantom: PhantomData<A>,
}

/// Errors that may occur when calling [`AtomOutputPort::write_atom_body`](struct.AtomOuputPort.html#method.write_atom_body)
#[derive(Debug)]
pub enum WriteAtomError {
    /// The internal pointer points to zero.
    ///
    /// Maybe `connect_port` is not implemented correctly?
    NullPointer,
    /// The host hasn't allocated enough memory to initialize the atom.
    InsufficientSpace,
}

#[derive(Debug)]
/// Error that may occur when calling [`AtomInputPort::get_atom_body`](struct.AtomInputPort.html#method.get_atom_body).
pub enum GetAtomError {
    /// The internal pointer points to zero.
    ///
    /// Maybe `connect_port` is not implemented correctly?
    NullPointer,
    /// Widening the atom header failed.
    GetBody(GetBodyError),
}

impl<A: AtomBody + ?Sized> AtomOutputPort<A> {
    /// Create a new port.
    ///
    /// Please note that the newly created port wil point to null and therefore,
    /// [`write_atom_body`](#method.write_atom_body) will yield undefined behaviour.
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
    pub fn connect_port(&mut self, atom: *mut Atom) {
        self.atom = atom;
    }

    /// Write an atom to the internal atom pointer.
    ///
    /// This method will create a [`RootFrame`](../frame/struct.RootFrame.html) and initialize the
    /// body. For [scalar atoms](../scalar/index.html), this is all you can and need to
    /// do. For all other atoms, you can write additional data using the `RootFrame`.
    ///
    /// This method is unsafe since it dereferences the raw, internal pointer and therefore could
    /// yield undefined behaviour. Make sure that your plugin's `connect_port` method calls this
    /// port's [`connect_port`](#method.connect_port) method correctly!
    pub unsafe fn write_atom_body<'a>(
        &'a mut self,
        parameter: &A::InitializationParameter,
        urids: &mut urid::CachedMap,
    ) -> Result<RootFrame<'a, A>, WriteAtomError> {
        let header = match self.atom.as_mut() {
            Some(header) => header,
            None => return Err(WriteAtomError::NullPointer),
        };
        let data = std::slice::from_raw_parts_mut(self.atom as *mut u8, header.size() as usize);
        let mut frame =
            RootFrame::new(data, urids).map_err(|_| WriteAtomError::InsufficientSpace)?;
        A::initialize_body(&mut frame, parameter, urids)
            .map_err(|_| WriteAtomError::InsufficientSpace)?;
        Ok(frame)
    }
}

/// Wrapper for atom reading operations.
pub struct AtomInputPort<A: AtomBody + ?Sized> {
    atom: *const Atom,
    phantom: PhantomData<A>,
}

impl<A: AtomBody + ?Sized> AtomInputPort<A> {
    /// Create a new port.
    ///
    /// Please note that the newly created port wil point to null and therefore,
    /// [`write_atom_body`](#method.write_atom_body) will yield undefined behaviour.
    pub fn new() -> Self {
        Self {
            atom: null(),
            phantom: PhantomData,
        }
    }

    /// Set the internal atom pointer.
    ///
    /// As implied by the name, this method should be called by an atom's `connect_port`. However,
    /// you have to cast the passed pointer to the correct type!
    pub fn connect_port(&mut self, atom: *const Atom) {
        self.atom = atom;
    }

    /// Dereference the internal raw pointer to an atom body reference.
    ///
    /// This method is unsafe since it dereferences the raw, internal pointer and therefore could
    /// yield undefined behaviour. Make sure that your plugin's `connect_port` method calls this
    /// port's [`connect_port`](#method.connect_port) method correctly!
    pub unsafe fn get_atom_body(&self, urids: &mut urid::CachedMap) -> Result<&A, GetAtomError> {
        let atom = match self.atom.as_ref() {
            Some(atom) => atom,
            None => return Err(GetAtomError::NullPointer),
        };
        atom.get_body(urids)
            .map_err(|err| GetAtomError::GetBody(err))
    }
}
