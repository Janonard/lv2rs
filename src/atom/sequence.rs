use crate::atom::array::{ArrayAtomBody, ArrayAtomHeader};
use crate::atom::chunk::*;
use crate::atom::{Atom, AtomBody, AtomHeader};
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::uris;
use std::ffi::CStr;
use urid::URID;

#[derive(Clone, PartialEq, Debug)]
pub enum TimeUnit {
    Frames,
    Beats,
}

impl TimeUnit {
    pub fn try_from_urid(urid: URID, mapped_urids: &uris::MappedURIDs) -> Result<TimeUnit, ()> {
        if urid == mapped_urids.beat_time {
            Ok(TimeUnit::Beats)
        } else if urid == mapped_urids.frame_time {
            Ok(TimeUnit::Frames)
        } else {
            Err(())
        }
    }

    pub fn into_urid(&self, mapped_urids: &uris::MappedURIDs) -> URID {
        match self {
            TimeUnit::Frames => mapped_urids.frame_time,
            TimeUnit::Beats => mapped_urids.beat_time,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TimeStamp {
    Frames(i64),
    Beats(f64),
}

impl TimeStamp {
    pub fn get_unit(&self) -> TimeUnit {
        match self {
            TimeStamp::Frames(_) => TimeUnit::Frames,
            TimeStamp::Beats(_) => TimeUnit::Beats,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
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
pub struct SequenceHeader {
    pub unit: URID,
    pub pad: u32,
}

impl ArrayAtomHeader for SequenceHeader {
    type InitializationParameter = TimeUnit;

    fn initialize<'a, W, T>(writer: &mut W, unit: &TimeUnit) -> Result<(), ()>
    where
        T: 'static + Sized + Copy,
        ArrayAtomBody<Self, T>: AtomBody,
        W: WritingFrame<'a> + WritingFrameExt<'a, ArrayAtomBody<Self, T>>,
    {
        let header = SequenceHeader {
            unit: unit.into_urid(unsafe { uris::MappedURIDs::get_map() }),
            pad: 0,
        };
        unsafe { writer.write_sized(&header) }.map(|_| ())
    }
}

pub type Sequence = ArrayAtomBody<SequenceHeader, u8>;

impl AtomBody for Sequence {
    type InitializationParameter = TimeUnit;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(uris::SEQUENCE_TYPE_URI) }
    }

    fn get_urid(urids: &uris::MappedURIDs) -> URID {
        urids.sequence
    }

    unsafe fn initialize_body<'a, W>(writer: &mut W, parameter: &TimeUnit) -> Result<(), ()>
    where
        W: WritingFrame<'a> + WritingFrameExt<'a, Self>,
    {
        Self::__initialize_body(writer, parameter)
    }

    unsafe fn widen_ref(header: &AtomHeader) -> Result<&Atom<Self>, ()> {
        Self::__widen_ref(header)
    }
}

impl Atom<Sequence> {
    pub fn iter<'a>(
        &'a self,
        urids: &uris::MappedURIDs,
    ) -> Result<impl Iterator<Item = (TimeStamp, &'a Atom<Chunk>)>, ()> {
        let time_unit = TimeUnit::try_from_urid(self.body.header.unit, urids)?;
        Ok(ChunkIterator::new(&self.body.data)
            .map(
                move |(raw_stamp, chunk): (&'a RawTimeStamp, &'a Atom<Chunk>)|
                    -> (TimeStamp, &'a Atom<Chunk>)
                {
                    let stamp = match time_unit {
                        TimeUnit::Frames => TimeStamp::Frames(unsafe {raw_stamp.frames}),
                        TimeUnit::Beats => TimeStamp::Beats(unsafe {raw_stamp.beats}),
                    };
                    (stamp, chunk)
                }
            )
        )
    }
}

pub trait SequenceWritingFrame<'a>: WritingFrame<'a> + WritingFrameExt<'a, Sequence> {
    fn push_event<'b, A: AtomBody + ?Sized>(
        &'b mut self,
        time: TimeStamp,
        parameter: &A::InitializationParameter,
        urids: &uris::MappedURIDs,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        // Retrieving the time unit of the sequence.
        let header_unit: TimeUnit = {
            let atom = unsafe { self.get_atom() }.unwrap();
            TimeUnit::try_from_urid(atom.body.header.unit, urids)
                .expect("Illegal time unit in atom sequence header")
        };

        if header_unit != time.get_unit() {
            return Err(());
        }

        unsafe {
            self.write_sized(&RawTimeStamp::from(time.clone())) ?;
            let mut frame = self.create_atom_frame::<A>(urids) ?;
            A::initialize_body(&mut frame, parameter)?;
            Ok(frame)
        }
    }
}

impl<'a, W> SequenceWritingFrame<'a> for W where W: WritingFrame<'a> + WritingFrameExt<'a, Sequence> {}
