extern crate lv2rs_atom as atom;
extern crate lv2rs_urid as urid;

use atom::ports::*;
use atom::prelude::*;
use std::ffi::CStr;
use urid::debug::DebugMap;

#[test]
fn test_scalar() {
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<f32> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<f32> = AtomInputPort::new(&mut urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    unsafe { out_port.write_atom(&42.0f32, &mut urids) }.unwrap();

    // Reading.
    let atom = unsafe { in_port.get_atom(&mut urids) }.unwrap();

    // Asserting.
    assert_eq!(4, atom.body_size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::FLOAT_TYPE_URI).unwrap()),
        atom.body_type()
    );
    assert_eq!(42.0, **atom);
}

#[test]
fn test_literal() {
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Literal> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Literal> = AtomInputPort::new(&mut urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    assert!(unsafe { out_port.write_atom(&0, &mut urids) }
        .unwrap()
        .write_string("Hello World!")
        .is_ok());

    // Reading.
    let atom = unsafe { in_port.get_atom(&mut urids) }.unwrap();

    // Asserting.
    assert_eq!(21, atom.body_size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::LITERAL_TYPE_URI).unwrap()),
        atom.body_type()
    );
    assert_eq!(0, atom.lang());
    assert_eq!("Hello World!", atom.as_str().unwrap());
}

#[test]
fn test_string() {
    use std::ffi::CStr;

    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<AtomString> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<AtomString> = AtomInputPort::new(&mut urids);
    in_port.connect_port(atom as &AtomHeader);

    let message = CStr::from_bytes_with_nul(b"Hello World!\0").unwrap();
    // Writing.
    unsafe { out_port.write_atom(message, &mut urids) }.unwrap();

    // Reading.
    let atom = unsafe { in_port.get_atom(&mut urids) }.unwrap();

    // Asserting.
    assert_eq!(13, atom.body_size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::STRING_TYPE_URI).unwrap()),
        atom.body_type()
    );
    assert_eq!("Hello World!", atom.as_cstr().unwrap().to_str().unwrap());
}

#[test]
fn test_vector() {
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Vector<f32>> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Vector<f32>> = AtomInputPort::new(&mut urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    {
        let mut frame = unsafe { out_port.write_atom(&(), &mut urids) }.unwrap();
        frame.push(0.0).unwrap();
        frame.append(&[1.0, 2.0, 3.0, 4.0]).unwrap();
    }

    // Reading.
    let atom = unsafe { in_port.get_atom(&mut urids) }.unwrap();

    // Asserting.
    assert_eq!(8 + 4 * 5, atom.body_size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::VECTOR_TYPE_URI).unwrap()),
        atom.body_type()
    );
    assert_eq!(4, atom.child_body_size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::FLOAT_TYPE_URI).unwrap()),
        atom.child_body_type()
    );
    assert_eq!([0.0, 1.0, 2.0, 3.0, 4.0], atom.as_slice());
}

#[test]
fn test_tuple() {
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Tuple> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Tuple> = AtomInputPort::new(&mut urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    {
        let mut frame = unsafe { out_port.write_atom(&(), &mut urids) }.unwrap();
        assert_eq!(0, frame.get_header().size % 8);
        frame.push_atom::<i32>(&42, &mut urids).unwrap();
        assert_eq!(0, frame.get_header().size % 8);
        frame
            .push_atom::<Vector<i32>>(&(), &mut urids)
            .unwrap()
            .append(&[0, 2, 4])
            .unwrap();
        assert_eq!(0, frame.get_header().size % 8);
        frame
            .push_atom::<Literal>(&0, &mut urids)
            .unwrap()
            .write_string("Hello World!")
            .unwrap();
        assert_eq!(0, frame.get_header().size % 8);
    }

    // Reading.
    let atom = unsafe { in_port.get_atom(&mut urids) }.unwrap();

    // i32: AtomHeader, int, pad.
    let mut assumed_size = 8 + 4 + 4;
    // Vector: AtomHeader, VectorHeader, slice, pad.
    assumed_size += 8 + 8 + 3 * 4 + 4;
    // Literal: AtomHeader, LiteralHeader, string, pad.
    assumed_size += 8 + 8 + 13 + 3;
    assert_eq!(assumed_size, atom.body_size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::TUPLE_TYPE_URI).unwrap()),
        atom.body_type()
    );

    let mut iter = atom.iter();
    let integer = iter.next().unwrap().cast::<i32>(&mut urids).unwrap();
    assert_eq!(42, **integer);

    let vector = iter.next().unwrap();
    let vector = vector.cast::<Vector<i32>>(&mut urids);
    let vector = vector.unwrap();
    assert_eq!([0, 2, 4], *vector.as_slice());

    let literal = iter.next().unwrap().cast::<Literal>(&mut urids).unwrap();
    assert_eq!("Hello World!", literal.as_str().unwrap());
}

#[test]
fn test_sequence() {
    use atom::sequence::{TimeStamp, TimeUnit};
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut AtomHeader).as_mut() }.unwrap();
    atom.size = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Sequence> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Sequence> = AtomInputPort::new(&mut urids);
    in_port.connect_port(atom as &AtomHeader);

    // Writing.
    {
        let mut frame = unsafe { out_port.write_atom(&TimeUnit::Frames, &mut urids) }.unwrap();
        assert_eq!(0, frame.get_header().size % 8);
        frame
            .push_event::<i32>(TimeStamp::Frames(0), &42, &mut urids)
            .unwrap();
        assert_eq!(0, frame.get_header().size % 8);

        let old_atom_size = frame.get_header().size;
        assert!(frame
            .push_event::<i32>(TimeStamp::Beats(42.0), &42, &mut urids)
            .is_err());
        assert_eq!(old_atom_size, frame.get_header().size);

        {
            let mut tuple_frame = frame
                .push_event::<Tuple>(TimeStamp::Frames(1), &(), &mut urids)
                .unwrap();
            assert_eq!(0, tuple_frame.get_header().size % 8);
            tuple_frame.push_atom::<i32>(&1, &mut urids).unwrap();
            assert_eq!(0, tuple_frame.get_header().size % 8);
            tuple_frame.push_atom::<i32>(&2, &mut urids).unwrap();
            assert_eq!(0, tuple_frame.get_header().size % 8);
        }
        assert_eq!(0, frame.get_header().size % 8);
    }

    // Reading.
    let atom = unsafe { in_port.get_atom(&mut urids) }.unwrap();
    let mut sequence_iter = atom.iter(&mut urids).unwrap();

    let (time_stamp, integer) = sequence_iter.next().unwrap();
    assert_eq!(TimeStamp::Frames(0), time_stamp);
    let integer: &Atom<i32> = integer.cast(&mut urids).unwrap();
    assert_eq!(42, **integer);

    let (time_stamp, tuple) = sequence_iter.next().unwrap();
    assert_eq!(TimeStamp::Frames(1), time_stamp);
    let tuple: &Atom<Tuple> = tuple.cast(&mut urids).unwrap();
    {
        let mut iter = tuple.iter();
        let integer: &Atom<i32> = iter.next().unwrap().cast(&mut urids).unwrap();
        assert_eq!(1, **integer);
        let integer: &Atom<i32> = iter.next().unwrap().cast(&mut urids).unwrap();
        assert_eq!(2, **integer);
    }
}
