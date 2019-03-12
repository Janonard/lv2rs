use crate::atom::{AtomHeader, AtomBody};
use std::ptr::null_mut;
use std::marker::PhantomData;

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
}