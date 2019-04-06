//! Atom tuple with time stamps for every contained atom.
//!
//! A sequence is basically a tuple, but every contained atom is marked with the time stamp. This
//! way, these atoms, now called "events", can be matched to frames in audio slices and processed
//! with accurate timing. However, these time stamps can be given either in frames, which
//! correspond to elements in an audio data slice, or in beats, a more abstract, tempo-independent
//! way of describing timing.
//!
//! When writing a sequence, you have to pass over the time unit the sequence should. However, after
//! it was initialized, a sequence does not contain any atoms. These have to be pushed to the sequence
//! using the [`SequenceWritingFrame`](trait.SequenceWritingFrame.html) trait. Every
//! writing frame implements this trait via a blanket implementation and the trait is included in
//! the crate's prelude. You can, therefore, act as if the extended methods were normal methods of a
//! writing frame.
//!
//! Reading atoms is done by iterating through all atoms one by one. Iterators are produced by the
//! [`iter`](type.Tuple.html#method.iter) method.
//!
//! An example:
//!
//!     extern crate lv2rs_atom as atom;
//!     extern crate lv2rs_urid as urid;
//!
//!     use atom::prelude::*;
//!     use atom::ports::*;
//!     use atom::*;
//!     use urid::{CachedMap, debug::DebugMap};
//!     use std::ffi::CStr;
//!
//!     pub struct Plugin {
//!         in_port: AtomInputPort<Tuple>,
//!         out_port: AtomOutputPort<Tuple>,
//!         urids: CachedMap,
//!     }
//!
//!     impl Plugin {
//!         /// Simulated `run` method.
//!         fn run(&mut self) {
//!             // Writing
//!             {
//!                 let mut frame =
//!                     unsafe { self.out_port.write_atom_body(&(), &mut self.urids) }.unwrap();
//!                 frame.push_atom::<i32>(&42, &mut self.urids).unwrap();
//!                 frame.push_atom::<f32>(&17.0, &mut self.urids).unwrap();
//!             }
//!
//!             let i32_urid = self.urids.map(<i32 as AtomBody>::get_uri());
//!             let f32_urid = self.urids.map(<f32 as AtomBody>::get_uri());
//!
//!             // Reading.
//!             let tuple = unsafe { self.in_port.get_atom_body(&mut self.urids) }.unwrap();
//!             for sub_atom in tuple.iter() {
//!                 match unsafe { sub_atom.get_body::<i32>(&mut self.urids) } {
//!                     Ok(integer) => {
//!                         assert_eq!(42, *integer);
//!                         continue
//!                     }
//!                     Err(_) => (),
//!                 }
//!                 match unsafe { sub_atom.get_body::<f32>(&mut self.urids) } {
//!                     Ok(float) => {
//!                         assert_eq!(17.0, *float);
//!                         continue
//!                     }
//!                     Err(_) => (),
//!                 }
//!                 panic!("Unknown property in object!");
//!             }
//!         }
//!     }
//!
//!     // Getting a debug URID map.
//!     let mut debug_map = DebugMap::new();
//!     let mut urids = unsafe {debug_map.create_cached_map()};
//!
//!     // Creating the plugin.
//!     let mut plugin = Plugin {
//!         in_port: AtomInputPort::new(),
//!         out_port: AtomOutputPort::new(),
//!         urids: urids,
//!     };
//!
//!     // Creating the atom space.
//!     let mut atom_space = vec![0u8; 256];
//!     let atom = unsafe { (atom_space.as_mut_ptr() as *mut Atom).as_mut() }.unwrap();
//!     *(atom.mut_size()) = 256 - 8;
//!
//!     // Connecting the ports.
//!     plugin.in_port.connect_port(atom as &Atom);
//!     plugin.out_port.connect_port(atom);
//!
//!     // Calling `run`.
//!     plugin.run();
use crate::atom::{array::*, *};
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use urid::URID;

/// Nice handle for the time unit.
///
/// It is primarily used as a parameter for the
/// [`Sequence`](type.Sequence.html) initialization method.
///
/// This type is not `repr(C)` and can not be directly used to interpret raw data.
#[derive(Clone, PartialEq, Debug)]
pub enum TimeUnit {
    Frames,
    Beats,
}

impl TimeUnit {
    /// Try to get a `TimeUnit` value from a urid.
    ///
    /// If the given uri is the same as the URID of
    /// [`uris::BEAT_TIME_URI`](../uris/constant.BEAT_TIME_URI.html), this method will return
    /// `TimeUnit::Beats`. Otherwise, it will return `Time::Frames`.
    pub fn from_urid(urid: URID, urids: &mut urid::CachedMap) -> TimeUnit {
        if urid == urids.map(unsafe { CStr::from_bytes_with_nul_unchecked(uris::BEAT_TIME_URI) }) {
            TimeUnit::Beats
        } else {
            TimeUnit::Frames
        }
    }

