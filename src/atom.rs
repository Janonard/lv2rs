use crate::message::*;
use lv2rs_atom::prelude::*;
use lv2rs_atom::atom::*;
use lv2rs_urid::CachedMap;
use std::ffi::CStr;
use ux::*;

pub struct RawMidiMessage([u8]);

pub enum MidiMessageParameter {
    Standard(StandardMidiMessage),
    SysEx(Box<[u7]>),
}

unsafe fn write_channel_status<'a, W>(writer: &mut W, status: u8, channel: u4) -> Result<(), ()>
where
    W: WritingFrame<'a> + WritingFrameExt<'a, RawMidiMessage>,
{
    let channel: u8 = channel.into();
    let status = status + channel;
    writer.write_sized(&status).map(|_| ())
}

unsafe fn write_data<'a, W>(writer: &mut W, data: u7) -> Result<(), ()>
where
    W: WritingFrame<'a> + WritingFrameExt<'a, RawMidiMessage>,
{
    let data: u8 = data.into();
    writer.write_sized(&data).map(|_| ())
}

unsafe fn write_u14_data<'a, W>(writer: &mut W, data: u14) -> Result<(), ()>
where
    W: WritingFrame<'a> + WritingFrameExt<'a, RawMidiMessage>,
{
    let data: u16 = data.into();
    let msb: u8 = ((data & 0b0011_1111_1000_0000) >> 7) as u8;
    let lsb: u8 = (data & 0b0000_0000_0111_1111) as u8;
    writer.write_sized(&lsb)?;
    writer.write_sized(&msb)?;
    Ok(())
}

impl RawMidiMessage {
    unsafe fn initialize_standard_message<'b, W>(
        writer: &mut W,
        message: &StandardMidiMessage,
        _urids: &mut CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'b> + WritingFrameExt<'b, Self>,
    {
        match message {
            StandardMidiMessage::NoteOff {
                channel,
                note,
                velocity,
            } => {
                write_channel_status(writer, NOTE_OFF_STATUS, *channel)?;
                write_data(writer, *note)?;
                write_data(writer, *velocity)?;
            }
            StandardMidiMessage::NoteOn {
                channel,
                note,
                velocity,
            } => {
                write_channel_status(writer, NOTE_ON_STATUS, *channel)?;
                write_data(writer, *note)?;
                write_data(writer, *velocity)?;
            }
            StandardMidiMessage::PolyKeyPressure { channel, pressure } => {
                write_channel_status(writer, POLY_KEY_PRESSURE_STATUS, *channel)?;
                write_data(writer, *pressure)?;
            }
            StandardMidiMessage::ControlChange {
                channel,
                control_number,
                control_value,
            } => {
                write_channel_status(writer, CONTROL_CHANGE_STATUS, *channel)?;
                write_data(writer, *control_number)?;
                write_data(writer, *control_value)?;
            }
            StandardMidiMessage::ProgramChange {
                channel,
                program_number,
            } => {
                write_channel_status(writer, PROGRAM_CHANGE_STATUS, *channel)?;
                write_data(writer, *program_number)?;
            }
            StandardMidiMessage::ChannelPressure { channel, pressure } => {
                write_channel_status(writer, CHANNEL_PRESSURE_STATUS, *channel)?;
                write_data(writer, *pressure)?;
            }
            StandardMidiMessage::PitchBendChange { channel, value } => {
                write_channel_status(writer, PITCH_BEND_CHANGE_STATUS, *channel)?;
                write_u14_data(writer, *value)?;
            }
            StandardMidiMessage::TimeCodeQuarterFrame {
                message_type,
                value,
            } => {
                writer.write_sized(&TIME_CODE_QUARTER_FRAME_STATUS)?;
                let message_type: u8 = (*message_type).into();
                let value: u8 = (*value).into();
                let byte: u8 = value + (message_type << 4);
                writer.write_sized(&byte)?;
            }
            StandardMidiMessage::SongPositionPointer { position } => {
                writer.write_sized(&SONG_POSITION_POINTER_STATUS)?;
                write_u14_data(writer, *position)?;
            }
            StandardMidiMessage::SongSelect { song } => {
                writer.write_sized(&SONG_SELECT_STATUS)?;
                write_data(writer, *song)?;
            }
            StandardMidiMessage::TuneRequest => {
                writer.write_sized(&TUNE_REQUEST_STATUS)?;
            }
            StandardMidiMessage::TimingClock => {
                writer.write_sized(&TIMING_CLOCK_STATUS)?;
            }
            StandardMidiMessage::Start => {
                writer.write_sized(&START_STATUS)?;
            }
            StandardMidiMessage::Continue => {
                writer.write_sized(&CONTINUE_STATUS)?;
            }
            StandardMidiMessage::Stop => {
                writer.write_sized(&STOP_STATUS)?;
            }
            StandardMidiMessage::ActiveSensing => {
                writer.write_sized(&ACTIVE_SENSING_STATUS)?;
            }
            StandardMidiMessage::SystemReset => {
                writer.write_sized(&SYSTEM_RESET_STATUS)?;
            }
        }
        Ok(())
    }

    unsafe fn initialize_sysex_message<'b, W>(
        writer: &mut W,
        data: &Box<[u7]>,
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

    pub fn interpret<'a>(&'a self) -> Result<MidiMessage<'a>, TryFromError> {
        MidiMessage::try_from(&self.0)
    }
}

impl<'a> AtomBody for RawMidiMessage {
    type InitializationParameter = MidiMessageParameter;

    fn get_uri() -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(crate::uris::EVENT_URI) }
    }

    unsafe fn initialize_body<'b, W>(
        writer: &mut W,
        parameter: &MidiMessageParameter,
        urids: &mut CachedMap,
    ) -> Result<(), ()>
    where
        W: WritingFrame<'b> + WritingFrameExt<'b, Self>,
    {
        match parameter {
            MidiMessageParameter::Standard(message) => {
                Self::initialize_standard_message(writer, message, urids)
            }
            MidiMessageParameter::SysEx(data) => {
                Self::initialize_sysex_message(writer, data, urids)
            }
        }
    }

    unsafe fn widen_ref<'b>(
        header: &'b AtomHeader,
        _urids: &mut CachedMap,
    ) -> Result<&'b Atom<Self>, WidenRefError> {
        let size: usize = header.size as usize;
        let fat_ptr: (*const AtomHeader, usize) = (header, size);
        let fat_ptr: *const Atom<Self> = std::mem::transmute(fat_ptr);
        Ok(fat_ptr.as_ref().unwrap())
    }
}
