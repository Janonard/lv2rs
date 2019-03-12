use crate::atom::*;
use crate::writer::RawWriter;
use std::marker::PhantomData;
use std::ptr::null_mut;

pub struct AtomOutputPort<A: AtomBody + Clone + ?Sized> {
    atom: *mut AtomHeader,
    phantom: PhantomData<A>,
}

impl<A: AtomBody + Clone + ?Sized> AtomOutputPort<A> {
    pub fn new() -> Self {
        Self {
            atom: null_mut(),
            phantom: PhantomData,
        }
    }

    pub fn connect_port(&mut self, atom: *mut AtomHeader) {
        self.atom = atom;
    }

    pub fn get_writer<'a>(&'a mut self) -> Result<RawWriter<'a>, ()> {
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
        Ok(RawWriter::new(data))
    }
}
