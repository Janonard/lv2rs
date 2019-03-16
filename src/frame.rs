use crate::atom::{AtomBody, AtomHeader};
use crate::uris::MappedURIDs;
use std::marker::PhantomData;
use std::mem::size_of;

pub trait CoreWritingFrame<'a> {
    unsafe fn write_raw(&mut self, data: &[u8], padding: bool)
        -> Result<(&'a mut [u8], usize), ()>;
}

pub trait WritingFrame<'a> {
    unsafe fn write_sized<T: Sized>(
        &mut self,
        object: &T,
        padding: bool,
    ) -> Result<(&'a mut T, usize), ()>;

    unsafe fn create_atom<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        urids: &MappedURIDs,
        parameter: &A::InitializationParameter,
    ) -> Result<AtomFrame<'b, 'a, A>, ()>;
}

impl<'a, W> WritingFrame<'a> for W
where
    W: CoreWritingFrame<'a>,
{
    unsafe fn write_sized<T: Sized>(
        &mut self,
        object: &T,
        padding: bool,
    ) -> Result<(&'a mut T, usize), ()> {
        let data: &[u8] =
            std::slice::from_raw_parts(object as *const T as *const u8, size_of::<T>());
        match self.write_raw(data, padding) {
            Ok((data, n_written_bytes)) => {
                let object = (data.as_mut_ptr() as *mut T).as_mut().unwrap();
                Ok((object, n_written_bytes))
            }
            Err(_) => Err(()),
        }
    }

    unsafe fn create_atom<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        urids: &MappedURIDs,
        parameter: &A::InitializationParameter,
    ) -> Result<AtomFrame<'b, 'a, A>, ()> {
        let header = AtomHeader {
            size: 0,
            atom_type: A::get_urid(urids),
        };
        let header = self.write_sized(&header, true)?.0;
        let mut writer = AtomFrame {
            header: header,
            parent: self,
            phantom: PhantomData,
        };
        A::initialize_body(&mut writer, parameter)?;

        Ok(writer)
    }
}

pub struct RootFrame<'a, A: AtomBody + ?Sized> {
    n_bytes_written: usize,
    free_data: &'a mut [u8],
    phantom: PhantomData<A>,
}

impl<'a, A: AtomBody + ?Sized> RootFrame<'a, A> {
    pub fn new(data: &'a mut [u8]) -> Self {
        RootFrame {
            n_bytes_written: 0,
            free_data: data,
            phantom: PhantomData,
        }
    }
}

impl<'a, A: AtomBody + ?Sized> CoreWritingFrame<'a> for RootFrame<'a, A> {
    unsafe fn write_raw(
        &mut self,
        data: &[u8],
        padding: bool,
    ) -> Result<(&'a mut [u8], usize), ()> {
        let n_payload_bytes = data.len();
        let n_padding_bytes = if padding {
            (self.n_bytes_written + n_payload_bytes) % 8
        } else {
            0
        };
        let n_written_bytes = n_payload_bytes + n_padding_bytes;
        if n_written_bytes > self.free_data.len() {
            return Err(());
        }
        let n_free_bytes = self.free_data.len() - n_written_bytes;

        // Creating all required slices.
        let data_ptr = self.free_data.as_mut_ptr();

        let target_data = std::slice::from_raw_parts_mut(data_ptr, n_payload_bytes);
        let padding =
            std::slice::from_raw_parts_mut(data_ptr.add(n_payload_bytes), n_padding_bytes);
        let free_data = std::slice::from_raw_parts_mut(data_ptr.add(n_written_bytes), n_free_bytes);

        target_data.copy_from_slice(data);
        for byte in padding.iter_mut() {
            *byte = 0;
        }
        self.n_bytes_written += n_written_bytes;
        self.free_data = free_data;

        // Construct a reference to the newly written atom.
        Ok((target_data, n_written_bytes))
    }
}

pub struct AtomFrame<'a, 'b, A: AtomBody + ?Sized> {
    header: &'b mut AtomHeader,
    parent: &'a mut CoreWritingFrame<'b>,
    phantom: PhantomData<A>,
}

impl<'a, 'b, A: AtomBody + ?Sized> CoreWritingFrame<'b> for AtomFrame<'a, 'b, A> {
    unsafe fn write_raw(
        &mut self,
        data: &[u8],
        padding: bool,
    ) -> Result<(&'b mut [u8], usize), ()> {
        let (data, n_bytes_written) = self.parent.write_raw(data, padding)?;
        self.header.size += n_bytes_written as i32;
        Ok((data, n_bytes_written))
    }
}
