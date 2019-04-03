use crate::message::*;
use lv2rs_atom::atom::*;
use lv2rs_atom::prelude::*;
use lv2rs_urid::CachedMap;
use std::ffi::CStr;
use ux::*;

pub struct RawMidiMessage([u8]);

impl RawMidiMessage {
    pub fn interpret(&self) -> Result<MidiMessage, TryFromError> {
        MidiMessage::try_from(&self.0)
    }
}

impl<'a> AtomBody for RawMidiMessage {
    type InitializationParameter = MidiMessage;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(crate::uris::EVENT_URI) }
    }

    unsafe fn initialize_body<'b, W>(
        writer: &mut W,
        message: &MidiMessage,
        _urids: &mut CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'b> + WritingFrameExt<'b, Self>,
    {
        match message {
            MidiMessage::NoteOff {
                channel,
                note,
                velocity,
            } => {
                write_channel_status(writer, NOTE_OFF_STATUS, *channel)?;
                write_data(writer, *note)?;
                write_data(writer, *velocity)?;
            }
            MidiMessage::NoteOn {
                channel,
                note,
                velocity,
            } => {
                write_channel_status(writer, NOTE_ON_STATUS, *channel)?;
                write_data(writer, *note)?;
                write_data(writer, *velocity)?;
            }
            MidiMessage::PolyKeyPressure { channel, pressure } => {
                write_channel_status(writer, POLY_KEY_PRESSURE_STATUS, *channel)?;
                write_data(writer, *pressure)?;
            }
            MidiMessage::ControlChange {
                channel,
                control_number,
                control_value,
            } => {
                write_channel_status(writer, CONTROL_CHANGE_STATUS, *channel)?;
                write_data(writer, *control_number)?;
                write_data(writer, *control_value)?;
            }
            MidiMessage::ProgramChange {
                channel,
                program_number,
            } => {
                write_channel_status(writer, PROGRAM_CHANGE_STATUS, *channel)?;
                write_data(writer, *program_number)?;
            }
            MidiMessage::ChannelPressure { channel, pressure } => {
                write_channel_status(writer, CHANNEL_PRESSURE_STATUS, *channel)?;
                write_data(writer, *pressure)?;
            }
            MidiMessage::PitchBendChange { channel, value } => {
                write_channel_status(writer, PITCH_BEND_CHANGE_STATUS, *channel)?;
                write_u14_data(writer, *value)?;
            }
            MidiMessage::TimeCodeQuarterFrame {
                message_type,
                value,
            } => {
                writer.write_sized(&TIME_CODE_QUARTER_FRAME_STATUS)?;
                let message_type: u8 = (*message_type).into();
                let value: u8 = (*value).into();
                let byte: u8 = value + (message_type << 4);
                writer.write_sized(&byte)?;
            }
            MidiMessage::SongPositionPointer { position } => {
                writer.write_sized(&SONG_POSITION_POINTER_STATUS)?;
                write_u14_data(writer, *position)?;
            }
            MidiMessage::SongSelect { song } => {
                writer.write_sized(&SONG_SELECT_STATUS)?;
                write_data(writer, *song)?;
            }
            MidiMessage::TuneRequest => {
                writer.write_sized(&TUNE_REQUEST_STATUS)?;
            }
            MidiMessage::TimingClock => {
                writer.write_sized(&TIMING_CLOCK_STATUS)?;
            }
            MidiMessage::Start => {
                writer.write_sized(&START_STATUS)?;
            }
            MidiMessage::Continue => {
                writer.write_sized(&CONTINUE_STATUS)?;
            }
            MidiMessage::Stop => {
                writer.write_sized(&STOP_STATUS)?;
            }
            MidiMessage::ActiveSensing => {
                writer.write_sized(&ACTIVE_SENSING_STATUS)?;
            }
            MidiMessage::SystemReset => {
                writer.write_sized(&SYSTEM_RESET_STATUS)?;
            }
        }
        Ok(())
    }

    unsafe fn widen_ref<'b>(
        header: &'b AtomHeader,
        _urids: &mut CachedMap,
    ) -> Result<&'b Atom<Self>, WidenRefError> {
        let size: usize = header.size as usize;
        let fat_ptr: (*const AtomHeader, usize) = (header, size);
        let fat_ptr: *const Atom<Self> = std::mem::transmute(fat_ptr);
        let atom = fat_ptr.as_ref().unwrap();
        if atom.body.0[0] != crate::message::START_OF_SYSTEM_EXCLUSIVE_STATUS {
            Ok(atom)
        } else {
            Err(WidenRefError::MalformedAtom)
        }
    }
}

