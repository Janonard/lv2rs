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
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut Atom).as_mut() }.unwrap();
    *(atom.mut_size()) = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<f32> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<f32> = AtomInputPort::new();
    in_port.connect_port(atom as &Atom);

    // Writing.
    unsafe { out_port.write_atom_body(&42.0f32, &mut urids) }.unwrap();

    // Reading.
    let float = unsafe { in_port.get_atom_body(&mut urids) }.unwrap();
    let header = unsafe {
        (float as *const f32 as *const Atom)
            .sub(1)
            .as_ref()
            .unwrap()
    };

    // Asserting.
    assert_eq!(4, header.size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::FLOAT_TYPE_URI).unwrap()),
        header.atom_type()
    );
    assert_eq!(42.0, *float);
}

#[test]
fn test_literal() {
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut Atom).as_mut() }.unwrap();
    *(atom.mut_size()) = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Literal> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Literal> = AtomInputPort::new();
    in_port.connect_port(atom as &Atom);

    // Writing.
    assert!(unsafe { out_port.write_atom_body(&0, &mut urids) }
        .unwrap()
        .append_string("Hello World!")
        .is_ok());

    // Reading.
    let literal = unsafe { in_port.get_atom_body(&mut urids) }.unwrap();
    let header = unsafe {
        (literal as *const Literal as *const Atom)
            .sub(1)
            .as_ref()
            .unwrap()
    };

    // Asserting.
    assert_eq!(20, header.size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::LITERAL_TYPE_URI).unwrap()),
        header.atom_type()
    );
    assert_eq!(0, literal.lang());
    assert_eq!("Hello World!", literal.as_str().unwrap());
}

#[test]
fn test_string() {
    use std::ffi::CStr;

    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut Atom).as_mut() }.unwrap();
    *(atom.mut_size()) = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<AtomString> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<AtomString> = AtomInputPort::new();
    in_port.connect_port(atom as &Atom);

    let message = CStr::from_bytes_with_nul(b"Hello World!\0").unwrap();
    // Writing.
    unsafe { out_port.write_atom_body(message, &mut urids) }.unwrap();

    // Reading.
    let string = unsafe { in_port.get_atom_body(&mut urids) }.unwrap();
    let header = unsafe {
        (string as *const AtomString as *const Atom)
            .sub(1)
            .as_ref()
            .unwrap()
    };

    // Asserting.
    assert_eq!(13, header.size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::STRING_TYPE_URI).unwrap()),
        atom.atom_type()
    );
    assert_eq!("Hello World!", string.as_cstr().unwrap().to_str().unwrap());
}

#[test]
fn test_vector() {
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut Atom).as_mut() }.unwrap();
    *(atom.mut_size()) = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Vector<f32>> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Vector<f32>> = AtomInputPort::new();
    in_port.connect_port(atom as &Atom);

    // Writing.
    {
        let mut frame = unsafe { out_port.write_atom_body(&(), &mut urids) }.unwrap();
        frame.push(0.0).unwrap();
        frame.append(&[1.0, 2.0, 3.0, 4.0]).unwrap();
    }

    // Reading.
    let vector = unsafe { in_port.get_atom_body(&mut urids) }.unwrap();
    let header = unsafe {
        (vector as *const Vector<f32> as *const Atom)
            .sub(1)
            .as_ref()
            .unwrap()
    };

    // Asserting.
    assert_eq!(8 + 4 * 5, header.size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::VECTOR_TYPE_URI).unwrap()),
        header.atom_type()
    );
    assert_eq!(4, vector.child_body_size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::FLOAT_TYPE_URI).unwrap()),
        vector.child_body_type()
    );
    assert_eq!([0.0, 1.0, 2.0, 3.0, 4.0], vector.as_slice());
}

