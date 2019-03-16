use crate::atom::*;
use crate::frame::{RootFrame, WritingFrame};
use crate::uris::MappedURIDs;
use std::marker::PhantomData;
use std::ptr::null_mut;

pub struct AtomOutputPort<A: AtomBody + ?Sized> {
    atom: *mut AtomHeader,
    phantom: PhantomData<A>,
}

impl<A: AtomBody + ?Sized> AtomOutputPort<A> {
    pub fn new() -> Self {
        Self {
            atom: null_mut(),
            phantom: PhantomData,
        }
    }

    pub fn connect_port(&mut self, atom: *mut AtomHeader) {
        self.atom = atom;
    }

    pub fn create_root_frame<'a>(&'a mut self) -> Result<RootFrame<'a, A>, ()> {
        let data_size: usize = match unsafe { self.atom.as_ref() } {
            Some(header) => header.size as usize,
            None => return Err(()),
        };
        let data = unsafe {
            std::slice::from_raw_parts_mut(
                self.atom as *mut u8,
                data_size + std::mem::size_of::<AtomHeader>(),
            )
        };
        Ok(RootFrame::new(data))
    }
}

impl<A: AtomBody + Sized> AtomOutputPort<A> {
    pub fn write_sized_atom(
        &mut self,
        urids: &MappedURIDs,
        parameter: &A::InitializationParameter,
    ) -> Result<(), ()> {
        let mut frame = self.create_root_frame()?;
        unsafe { frame.create_atom::<A>(urids, parameter)? };
        Ok(())
    }
}
