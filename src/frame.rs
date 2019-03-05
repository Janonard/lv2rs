use crate::atom::{Atom, AtomBody, AtomHeader};
use std::mem::size_of_val;

pub trait WritingFrame {
    fn write<'a, A: AtomBody + Clone>(
        &'a mut self,
        atom: Atom<A>,
    ) -> Result<(&'a mut Atom<A>, usize), ()>;
}

pub struct RootFrame<'a> {
    data: &'a mut [u8],
    used_space: usize,
}

impl<'a> RootFrame<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            data: data,
            used_space: 0,
        }
    }
}

impl<'a> WritingFrame for RootFrame<'a> {
    fn write<'b, A: AtomBody + Clone>(
        &'b mut self,
        atom: Atom<A>,
    ) -> Result<(&'b mut Atom<A>, usize), ()> {
        // Create a byte slice containing the atom that is about to be written.
        let origin_atom_space: &[u8] = unsafe {
            std::slice::from_raw_parts(&atom as *const Atom<A> as *const u8, size_of_val(&atom))
        };

        // Calculate the new amount of used space, including the atom and padding for 64-bit alignment.
        let new_used_space: usize = self.used_space + origin_atom_space.len();
        let padding = (new_used_space) % 8;
        let written_space = origin_atom_space.len() + padding;
        let new_used_space: usize = new_used_space + padding;
        if new_used_space > self.data.len() {
            return Err(());
        }

        // Chop of the space that's already used and that's still free.
        let free_space = self.data.split_at_mut(self.used_space).1;
        let target_atom_space = free_space.split_at_mut(origin_atom_space.len()).0;

        // Copy the data.
        target_atom_space.copy_from_slice(origin_atom_space);

        // Safe the new used space.
        self.used_space = new_used_space;

        // Construct a reference to the newly written atom.
        let written_atom =
            unsafe { (target_atom_space.as_mut_ptr() as *mut Atom<A>).as_mut() }.unwrap();
        Ok((written_atom, written_space))
    }
}

pub struct AtomFrame<'a, 'b, F: WritingFrame> {
    atom: &'a mut AtomHeader,
    parent_frame: &'b mut F,
}

impl<'a, 'b, F: WritingFrame> WritingFrame for AtomFrame<'a, 'b, F> {
    fn write<'c, A: AtomBody + Clone>(
        &'c mut self,
        atom: Atom<A>,
    ) -> Result<(&'c mut Atom<A>, usize), ()> {
        match self.parent_frame.write(atom) {
            Ok((written_atom, written_space)) => {
                self.atom.size += written_space as i32;
                Ok((written_atom, written_space))
            }
            Err(_) => Err(()),
        }
    }
}
