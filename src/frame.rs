use crate::atom::AtomHeader;
use std::mem::size_of;

pub trait WritingFrame {
    fn write_raw<'a>(&'a mut self, data: &[u8], padding: bool)
        -> Result<(&'a mut [u8], usize), ()>;

    fn write_sized<T: Sized>(&mut self, object: &T, padding: bool) -> Result<(&mut T, usize), ()> {
        let data: &[u8] =
            unsafe { std::slice::from_raw_parts(object as *const T as *const u8, size_of::<T>()) };
        match self.write_raw(data, padding) {
            Ok((data, written_bytes)) => {
                let object = unsafe { (data.as_mut_ptr() as *mut T).as_mut() }.unwrap();
                Ok((object, written_bytes))
            }
            Err(_) => Err(()),
        }
    }
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
    fn write_raw<'b>(
        &'b mut self,
        data: &[u8],
        padding: bool,
    ) -> Result<(&'b mut [u8], usize), ()> {
        // Calculate the new amount of used space, including the atom and padding for 64-bit alignment.
        let mut new_used_space: usize = self.used_space + data.len();
        let mut written_space = data.len();
        if padding {
            let padding = (new_used_space) % 8;
            new_used_space += padding;
            written_space += padding;
        }
        if new_used_space > self.data.len() {
            return Err(());
        }

        // Chop of the space that's already used and that's still free.
        let free_space = self.data.split_at_mut(self.used_space).1;
        let target_data = free_space.split_at_mut(data.len()).0;

        // Copy the data.
        target_data.copy_from_slice(data);

        // Safe the new used space.
        self.used_space = new_used_space;

        // Construct a reference to the newly written atom.
        Ok((target_data, written_space))
    }
}

pub struct AtomFrame<'a, 'b, F: WritingFrame> {
    atom: &'a mut AtomHeader,
    parent_frame: &'b mut F,
}

impl<'a, 'b, F: WritingFrame> WritingFrame for AtomFrame<'a, 'b, F> {
    fn write_raw<'c>(
        &'c mut self,
        data: &[u8],
        padding: bool,
    ) -> Result<(&'c mut [u8], usize), ()> {
        match self.parent_frame.write_raw(data, padding) {
            Ok((written_atom, written_space)) => {
                self.atom.size += written_space as i32;
                Ok((written_atom, written_space))
            }
            Err(_) => Err(()),
        }
    }
}