    /// Return the corresponding URID of the time unit.
    pub fn into_urid(&self, urids: &mut urid::CachedMap) -> URID {
        match self {
            TimeUnit::Frames => {
                urids.map(unsafe { CStr::from_bytes_with_nul_unchecked(uris::FRAME_TIME_URI) })
            }
            TimeUnit::Beats => {
                urids.map(unsafe { CStr::from_bytes_with_nul_unchecked(uris::BEAT_TIME_URI) })
            }
        }
    }
}

/// Nice handle for time stamps.
///
/// Time stamps can be given in frames since startup or in beats since startup.
///
/// This type is not `repr(C)` and can not be directly used to interpret raw data.
#[derive(Clone, PartialEq, Debug)]
pub enum TimeStamp {
    Frames(i64),
    Beats(f64),
}

impl TimeStamp {
    /// Get the time unit of the stamp.
    pub fn get_unit(&self) -> TimeUnit {
        match self {
            TimeStamp::Frames(_) => TimeUnit::Frames,
            TimeStamp::Beats(_) => TimeUnit::Beats,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
/// Raw representation of the time stamp.
///
/// The type of this union is depending on the context, in our case the time unit given in the
/// body header of the sequence.
///
/// This type is `repr(C)` and is used to interpret raw data.
union RawTimeStamp {
    frames: i64,
    beats: f64,
}

impl From<TimeStamp> for RawTimeStamp {
    fn from(other: TimeStamp) -> RawTimeStamp {
        match other {
            TimeStamp::Frames(frames) => RawTimeStamp { frames: frames },
            TimeStamp::Beats(beats) => RawTimeStamp { beats: beats },
        }
    }
}

#[repr(C)]
/// The header of a sequence.
///
/// It contains the time unit used by the time stamp of every event.
///
/// This type is `repr(C)` and is used to interpret raw data.
pub struct SequenceHeader {
    pub unit: URID,
    pub pad: u32,
}

impl ArrayAtomHeader for SequenceHeader {
    type InitializationParameter = TimeUnit;

    unsafe fn initialize<'a, W, T>(
        writer: &mut W,
        unit: &TimeUnit,
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
    {
        let header = SequenceHeader {
            unit: unit.into_urid(urids),
            pad: 0,
        };
        writer.write_sized(&header).map(|_| ())
    }
}

/// Atom tuple with time stamps for every contained atom.
///
/// Sequences are used to express real-time events that should be handled with frame- or
/// beat-perfect timing, for example midi events.
///
/// See the [module documentation](index.html) for more information.
pub type Sequence = ArrayAtomBody<SequenceHeader, u8>;

impl AtomBody for Sequence {
    type InitializationParameter = TimeUnit;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::SEQUENCE_TYPE_URI) }
    }

    unsafe fn initialize_body<'a, W>(
        writer: &mut W,
        parameter: &TimeUnit,
        urids: &mut urid::CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, parameter, urids)
    }

    fn create_ref<'a>(raw_data: &'a [u8]) -> Result<&'a Self, ()> {
        Self::__create_ref(raw_data)
    }
}

impl Sequence {
    pub fn time_unit(&self, urids: &mut urid::CachedMap) -> TimeUnit {
        TimeUnit::from_urid(self.header.unit, urids)
    }

    pub fn iter<'a>(
        &'a self,
        urids: &mut urid::CachedMap,
    ) -> impl Iterator<Item = (TimeStamp, &'a Atom)> {
        let time_unit = TimeUnit::from_urid(self.header.unit, urids);
        AtomIterator::new(&self.data).map(
            move |(raw_stamp, chunk): (&'a RawTimeStamp, &'a Atom)| -> (TimeStamp, &'a Atom) {
                let stamp = match time_unit {
                    TimeUnit::Frames => TimeStamp::Frames(unsafe { raw_stamp.frames }),
                    TimeUnit::Beats => TimeStamp::Beats(unsafe { raw_stamp.beats }),
                };
                (stamp, chunk)
            },
        )
    }
}

/// Extension for [`WritingFrame`](../frame/trait.WritingFrame.html) and
/// [`WritingFrameExt`](../frame/trait.WritingFrameExt.html) for vectors.
///
/// See the [module documentation](index.html) for more information.
pub trait SequenceWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Sequence> {
    fn push_event<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        time: TimeStamp,
        parameter: &A::InitializationParameter,
        urids: &mut urid::CachedMap,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        // Retrieving the time unit of the sequence.
        let header_unit: TimeUnit = {
            let atom_body = unsafe { self.get_atom_body(urids) }.unwrap();
            TimeUnit::from_urid(atom_body.header.unit, urids)
        };

        if header_unit != time.get_unit() {
            return Err(());
        }

        unsafe {
            self.write_sized(&RawTimeStamp::from(time.clone()))?;
            let mut frame = self.create_nested_frame::<A>(urids)?;
            A::initialize_body(&mut frame, parameter, urids)?;
            Ok(frame)
        }
    }
}

impl<'a, W> SequenceWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Sequence> {}
