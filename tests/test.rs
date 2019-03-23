extern crate lv2rs_atom as atom;

use atom::ports::*;
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
    let atom = in_port.get_atom(&urids).unwrap();

    // Asserting.
    assert_eq!(4, atom.body_size());
    assert_eq!(urids.float, atom.body_type());
    assert_eq!(42.0, **atom);
}

#[test]
fn test_literal() {
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
    let atom = in_port.get_atom(&urids).unwrap();

    // Asserting.
    assert_eq!(21, atom.body_size());
    assert_eq!(urids.literal, atom.body_type());
    assert_eq!(0, atom.lang());
    assert_eq!("Hello World!", atom.as_str().unwrap());
}

#[test]
fn test_string() {
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
    let atom = in_port.get_atom(&urids).unwrap();

    // Asserting.
    assert_eq!(13, atom.body_size());
    assert_eq!(urids.string, atom.body_type());
    assert_eq!("Hello World!", atom.as_cstr().unwrap().to_str().unwrap());
}

#[test]
fn test_vector() {
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
    let atom = in_port.get_atom(&urids).unwrap();

    // Asserting.
    assert_eq!(8 + 4 * 5, atom.body_size());
    assert_eq!(urids.vector, atom.body_type());
    assert_eq!(4, atom.child_body_size());
    assert_eq!(urids.float, atom.child_body_type());
    assert_eq!([0.0, 1.0, 2.0, 3.0, 4.0], atom.as_slice());
}

#[test]
fn test_tuple() {
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
        assert_eq!(0, frame.get_header().size % 8);
        frame.push_atom::<i32>(&42, &urids).unwrap();
        assert_eq!(0, frame.get_header().size % 8);
        frame
            .push_atom::<Vector<i32>>(&urids, &urids)
            .unwrap()
            .append(&[0, 2, 4])
            .unwrap();
        assert_eq!(0, frame.get_header().size % 8);
        frame
            .push_atom::<Literal>(&0, &urids)
            .unwrap()
            .write_string("Hello World!")
            .unwrap();
        assert_eq!(0, frame.get_header().size % 8);
    }

    // Reading.
    let atom = in_port.get_atom(&urids).unwrap();

    // i32: AtomHeader, int, pad.
    let mut assumed_size = 8 + 4 + 4;
    // Vector: AtomHeader, VectorHeader, slice, pad.
    assumed_size += 8 + 8 + 3 * 4 + 4;
    // Literal: AtomHeader, LiteralHeader, string, pad.
    assumed_size += 8 + 8 + 13 + 3;
    assert_eq!(assumed_size, atom.body_size());
    assert_eq!(urids.tuple, atom.body_type());

    let mut iter = atom.iter();
    let integer = iter.next().unwrap().cast::<i32>(&urids).unwrap();
    assert_eq!(42, **integer);

    let vector = iter.next().unwrap();
    let vector = vector.cast::<Vector<i32>>(&urids);
    let vector = vector.unwrap();
    assert_eq!([0, 2, 4], *vector.as_slice());

    let literal = iter.next().unwrap().cast::<Literal>(&urids).unwrap();
    assert_eq!("Hello World!", literal.as_str().unwrap());
}

#[test]
fn test_sequence() {
    use atom::sequence::{TimeStamp, TimeUnit};
    let urids = MappedURIDs::default();

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Sequence> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Sequence> = AtomInputPort::new(&urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    {
        let mut frame = out_port.write_atom(&TimeUnit::Frames, &urids).unwrap();
        assert_eq!(0, frame.get_header().size % 8);
        frame
            .push_event::<i32>(TimeStamp::Frames(0), &42, &urids)
            .unwrap();
        assert_eq!(0, frame.get_header().size % 8);

        let old_atom_size = frame.get_header().size;
        assert!(frame
            .push_event::<i32>(TimeStamp::Beats(42.0), &42, &urids)
            .is_err());
        assert_eq!(old_atom_size, frame.get_header().size);

        {
            let mut tuple_frame = frame
                .push_event::<Tuple>(TimeStamp::Frames(1), &(), &urids)
                .unwrap();
            assert_eq!(0, tuple_frame.get_header().size % 8);
            tuple_frame.push_atom::<i32>(&1, &urids).unwrap();
            assert_eq!(0, tuple_frame.get_header().size % 8);
            tuple_frame.push_atom::<i32>(&2, &urids).unwrap();
            assert_eq!(0, tuple_frame.get_header().size % 8);
        }
        assert_eq!(0, frame.get_header().size % 8);
    }

    // Reading.
    let atom = in_port.get_atom(&urids).unwrap();
    let mut sequence_iter = atom.iter(&urids).unwrap();

    let (time_stamp, integer) = sequence_iter.next().unwrap();
    assert_eq!(TimeStamp::Frames(0), time_stamp);
    let integer: &Atom<i32> = integer.cast(&urids).unwrap();
    assert_eq!(42, **integer);

    let (time_stamp, tuple) = sequence_iter.next().unwrap();
    assert_eq!(TimeStamp::Frames(1), time_stamp);
    let tuple: &Atom<Tuple> = tuple.cast(&urids).unwrap();
    {
        let mut iter = tuple.iter();
        let integer: &Atom<i32> = iter.next().unwrap().cast(&urids).unwrap();
        assert_eq!(1, **integer);
        let integer: &Atom<i32> = iter.next().unwrap().cast(&urids).unwrap();
        assert_eq!(2, **integer);
    }
}
