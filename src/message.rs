use ux::*;

pub enum StandardMidiMessage {
    NoteOff {
        channel: u4,
        note: u7,
        velocity: u7,
    },
    NoteOn {
        channel: u4,
        note: u7,
        velocity: u7,
    },
    PolyKeyPressure {
        channel: u4,
        pressure: u7,
    },
    ControlChange {
        channel: u4,
        control_number: u7,
        control_value: u7,
    },
    ProgramChange {
        channel: u4,
        program_number: u7,
    },
    ChannelPressure {
        channel: u4,
        pressure: u7,
    },
    PitchBendChange {
        channel: u4,
        value: u14,
    },
    TimeCodeQuarterFrame {
        message_type: u3,
        value: u4,
    },
    SongPositionPointer {
        position: u14,
    },
    SongSelect {
        song: u7,
    },
    TuneRequest,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    SystemReset,
}

type SystemExclusiveMessage<'a> = &'a [u7];

pub enum MidiMessage<'a> {
    Standard(StandardMidiMessage),
    SysEx(SystemExclusiveMessage<'a>),
}

pub const NOTE_OFF_STATUS: u8 = 0b10000000;
pub const NOTE_ON_STATUS: u8 = 0b10010000;
pub const POLY_KEY_PRESSURE_STATUS: u8 = 0b10100000;
pub const CONTROL_CHANGE_STATUS: u8 = 0b10110000;
pub const PROGRAM_CHANGE_STATUS: u8 = 0b11000000;
pub const CHANNEL_PRESSURE_STATUS: u8 = 0b11010000;
pub const PITCH_BEND_CHANGE_STATUS: u8 = 0b11100000;
pub const START_OF_SYSTEM_EXCLUSIVE_STATUS: u8 = 0b11110000;
pub const TIME_CODE_QUARTER_FRAME_STATUS: u8 = 0b11110001;
pub const SONG_POSITION_POINTER_STATUS: u8 = 0b11110010;
pub const SONG_SELECT_STATUS: u8 = 0b11110011;
pub const TUNE_REQUEST_STATUS: u8 = 0b11110110;
pub const END_OF_SYSTE_EXCLUSICE_STATUS: u8 = 0b11110111;
pub const TIMING_CLOCK_STATUS: u8 = 0b11111000;
pub const START_STATUS: u8 = 0b11111010;
pub const CONTINUE_STATUS: u8 = 0b11111011;
pub const STOP_STATUS: u8 = 0b11111100;
pub const ACTIVE_SENSING_STATUS: u8 = 0b11111110;
pub const SYSTEM_RESET_STATUS: u8 = 0b11111111;

#[derive(Debug)]
pub enum TryFromError {
    UnknownMessage,
    SliceToShort,
    NoStatusByte,
    NoEndOfSystemExclusive,
    InteriorStatusByte,
}

fn split_to_channel_status(status: u8) -> (u8, u4) {
    let channel_status = status & 0b11110000;
    let channel = u4::new(status & 0b00001111);
    (channel_status, channel)
}

fn data_to_u14(lsb: u7, msb: u7) -> u14 {
    let lsb: u16 = lsb.into();
    let msb: u16 = msb.into();
    let value: u14 = u14::new((msb << 7) + lsb);
    value
}

impl<'a> MidiMessage<'a> {
    fn try_from_one_byte(status: u8) -> Result<Self, TryFromError> {
        match status {
            TUNE_REQUEST_STATUS => Ok(MidiMessage::Standard(StandardMidiMessage::TuneRequest)),
            TIMING_CLOCK_STATUS => Ok(MidiMessage::Standard(StandardMidiMessage::TimingClock)),
            START_STATUS => Ok(MidiMessage::Standard(StandardMidiMessage::Start)),
            CONTINUE_STATUS => Ok(MidiMessage::Standard(StandardMidiMessage::Continue)),
            STOP_STATUS => Ok(MidiMessage::Standard(StandardMidiMessage::Stop)),
            ACTIVE_SENSING_STATUS => Ok(MidiMessage::Standard(StandardMidiMessage::ActiveSensing)),
            SYSTEM_RESET_STATUS => Ok(MidiMessage::Standard(StandardMidiMessage::SystemReset)),
            _ => Err(TryFromError::UnknownMessage),
        }
    }

