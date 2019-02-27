use crate::{uris::MappedURIDs, Atom, AtomHeader};

pub struct AtomInputPort {
    raw: *const AtomHeader,
    urids: MappedURIDs,
}

impl AtomInputPort {
    pub fn new(urids: MappedURIDs) -> Self {
        Self {
            raw: std::ptr::null(),
            urids: urids,
        }
    }

    pub fn connect(&mut self, raw: *const AtomHeader) {
        self.raw = raw;
    }

    pub unsafe fn try_get<A: Atom>(&self) -> Option<&A> {
        let atom = match self.raw.as_ref() {
            Some(atom) => atom,
            None => return None,
        };
        if atom.atom_type == A::get_urid(&self.urids) {
            Some((self.raw as *const A).as_ref().unwrap())
        } else {
            None
        }
    }
}
