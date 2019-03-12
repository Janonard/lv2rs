use crate::atom::{AtomBody, AtomHeader};
use crate::uris::MappedURIDs;
use std::collections::LinkedList;
use std::marker::PhantomData;
use std::mem::size_of;

pub struct WritingFrame<'a> {
    headers: LinkedList<&'a mut AtomHeader>,
    free_data: &'a mut [u8],
}

impl<'a> WritingFrame<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            headers: LinkedList::new(),
            free_data: data,
        }
    }

    pub fn write_raw(&mut self, data: &[u8]) -> Result<(&'a mut [u8], usize), ()> {
        let n_payload_bytes = data.len();
        let n_padding_bytes = n_payload_bytes % 8;
        if n_payload_bytes + n_padding_bytes > self.free_data.len() {
            return Err(());
        }
        let n_free_bytes = self.free_data.len() - n_payload_bytes - n_padding_bytes;

        // Creating all required slices.
        let data_ptr = self.free_data.as_mut_ptr();

        let target_data = unsafe { std::slice::from_raw_parts_mut(data_ptr, n_payload_bytes) };
        let padding = unsafe {
            std::slice::from_raw_parts_mut(data_ptr.add(n_payload_bytes), n_padding_bytes)
        };
        let free_data = unsafe {
            std::slice::from_raw_parts_mut(
                data_ptr.add(n_payload_bytes + n_padding_bytes),
                n_free_bytes,
            )
        };

        target_data.copy_from_slice(data);
        for byte in padding.iter_mut() {
            *byte = 0;
        }
        std::mem::replace(&mut self.free_data, free_data);

        // updating all headers.
        for header in self.headers.iter_mut() {
            header.size += (n_payload_bytes + n_padding_bytes) as i32;
        }

        // Construct a reference to the newly written atom.
        Ok((target_data, n_payload_bytes + n_padding_bytes))
    }

    pub fn write_sized<T: Sized>(&mut self, object: &T) -> Result<(&'a mut T, usize), ()> {
        let data: &[u8] =
            unsafe { std::slice::from_raw_parts(object as *const T as *const u8, size_of::<T>()) };
        match self.write_raw(data) {
            Ok((data, n_written_bytes)) => {
                let object = unsafe { (data.as_mut_ptr() as *mut T).as_mut() }.unwrap();
                Ok((object, n_written_bytes))
            }
            Err(_) => Err(()),
        }
    }

    pub fn push_atom_header<A: AtomBody + ?Sized>(&mut self, urid: &MappedURIDs) -> Result<(), ()> {
        let header = AtomHeader {
            size: 0,
            atom_type: A::get_urid(urid),
        };
        match self.write_sized(&header) {
            Ok((header, _)) => {
                self.headers.push_back(header);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    pub fn pop_atom_header(&mut self) {
        self.headers.pop_back();
    }
}