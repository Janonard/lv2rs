use crate::atom::{array::*, *};
use crate::frame::{NestedFrame, WritingFrame, WritingFrameExt};
use crate::unknown::*;
use crate::uris;
use std::ffi::CStr;
use urid::URID;

#[derive(Clone, PartialEq, Debug)]
pub enum TimeUnit {
    Frames,
    Beats,
}

impl TimeUnit {
    pub fn try_from_urid(urid: URID, urids: &mut urid::CachedMap) -> Result<TimeUnit, ()> {
        if urid == 0 {
            Ok(TimeUnit::Frames)
        } else if urid
            == urids.map(unsafe { CStr::from_bytes_with_nul_unchecked(uris::BEAT_TIME_URI) })
        {
            Ok(TimeUnit::Beats)
        } else if urid
            == urids.map(unsafe { CStr::from_bytes_with_nul_unchecked(uris::FRAME_TIME_URI) })
        {
            Ok(TimeUnit::Frames)
        } else {
            Err(())
        }
    }

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

    unsafe fn widen_ref<'a>(
        header: &'a AtomHeader,
        urids: &mut urid::CachedMap,
    ) -> Result<&'a Atom<Self>, WidenRefError> {
        Self::__widen_ref(header, urids)
    }
}

impl Atom<Sequence> {
    pub fn iter<'a>(
        &'a self,
        urids: &mut urid::CachedMap,
    ) -> Result<impl Iterator<Item = (TimeStamp, &'a Atom<Unknown>)>, ()> {
        let time_unit = TimeUnit::try_from_urid(self.body.header.unit, urids)?;
        Ok(ChunkIterator::new(&self.body.data)
            .map(
                move |(raw_stamp, chunk): (&'a RawTimeStamp, &'a Atom<Unknown>)|
                    -> (TimeStamp, &'a Atom<Unknown>)
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
        urids: &mut urid::CachedMap,
    ) -> Result<NestedFrame<'b, 'a, A>, ()> {
        // Retrieving the time unit of the sequence.
        let header_unit: TimeUnit = {
            let atom = unsafe { self.get_atom(urids) }.unwrap();
            TimeUnit::try_from_urid(atom.body.header.unit, urids)
                .expect("Illegal time unit in atom sequence header")
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
