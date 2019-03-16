extern crate lv2rs_core as core;
extern crate lv2rs_urid as urid;

pub mod atom;
pub mod frame;
pub mod ports;
pub mod uris;

#[test]
fn test() {
    use atom::*;
    use frame::*;
    use ports::*;
    use uris::MappedURIDs;

    let mut atom_space = vec![0u8; std::mem::size_of::<Atom<f32>>() + 8];
    atom_space[0] = std::mem::size_of::<f32>() as u8 + 8;

    let mut port: AtomOutputPort<f32> = AtomOutputPort::new();
    port.connect_port(atom_space.as_mut_ptr() as *mut AtomHeader);
    let frame = port
        .initialize_atom(&42.0f32, &MappedURIDs::default())
        .unwrap();
    assert_eq!(42.0f32, unsafe { frame.get_atom() }.body);
}
