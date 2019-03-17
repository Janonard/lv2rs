use crate::atom::*;
use crate::frame::RootFrame;
use crate::uris::MappedURIDs;
use std::marker::PhantomData;
use std::mem::size_of;
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

    pub fn write_atom<'a>(
        &'a mut self,
        parameter: &A::InitializationParameter,
        urids: &MappedURIDs,
    ) -> Result<RootFrame<'a, A>, ()> {
        let header = match unsafe { self.atom.as_mut() } {
            Some(header) => header,
            None => return Err(()),
        };
        let header_size = size_of::<AtomHeader>();
        let data = unsafe {
            std::slice::from_raw_parts_mut(
                (self.atom as *mut u8).add(header_size),
                header.size as usize,
            )
        };
        let mut frame = RootFrame::new(header, data, urids);
        A::initialize_body(&mut frame, parameter)?;
        Ok(frame)
    }
}
