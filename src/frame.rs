use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::uris::MappedURIDs;
use std::marker::PhantomData;
use std::mem::size_of;

pub trait WritingFrame<'a> {
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
    unsafe fn write_raw(&mut self, data: &[u8]) -> Result<&'a mut [u8], ()>;

    fn get_header(&self) -> &AtomHeader;

    fn get_header_mut(&mut self) -> &mut AtomHeader;
}

pub trait WritingFrameExt<'a, A: AtomBody + ?Sized>: WritingFrame<'a> + Sized {
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
    unsafe fn write_sized<T: Sized>(&mut self, object: &T) -> Result<&'a mut T, ()> {
        let data: &[u8] =
            std::slice::from_raw_parts(object as *const T as *const u8, size_of::<T>());
        match self.write_raw(data) {
            Ok(data) => Ok((data.as_mut_ptr() as *mut T).as_mut().unwrap()),
            Err(_) => Err(()),
        }
    }

    /// Create a new atom header and return a nested writing frame for it.
    ///
    /// This function can be used for container atoms. Please note that this function only writes
    /// the atom header and does not initialize the atom body.
    ///
    /// Also, this function is unsafe since one can mess up atom structures.
    unsafe fn create_atom_frame<'b, C: AtomBody + ?Sized>(
        &'b mut self,
        urids: &MappedURIDs,
    ) -> Result<NestedFrame<'b, 'a, C>, ()> {
        let header = AtomHeader {
            size: 0,
            atom_type: C::get_urid(urids),
        };
        let header = self.write_sized(&header)?;
        let writer = NestedFrame {
            header: header,
            parent: self,
            phantom: PhantomData,
        };

        Ok(writer)
    }

    unsafe fn get_atom(&self) -> Result<&Atom<A>, ()> {
        A::widen_ref(self.get_header())
    }
}

pub struct RootFrame<'a, A: AtomBody + ?Sized> {
    header: &'a mut AtomHeader,
    free_data: &'a mut [u8],
    phantom: PhantomData<A>,
}

impl<'a, A: AtomBody + ?Sized> RootFrame<'a, A> {
    pub fn new(header: &'a mut AtomHeader, data: &'a mut [u8], urids: &MappedURIDs) -> Self {
        header.atom_type = A::get_urid(urids);
        header.size = 0;
        RootFrame {
            header: header,
            free_data: data,
            phantom: PhantomData,
        }
    }

    unsafe fn internal_write_raw(&mut self, data: &[u8]) -> Result<&'a mut [u8], ()> {
        if data.len() > self.free_data.len() {
            return Err(());
        }
        let data_ptr = self.free_data.as_mut_ptr();
        let n_free_bytes = self.free_data.len() - data.len();

        let target_data = std::slice::from_raw_parts_mut(data_ptr, data.len());
        let free_data = std::slice::from_raw_parts_mut(data_ptr.add(data.len()), n_free_bytes);

        target_data.copy_from_slice(data);
        self.free_data = free_data;
        Ok(target_data)
    }
}

impl<'a, A: AtomBody + ?Sized> WritingFrame<'a> for RootFrame<'a, A> {
    unsafe fn write_raw(&mut self, data: &[u8]) -> Result<&'a mut [u8], ()> {
        if data.len() > self.free_data.len() {
            return Err(());
        }

        let target_data = self.internal_write_raw(data)?;
        self.header.size += data.len() as i32;

        // Construct a reference to the newly written atom.
        Ok(target_data)
    }

    fn get_header(&self) -> &AtomHeader {
        self.header
    }

    fn get_header_mut(&mut self) -> &mut AtomHeader {
        self.header
    }
}

impl<'a, A: AtomBody + ?Sized> WritingFrameExt<'a, A> for RootFrame<'a, A> {}

pub struct NestedFrame<'a, 'b, A>
where
    A: AtomBody + ?Sized,
{
    header: &'b mut AtomHeader,
    parent: &'a mut WritingFrame<'b>,
    phantom: PhantomData<A>,
}

impl<'a, 'b, A> Drop for NestedFrame<'a, 'b, A>
where
    A: AtomBody + ?Sized,
{
    fn drop(&mut self) {
        let pad: &[u8] = match 8 - (self.parent.get_header().size % 8) {
            1 => &[0; 1],
            2 => &[0; 2],
            3 => &[0; 3],
            4 => &[0; 4],
            5 => &[0; 5],
            6 => &[0; 6],
            7 => &[0; 7],
            8 => &[0; 0],
            _ => panic!("invalid pad size"),
        };
        unsafe { self.parent.write_raw(pad).unwrap() };
    }
}

impl<'a, 'b, A> WritingFrame<'b> for NestedFrame<'a, 'b, A>
where
    A: AtomBody + ?Sized,
{
    unsafe fn write_raw(&mut self, data: &[u8]) -> Result<&'b mut [u8], ()> {
        let data = self.parent.write_raw(data)?;
        self.header.size += data.len() as i32;
        Ok(data)
    }

    fn get_header(&self) -> &AtomHeader {
        self.header
    }

    fn get_header_mut(&mut self) -> &mut AtomHeader {
        self.header
    }
}

impl<'a, 'b, A: AtomBody + ?Sized> WritingFrameExt<'b, A> for NestedFrame<'a, 'b, A> {}