    fn try_from_two_byte(status: u8, data: u7) -> Result<Self, TryFromError> {
        let (channel_status, channel) = split_to_channel_status(status);

        match channel_status {
            POLY_KEY_PRESSURE_STATUS => {
                return Ok(MidiMessage::Standard(
                    StandardMidiMessage::PolyKeyPressure {
                        channel: channel,
                        pressure: data,
                    },
                ));
            }
            PROGRAM_CHANGE_STATUS => {
                return Ok(MidiMessage::Standard(StandardMidiMessage::ProgramChange {
                    channel: channel,
                    program_number: data,
                }));
            }
            CHANNEL_PRESSURE_STATUS => {
                return Ok(MidiMessage::Standard(
                    StandardMidiMessage::ChannelPressure {
                        channel: channel,
                        pressure: data,
                    },
                ));
            }
            _ => (),
        }

        match status {
            TIME_CODE_QUARTER_FRAME_STATUS => {
                let data: u8 = data.into();
                let message_type = u3::new((data & 0b01110000) >> 4);
                let value = u4::new(data & 0b00001111);
                Ok(MidiMessage::Standard(
                    StandardMidiMessage::TimeCodeQuarterFrame {
                        message_type: message_type,
                        value: value,
                    },
                ))
            }
            SONG_SELECT_STATUS => Ok(MidiMessage::Standard(StandardMidiMessage::SongSelect {
                song: data,
            })),
            _ => Err(TryFromError::UnknownMessage),
        }
    }

    fn try_from_three_byte(
        status: u8,
        first_data: u7,
        second_data: u7,
    ) -> Result<Self, TryFromError> {
        let (channel_status, channel) = split_to_channel_status(status);

        match channel_status {
            NOTE_OFF_STATUS => {
                return Ok(MidiMessage::Standard(StandardMidiMessage::NoteOff {
                    channel: channel,
                    note: first_data,
                    velocity: second_data,
                }));
            }
            NOTE_ON_STATUS => {
                return Ok(MidiMessage::Standard(StandardMidiMessage::NoteOn {
                    channel: channel,
                    note: first_data,
                    velocity: second_data,
                }));
            }
            CONTROL_CHANGE_STATUS => {
                return Ok(MidiMessage::Standard(StandardMidiMessage::ControlChange {
                    channel: channel,
                    control_number: first_data,
                    control_value: second_data,
                }));
            }
            PITCH_BEND_CHANGE_STATUS => {
                let value = data_to_u14(first_data, second_data);
                return Ok(MidiMessage::Standard(
                    StandardMidiMessage::PitchBendChange {
                        channel: channel,
                        value: value,
                    },
                ));
            }
            _ => (),
        }

        match status {
            SONG_POSITION_POINTER_STATUS => {
                let value = data_to_u14(first_data, second_data);
                Ok(MidiMessage::Standard(
                    StandardMidiMessage::SongPositionPointer { position: value },
                ))
            }
            _ => Err(TryFromError::UnknownMessage),
        }
    }

    pub fn try_from(slice: &'a [u8]) -> Result<Self, TryFromError> {
        if slice.len() == 0 {
            return Err(TryFromError::SliceToShort);
        }
        let status_byte = slice[0];

        let data: &[u8] = if status_byte == START_OF_SYSTEM_EXCLUSIVE_STATUS {
            if slice.len() == 1 {
                return Err(TryFromError::SliceToShort);
            }
            if slice[slice.len() - 1] != END_OF_SYSTE_EXCLUSICE_STATUS {
                return Err(TryFromError::NoEndOfSystemExclusive);
            }
            &slice[1..slice.len() - 1]
        } else {
            &slice[1..]
        };

        for byte in data {
            if byte & 0b10000000 != 0 {
                return Err(TryFromError::InteriorStatusByte);
            }
        }
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u7, data.len()) };

        if status_byte == START_OF_SYSTEM_EXCLUSIVE_STATUS {
            Ok(MidiMessage::SysEx(data))
        } else if data.len() == 0 {
            Self::try_from_one_byte(status_byte)
        } else if data.len() == 1 {
            Self::try_from_two_byte(status_byte, data[0])
        } else if data.len() == 2 {
            Self::try_from_three_byte(status_byte, data[0], data[1])
        } else {
            Err(TryFromError::UnknownMessage)
        }
    }
}
