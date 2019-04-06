use crate::prelude::*;
use crate::status_bytes::*;

/// A MIDI message.
///
/// Please consult the [MIDI reference](https://www.midi.org/specifications) for information on how
/// to use these messages.
pub enum MidiMessage {
    /// Stop playing a note.
    NoteOff { channel: u4, note: u7, velocity: u7 },
    /// Start playing a note.
    NoteOn { channel: u4, note: u7, velocity: u7 },
    /// Change the pressure on a key.
    PolyKeyPressure { channel: u4, pressure: u7 },
    /// Change the value of a controlled number.
    ControlChange {
        channel: u4,
        control_number: u7,
        control_value: u7,
    },
    /// Change the active program.
    ProgramChange { channel: u4, program_number: u7 },
    /// Change the pressure of the channel.
    ChannelPressure { channel: u4, pressure: u7 },
    /// Change the pitch bend.
    PitchBendChange { channel: u4, value: u14 },
    /// Synchronisation message.
    TimeCodeQuarterFrame { message_type: u3, value: u4 },
    /// Change the current position in a song.
    SongPositionPointer { position: u14 },
    /// Select another song.
    SongSelect { song: u7 },
    /// Tune analog oscillators.
    TuneRequest,
    /// A step of the timing clock.
    TimingClock,
    /// Start the playback.
    Start,
    /// Continue the playback.
    Continue,
    /// Stop the playback.
    Stop,
    /// Active sensing message.
    ActiveSensing,
    /// Reset the system.
    SystemReset,
}

/// Errors that may arise when using [`MidiMessage::try_from`](enum.MidiMessage.html#method.try_from)
#[derive(Debug)]
pub enum TryFromError {
    /// The first byte of the slice does not correspond to a known MIDI status byte.
    UnknownMessage,
    /// The message is a system-exclusive message.
    ///
    /// Please use the [`SystemExclusiveMessage`](struct.SystemExclusiveMessage.html) struct to
    /// interpret system-exclusive messages.
    SystemExclusiveMessage,
    /// The slice is to short for the message, or the message is incomplete.
    SliceToShort,
    /// The first byte of the slice is not a status byte.
    NoStatusByte,
    /// There are other status bytes in the slice except from the first one.
    ///
    /// LV2 does not allow multiple messages in one atom.
    InteriorStatusByte,
}

/// Split the status byte in the "raw" status byte and the channel number.
fn split_to_channel_status(status: u8) -> (u8, u4) {
    let channel_status = status & 0b11110000;
    let channel = u4::new(status & 0b00001111);
    (channel_status, channel)
}

/// Join to data bytes to a u14.
fn data_to_u14(lsb: u7, msb: u7) -> u14 {
    let lsb: u16 = lsb.into();
    let msb: u16 = msb.into();
    let value: u14 = u14::new((msb << 7) + lsb);
    value
}

impl MidiMessage {
    /// Try to create a `MidiMessage` from a one-byte-message.
    fn try_from_one_byte(status: u8) -> Result<Self, TryFromError> {
        match status {
            TUNE_REQUEST_STATUS => Ok(MidiMessage::TuneRequest),
            TIMING_CLOCK_STATUS => Ok(MidiMessage::TimingClock),
            START_STATUS => Ok(MidiMessage::Start),
            CONTINUE_STATUS => Ok(MidiMessage::Continue),
            STOP_STATUS => Ok(MidiMessage::Stop),
            ACTIVE_SENSING_STATUS => Ok(MidiMessage::ActiveSensing),
            SYSTEM_RESET_STATUS => Ok(MidiMessage::SystemReset),
            _ => Err(TryFromError::UnknownMessage),
        }
    }

    /// Try to create a `MidiMessage` from a two-byte-message.
    fn try_from_two_byte(status: u8, data: u7) -> Result<Self, TryFromError> {
        let (channel_status, channel) = split_to_channel_status(status);

        match channel_status {
            POLY_KEY_PRESSURE_STATUS => {
                return Ok(MidiMessage::PolyKeyPressure {
                    channel: channel,
                    pressure: data,
                });
            }
            PROGRAM_CHANGE_STATUS => {
                return Ok(MidiMessage::ProgramChange {
                    channel: channel,
                    program_number: data,
                });
            }
            CHANNEL_PRESSURE_STATUS => {
                return Ok(MidiMessage::ChannelPressure {
                    channel: channel,
                    pressure: data,
                });
            }
            _ => (),
        }

        match status {
            TIME_CODE_QUARTER_FRAME_STATUS => {
                let data: u8 = data.into();
                let message_type = u3::new((data & 0b01110000) >> 4);
                let value = u4::new(data & 0b00001111);
                Ok(MidiMessage::TimeCodeQuarterFrame {
                    message_type: message_type,
                    value: value,
                })
            }
            SONG_SELECT_STATUS => Ok(MidiMessage::SongSelect { song: data }),
            _ => Err(TryFromError::UnknownMessage),
        }
    }

    /// Try to create a `MidiMessage` from a three-byte-message.
    fn try_from_three_byte(
        status: u8,
        first_data: u7,
        second_data: u7,
    ) -> Result<Self, TryFromError> {
        let (channel_status, channel) = split_to_channel_status(status);

        match channel_status {
            NOTE_OFF_STATUS => {
                return Ok(MidiMessage::NoteOff {
                    channel: channel,
                    note: first_data,
                    velocity: second_data,
                });
            }
            NOTE_ON_STATUS => {
                return Ok(MidiMessage::NoteOn {
                    channel: channel,
                    note: first_data,
                    velocity: second_data,
                });
            }
            CONTROL_CHANGE_STATUS => {
                return Ok(MidiMessage::ControlChange {
                    channel: channel,
                    control_number: first_data,
                    control_value: second_data,
                });
            }
            PITCH_BEND_CHANGE_STATUS => {
                let value = data_to_u14(first_data, second_data);
                return Ok(MidiMessage::PitchBendChange {
                    channel: channel,
                    value: value,
                });
            }
            _ => (),
        }

        match status {
            SONG_POSITION_POINTER_STATUS => {
                let value = data_to_u14(first_data, second_data);
                Ok(MidiMessage::SongPositionPointer { position: value })
            }
            _ => Err(TryFromError::UnknownMessage),
        }
    }

    /// Try create a `MidiMessage` from a slice of bytes.
    ///
    /// This is pretty straight forward: Try to parse the data and create a `MidiMessage` object
    /// for it. Please note that this method does not support system-exclusive message due to
    /// their unorthodox nature. These are handled by the
    /// [`SystemExclusiveMessage`](struct.SystemExclusiveMessage.html) struct.
    ///
    /// The error cases are described in the `TryFromError` enum.
    pub fn try_from(slice: &[u8]) -> Result<Self, TryFromError> {
        if slice.len() == 0 {
            return Err(TryFromError::SliceToShort);
        }
        let status_byte = slice[0];

        let data: &[u8] = &slice[1..];

        for byte in data {
            if byte & 0b10000000 != 0 {
                return Err(TryFromError::InteriorStatusByte);
            }
        }
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u7, data.len()) };

        if data.len() == 0 {
            Self::try_from_one_byte(status_byte)
        } else if data.len() == 1 {
            Self::try_from_two_byte(status_byte, data[0])
        } else if data.len() == 2 {
            Self::try_from_three_byte(status_byte, data[0], data[1])
        } else if status_byte == START_OF_SYSTEM_EXCLUSIVE_STATUS {
            Err(TryFromError::SystemExclusiveMessage)
        } else {
            Err(TryFromError::UnknownMessage)
        }
    }
}
