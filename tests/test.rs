extern crate lv2rs_atom as atom;

use atom::prelude::*;
use atom::uris::MappedURIDs;

#[test]
fn test_scalar() {
    let urids = MappedURIDs::default();

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<f32> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<f32> = AtomInputPort::new(&urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    out_port.write_atom(&42.0f32, &urids).unwrap();

    // Reading.
    let atom = in_port.get_atom().unwrap();

    // Asserting.
    assert_eq!(4, atom.body_size());
    assert_eq!(urids.float, atom.body_type());
    assert_eq!(42.0, **atom);
}

#[test]
fn test_literal() {
    use atom::atom::literal::Literal;

    let urids = MappedURIDs::default();

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Literal> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Literal> = AtomInputPort::new(&urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    assert!(out_port
        .write_atom(&0, &urids)
        .unwrap()
        .write_string("Hello World!")
        .is_ok());

    // Reading.
    let atom = in_port.get_atom().unwrap();

    // Asserting.
    assert_eq!(21, atom.body_size());
    assert_eq!(urids.literal, atom.body_type());
    assert_eq!(0, atom.lang());
    assert_eq!("Hello World!", atom.as_str().unwrap());
}

#[test]
fn test_string() {
    use atom::atom::string::AtomString;
    use std::ffi::CStr;

    let urids = MappedURIDs::default();

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<AtomString> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<AtomString> = AtomInputPort::new(&urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    assert!(out_port
        .write_atom(&(), &urids)
        .unwrap()
        .write_string(CStr::from_bytes_with_nul(b"Hello World!\0").unwrap())
        .is_ok());

    // Reading.
    let atom = in_port.get_atom().unwrap();

    // Asserting.
    assert_eq!(13, atom.body_size());
    assert_eq!(urids.string, atom.body_type());
    assert_eq!("Hello World!", atom.as_cstr().unwrap().to_str().unwrap());
}

#[test]
fn test_vector() {
    use atom::atom::vector::Vector;

    let urids = MappedURIDs::default();

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Vector<f32>> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Vector<f32>> = AtomInputPort::new(&urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    {
        let mut frame = out_port.write_atom(&urids, &urids).unwrap();
        frame.push(0.0).unwrap();
        frame.append(&[1.0, 2.0, 3.0, 4.0]).unwrap();
    }

    // Reading.
    let atom = in_port.get_atom().unwrap();

    // Asserting.
    assert_eq!(8 + 4 * 5, atom.body_size());
    assert_eq!(urids.vector, atom.body_type());
    assert_eq!(4, atom.child_body_size());
    assert_eq!(urids.float, atom.child_body_type());
    assert_eq!([0.0, 1.0, 2.0, 3.0, 4.0], atom.as_slice());
}

#[test]
fn test_tuple() {
    use atom::atom::literal::Literal;
    use atom::atom::tuple::Tuple;
    use atom::atom::vector::Vector;

    let urids = MappedURIDs::default();

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Tuple> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Tuple> = AtomInputPort::new(&urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    {
        let mut frame = out_port.write_atom(&(), &urids).unwrap();
        frame.push_atom::<i32>(&42, &urids).unwrap();
        frame
            .push_atom::<Vector<i32>>(&urids, &urids)
            .unwrap()
            .append(&[0, 2, 4])
            .unwrap();
        assert!(frame
            .push_atom::<Literal>(&0, &urids)
            .unwrap()
            .write_string("Hello World!")
            .is_ok());
    }

    // Reading.
    let atom = in_port.get_atom().unwrap();

    // i32: AtomHeader, int, pad.
    let mut assumed_size = 8 + 4 + 4;
    // Vector: AtomHeader, VectorHeader, slice, pad.
    assumed_size += 8 + 8 + 3 * 4 + 4;
    // Literal: AtomHeader, LiteralHeader, string, pad.
    assumed_size += 8 + 8 + 13 + 5;
    assert_eq!(assumed_size, atom.body_size());
    assert_eq!(urids.tuple, atom.body_type());

    let mut iter = atom.iter();
    let integer = iter.next().unwrap().cast::<i32>(&urids).unwrap();
    assert_eq!(42, **integer);

    let vector = iter.next().unwrap().cast::<Vector<i32>>(&urids).unwrap();
    assert_eq!([0, 2, 4], *vector.as_slice());

    let literal = iter.next().unwrap().cast::<Literal>(&urids).unwrap();
    assert_eq!("Hello World!", literal.as_str().unwrap());
}