pub struct SystemExclusiveMessage([u8]);

impl SystemExclusiveMessage {
    pub fn get_data(&self) -> &[u8] {
        let data = &self.0;
        let len = data.len();
        &data[1..len - 1]
    }
}

impl<'a> AtomBody for SystemExclusiveMessage {
    type InitializationParameter = Box<[u8]>;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(crate::uris::EVENT_URI) }
    }

    unsafe fn initialize_body<'b, W>(
        writer: &mut W,
        data: &Box<[u8]>,
        _urids: &mut CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'b> + WritingFrameExt<'b, Self>,
    {
        writer.write_sized(&START_OF_SYSTEM_EXCLUSIVE_STATUS)?;
        let data: &[u8] = std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len());
        writer.write_raw(data)?;
        writer.write_sized(&END_OF_SYSTE_EXCLUSICE_STATUS)?;
        Ok(())
    }

    unsafe fn widen_ref<'b>(
        header: &'b AtomHeader,
        _urids: &mut CachedMap,
    ) -> Result<&'b Atom<Self>, WidenRefError> {
        // Creating the reference.
        let size: usize = header.size as usize;
        let fat_ptr: (*const AtomHeader, usize) = (header, size);
        let fat_ptr: *const Atom<Self> = std::mem::transmute(fat_ptr);
        let atom = fat_ptr.as_ref().unwrap();

        // Assuring a minimal length of two bytes.
        let data = &atom.body.0;
        if data.len() < 2 {
            return Err(WidenRefError::MalformedAtom);
        }

        // Check the first and the last byte to be the correct status bytes.
        let first_byte: u8 = *data.first().unwrap();
        let last_byte: u8 = *data.last().unwrap();
        if (first_byte != crate::message::START_OF_SYSTEM_EXCLUSIVE_STATUS)
            | (last_byte != crate::message::END_OF_SYSTE_EXCLUSICE_STATUS)
        {
            return Err(WidenRefError::MalformedAtom);
        }

        // Check for interior status bytes.
        // Original MIDI allows some of them, but LV2 doesn't.
        for byte in &data[1..data.len() - 1] {
            if (*byte & 0b1000_0000) != 0 {
                return Err(WidenRefError::MalformedAtom);
            }
        }

        Ok(atom)
    }
}

unsafe fn write_channel_status<'a, W, A>(writer: &mut W, status: u8, channel: u4) -> Result<(), ()>
where
    W: WritingFrame<'a> + WritingFrameExt<'a, A>,
    A: AtomBody + ?Sized,
{
    let channel: u8 = channel.into();
    let status = status + channel;
    writer.write_sized(&status).map(|_| ())
}

unsafe fn write_data<'a, W, A>(writer: &mut W, data: u7) -> Result<(), ()>
where
    W: WritingFrame<'a> + WritingFrameExt<'a, A>,
    A: AtomBody + ?Sized,
{
    let data: u8 = data.into();
    writer.write_sized(&data).map(|_| ())
}

unsafe fn write_u14_data<'a, W, A>(writer: &mut W, data: u14) -> Result<(), ()>
where
    W: WritingFrame<'a> + WritingFrameExt<'a, A>,
    A: AtomBody + ?Sized,
{
    let data: u16 = data.into();
    let msb: u8 = ((data & 0b0011_1111_1000_0000) >> 7) as u8;
    let lsb: u8 = (data & 0b0000_0000_0111_1111) as u8;
    writer.write_sized(&lsb)?;
    writer.write_sized(&msb)?;
    Ok(())
}
