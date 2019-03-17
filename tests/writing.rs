extern crate lv2rs_atom as atom;

use atom::prelude::*;
use atom::uris::MappedURIDs;

#[test]
fn test_scalar() {
    let mut atom_space = vec![0u8; std::mem::size_of::<Atom<f32>>() + 8];
    atom_space[0] = std::mem::size_of::<f32>() as u8 + 8;

    let mut port: AtomOutputPort<f32> = AtomOutputPort::new();
    port.connect_port(atom_space.as_mut_ptr() as *mut AtomHeader);
    let frame = port.write_atom(&42.0f32, &MappedURIDs::default()).unwrap();

    assert_eq!(42.0f32, unsafe { frame.get_atom() }.body);
}