#[test]
fn test_tuple() {
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut Atom).as_mut() }.unwrap();
    *(atom.mut_size()) = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Tuple> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Tuple> = AtomInputPort::new();
    in_port.connect_port(atom as &Atom);

    // Writing.
    {
        let mut frame = unsafe { out_port.write_atom_body(&(), &mut urids) }.unwrap();
        assert_eq!(0, frame.get_atom().size() % 8);
        frame.push_atom::<i32>(&42, &mut urids).unwrap();
        assert_eq!(0, frame.get_atom().size() % 8);
        frame
            .push_atom::<Vector<i32>>(&(), &mut urids)
            .unwrap()
            .append(&[0, 2, 4])
            .unwrap();
        assert_eq!(0, frame.get_atom().size() % 8);
        frame
            .push_atom::<Literal>(&0, &mut urids)
            .unwrap()
            .append_string("Hello World!")
            .unwrap();
        assert_eq!(0, frame.get_atom().size() % 8);
    }

    // Reading.
    let tuple = unsafe { in_port.get_atom_body(&mut urids) }.unwrap();
    let header = unsafe {
        (tuple as *const Tuple as *const Atom)
            .sub(1)
            .as_ref()
            .unwrap()
    };

    // i32: AtomHeader, int, pad.
    let mut assumed_size = 8 + 4 + 4;
    // Vector: AtomHeader, VectorHeader, slice, pad.
    assumed_size += 8 + 8 + 3 * 4 + 4;
    // Literal: AtomHeader, LiteralHeader, string, pad.
    assumed_size += 8 + 8 + 13 + 3;
    assert_eq!(assumed_size, header.size());
    assert_eq!(
        urids.map(CStr::from_bytes_with_nul(atom::uris::TUPLE_TYPE_URI).unwrap()),
        atom.atom_type()
    );

    let mut iter = tuple.iter();
    let integer = iter.next().unwrap().get_body::<i32>(&mut urids).unwrap();
    assert_eq!(42, *integer);

    let vector = iter.next().unwrap();
    let vector = vector.get_body::<Vector<i32>>(&mut urids);
    let vector = vector.unwrap();
    assert_eq!([0, 2, 4], *vector.as_slice());

    let literal = iter
        .next()
        .unwrap()
        .get_body::<Literal>(&mut urids)
        .unwrap();
    assert_eq!("Hello World!", literal.as_str().unwrap());
}

#[test]
fn test_sequence() {
    use atom::sequence::{TimeStamp, TimeUnit};
    let mut debug_map = DebugMap::new();
    let mut urids = unsafe { debug_map.create_cached_map() };

    // Creating the atom space.
    let mut atom_space = vec![0u8; 256];
    let atom = unsafe { (atom_space.as_mut_ptr() as *mut Atom).as_mut() }.unwrap();
    *(atom.mut_size()) = 256 - 8;

    // Creating the ports and connecting them.
    let mut out_port: AtomOutputPort<Sequence> = AtomOutputPort::new();
    out_port.connect_port(atom);
    let mut in_port: AtomInputPort<Sequence> = AtomInputPort::new();
    in_port.connect_port(atom as &Atom);

    // Writing.
    {
        let mut frame = unsafe { out_port.write_atom_body(&TimeUnit::Frames, &mut urids) }.unwrap();
        assert_eq!(0, frame.get_atom().size() % 8);
        frame
            .push_event::<i32>(TimeStamp::Frames(0), &42, &mut urids)
            .unwrap();
        assert_eq!(0, frame.get_atom().size() % 8);

        let old_atom_size = frame.get_atom().size();
        assert!(frame
            .push_event::<i32>(TimeStamp::Beats(42.0), &42, &mut urids)
            .is_err());
        assert_eq!(old_atom_size, frame.get_atom().size());

        {
            let mut tuple_frame = frame
                .push_event::<Tuple>(TimeStamp::Frames(1), &(), &mut urids)
                .unwrap();
            assert_eq!(0, tuple_frame.get_atom().size() % 8);
            tuple_frame.push_atom::<i32>(&1, &mut urids).unwrap();
            assert_eq!(0, tuple_frame.get_atom().size() % 8);
            tuple_frame.push_atom::<i32>(&2, &mut urids).unwrap();
            assert_eq!(0, tuple_frame.get_atom().size() % 8);
        }
        assert_eq!(0, frame.get_atom().size() % 8);
    }

    // Reading.
    let atom = unsafe { in_port.get_atom_body(&mut urids) }.unwrap();
    let mut sequence_iter = atom.iter(&mut urids);

    let (time_stamp, integer) = sequence_iter.next().unwrap();
    assert_eq!(TimeStamp::Frames(0), time_stamp);
    let integer: &i32 = integer.get_body(&mut urids).unwrap();
    assert_eq!(42, *integer);

    let (time_stamp, tuple) = sequence_iter.next().unwrap();
    assert_eq!(TimeStamp::Frames(1), time_stamp);
    let tuple: &Tuple = tuple.get_body(&mut urids).unwrap();
    {
        let mut iter = tuple.iter();
        let integer: &i32 = iter.next().unwrap().get_body(&mut urids).unwrap();
        assert_eq!(1, *integer);
        let integer: &i32 = iter.next().unwrap().get_body(&mut urids).unwrap();
        assert_eq!(2, *integer);
    }
}
