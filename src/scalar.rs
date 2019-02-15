use crate::types;

use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use urid::URID;

impl Deref for types::AtomInt {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.body
    }
}

impl DerefMut for types::AtomInt {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.body
    }
}

impl Deref for types::AtomLong {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.body
    }
}

impl DerefMut for types::AtomLong {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.body
    }
}

impl Deref for types::AtomFloat {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.body
    }
}

impl DerefMut for types::AtomFloat {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.body
    }
}

impl Deref for types::AtomDouble {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.body
    }
}

impl DerefMut for types::AtomDouble {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.body
    }
}

impl Deref for types::AtomURID {
    type Target = URID;
    fn deref(&self) -> &URID {
        &self.body
    }
}

impl DerefMut for types::AtomURID {
    fn deref_mut(&mut self) -> &mut URID {
        &mut self.body
    }
}

impl Deref for types::AtomString {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        use std::mem::size_of;
        use std::os::raw::c_char;

        let atom_pointer = self as *const Self;
        let atom_pointer = unsafe { atom_pointer.add(size_of::<types::AtomString>()) };
        unsafe { CStr::from_ptr(atom_pointer as *const c_char) }
    }
}
