use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::uris::MappedURIDs;
use std::marker::PhantomData;
use std::mem::size_of;

pub trait WritingFrame<'a, A: AtomBody + ?Sized> {
    /// Try to write out a slice of bytes into the atom space.
    ///
    /// The data will be written directly after the previously written data. If `padding` is `true`,
    /// this function will leave space after writing the data which results in the next write being
    /// aligned to 64-bit (or 8 bytes).
    ///
    /// If writing was successfull, a slice with the written data is returned. This slice references
    /// memory in the atom space and does not contain padding bytes (which will be zeroed). In case
    /// of insufficient atom space, this function will return an error.
    ///
    /// Also, this function is unsafe since one can mess up atom structures.
    unsafe fn write_raw(&mut self, data: &[u8], padding: bool)
        -> Result<(&'a mut [u8], usize), ()>;

    fn get_header(&self) -> &AtomHeader;

    fn get_header_mut(&mut self) -> &mut AtomHeader;

    unsafe fn get_atom(&self) -> &Atom<A>
    where
        Atom<A>: Sized;

    unsafe fn get_atom_mut(&mut self) -> &mut Atom<A>
    where
        Atom<A>: Sized;
}

pub trait WritingFrameExt<'a, A: AtomBody + ?Sized> {
    /// Try to write a sized object into the atom space.
    ///
    /// The data will be written directly after the previously written data. If `padding` is `true`,
    /// this function will leave space after writing the data which results in the next write being
    /// aligned to 64-bit (or 8 bytes).
    ///
    /// If writing was successfull, a mutable reference to the written data in the atom space is
    /// returned. Obviously, this does not contain padding bytes (which will be zeroed). In case
    /// of insufficient atom space, this function will return an error.
    ///
    /// Also, this function is unsafe since one can mess up atom structures.
    unsafe fn write_sized<T: Sized>(
        &mut self,
        object: &T,
        padding: bool,
    ) -> Result<(&'a mut T, usize), ()>;

    /// Create a new atom header and return a nested writing frame for it.
    ///
    /// This function can be used for container atoms. Please note that this function only writes
    /// the atom header and does not initialize the atom body.
    ///
    /// Also, this function is unsafe since one can mess up atom structures.
    unsafe fn create_atom_frame<'b, C: AtomBody + ?Sized>(
        &'b mut self,
        urids: &MappedURIDs,
    ) -> Result<NestedFrame<'b, 'a, A, C>, ()>;
}

impl<'a, W, A> WritingFrameExt<'a, A> for W
where
    W: WritingFrame<'a, A>,
    A: AtomBody + ?Sized,
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

    unsafe fn create_atom_frame<'b, C: AtomBody + ?Sized>(
        &'b mut self,
        urids: &MappedURIDs,
    ) -> Result<NestedFrame<'b, 'a, A, C>, ()> {
        let header = AtomHeader {
            size: 0,
            atom_type: A::get_urid(urids),
        };
        let header = self.write_sized(&header, true)?.0;
        let writer = NestedFrame {
            header: header,
            parent: self,
            phantom: PhantomData,
        };

        Ok(writer)
    }
}

pub struct RootFrame<'a, A: AtomBody + ?Sized> {
    header: &'a mut AtomHeader,
    n_bytes_written: usize,
    free_data: &'a mut [u8],
    phantom: PhantomData<A>,
}

impl<'a, A: AtomBody + ?Sized> RootFrame<'a, A> {
    pub fn new(header: &'a mut AtomHeader, data: &'a mut [u8], urids: &MappedURIDs) -> Self {
        header.atom_type = A::get_urid(urids);
        header.size = 0;
        RootFrame {
            header: header,
            n_bytes_written: 0,
            free_data: data,
            phantom: PhantomData,
        }
    }
}

impl<'a, A: AtomBody + ?Sized> WritingFrame<'a, A> for RootFrame<'a, A> {
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
        self.header.size += n_written_bytes as i32;
        self.free_data = free_data;

        // Construct a reference to the newly written atom.
        Ok((target_data, n_written_bytes))
    }

    fn get_header(&self) -> &AtomHeader {
        self.header
    }

    fn get_header_mut(&mut self) -> &mut AtomHeader {
        self.header
    }

    unsafe fn get_atom(&self) -> &Atom<A>
    where
        Atom<A>: Sized,
    {
        (self.header as *const AtomHeader as *const Atom<A>)
            .as_ref()
            .unwrap()
    }

    unsafe fn get_atom_mut(&mut self) -> &mut Atom<A>
    where
        Atom<A>: Sized,
    {
        (self.header as *mut AtomHeader as *mut Atom<A>)
            .as_mut()
            .unwrap()
    }
}

pub struct NestedFrame<'a, 'b, P, A>
where
    P: AtomBody + ?Sized,
    A: AtomBody + ?Sized,
{
    header: &'b mut AtomHeader,
    parent: &'a mut WritingFrame<'b, P>,
    phantom: PhantomData<A>,
}

impl<'a, 'b, P, A> WritingFrame<'b, A> for NestedFrame<'a, 'b, P, A>
where
    P: AtomBody + ?Sized,
    A: AtomBody + ?Sized,
{
    unsafe fn write_raw(
        &mut self,
        data: &[u8],
        padding: bool,
    ) -> Result<(&'b mut [u8], usize), ()> {
        let (data, n_bytes_written) = self.parent.write_raw(data, padding)?;
        self.header.size += n_bytes_written as i32;
        Ok((data, n_bytes_written))
    }

    fn get_header(&self) -> &AtomHeader {
        self.header
    }

    fn get_header_mut(&mut self) -> &mut AtomHeader {
        self.header
    }

    unsafe fn get_atom(&self) -> &Atom<A>
    where
        Atom<A>: Sized,
    {
        (self.header as *const AtomHeader as *const Atom<A>)
            .as_ref()
            .unwrap()
    }

    unsafe fn get_atom_mut(&mut self) -> &mut Atom<A>
    where
        Atom<A>: Sized,
    {
        (self.header as *mut AtomHeader as *mut Atom<A>)
            .as_mut()
            .unwrap()
    }
}
