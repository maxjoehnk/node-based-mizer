use std::convert::TryFrom;

/// Represents a Midi channel
///
/// Note that `Ch1 = 0`, `Ch2 = 1`, etc, as the actual protocol is 0-indexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel {
    Ch1 = 0,
    Ch2 = 1,
    Ch3 = 2,
    Ch4 = 3,
    Ch5 = 4,
    Ch6 = 5,
    Ch7 = 6,
    Ch8 = 7,
    Ch9 = 8,
    Ch10 = 9,
    Ch11 = 10,
    Ch12 = 11,
    Ch13 = 12,
    Ch14 = 13,
    Ch15 = 14,
    Ch16 = 15,
}

impl PartialEq<u8> for Channel {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Channel::Ch1 => *other == 1,
            Channel::Ch2 => *other == 2,
            Channel::Ch3 => *other == 3,
            Channel::Ch4 => *other == 4,
            Channel::Ch5 => *other == 5,
            Channel::Ch6 => *other == 6,
            Channel::Ch7 => *other == 7,
            Channel::Ch8 => *other == 8,
            Channel::Ch9 => *other == 9,
            Channel::Ch10 => *other == 10,
            Channel::Ch11 => *other == 11,
            Channel::Ch12 => *other == 12,
            Channel::Ch13 => *other == 13,
            Channel::Ch14 => *other == 14,
            Channel::Ch15 => *other == 15,
            Channel::Ch16 => *other == 16,
        }
    }
}

impl Channel {
    fn try_parse(data: u8) -> Result<Self, ()> {
        match data & 0b00001111 {
            0 => Ok(Channel::Ch1),
            1 => Ok(Channel::Ch2),
            2 => Ok(Channel::Ch3),
            3 => Ok(Channel::Ch4),
            4 => Ok(Channel::Ch5),
            5 => Ok(Channel::Ch6),
            6 => Ok(Channel::Ch7),
            7 => Ok(Channel::Ch8),
            8 => Ok(Channel::Ch9),
            9 => Ok(Channel::Ch10),
            10 => Ok(Channel::Ch11),
            11 => Ok(Channel::Ch12),
            12 => Ok(Channel::Ch13),
            13 => Ok(Channel::Ch14),
            14 => Ok(Channel::Ch15),
            15 => Ok(Channel::Ch16),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Channel {
    type Error = ();

    fn try_from(channel: u8) -> Result<Self, Self::Error> {
        match channel {
            1 => Ok(Channel::Ch1),
            2 => Ok(Channel::Ch2),
            3 => Ok(Channel::Ch3),
            4 => Ok(Channel::Ch4),
            5 => Ok(Channel::Ch5),
            6 => Ok(Channel::Ch6),
            7 => Ok(Channel::Ch7),
            8 => Ok(Channel::Ch8),
            9 => Ok(Channel::Ch9),
            10 => Ok(Channel::Ch10),
            11 => Ok(Channel::Ch11),
            12 => Ok(Channel::Ch12),
            13 => Ok(Channel::Ch13),
            14 => Ok(Channel::Ch14),
            15 => Ok(Channel::Ch15),
            16 => Ok(Channel::Ch16),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MidiMessage {
    ControlChange(Channel, u8, u8),
    NoteOff(Channel, u8, u8),
    NoteOn(Channel, u8, u8),
    Sysex((u8, u8, u8), u8, Vec<u8>),
    Unknown(Vec<u8>),
}

impl TryFrom<&[u8]> for MidiMessage {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        match data {
            [status, note, value] if matches_status(status, NOTE_OFF) => {
                let channel = Channel::try_parse(*status).unwrap();
                Ok(MidiMessage::NoteOff(channel, *note, *value))
            }
            [status, note, value] if matches_status(status, NOTE_ON) => {
                let channel = Channel::try_parse(*status).unwrap();
                Ok(MidiMessage::NoteOn(channel, *note, *value))
            }
            [status, d1, d2] if matches_status(status, CONTROL_CHANGE) => {
                let channel = Channel::try_parse(*status).unwrap();
                Ok(MidiMessage::ControlChange(channel, *d1, *d2))
            }
            [SYSEX, manu1, manu2, manu3, model, data @ .., SYSEX_EOX] => Ok(MidiMessage::Sysex(
                (*manu1, *manu2, *manu3),
                *model,
                data.to_vec(),
            )),
            _ => {
                log::warn!("unimplemented: {:?}", data);
                Ok(MidiMessage::Unknown(data.to_vec()))
            }
        }
    }
}

impl From<MidiMessage> for Vec<u8> {
    fn from(msg: MidiMessage) -> Self {
        match msg {
            MidiMessage::NoteOn(channel, note, value) => {
                vec![status_byte(NOTE_ON, channel), note, value]
            }
            MidiMessage::NoteOff(channel, note, value) => {
                vec![status_byte(NOTE_OFF, channel), note, value]
            }
            MidiMessage::ControlChange(channel, note, value) => {
                vec![status_byte(CONTROL_CHANGE, channel), note, value]
            }
            MidiMessage::Sysex(manufacturer, model, mut data) => {
                let mut bytes = vec![SYSEX, manufacturer.0, manufacturer.1, manufacturer.2, model];
                bytes.append(&mut data);
                bytes.push(SYSEX_EOX);
                bytes
            }
            MidiMessage::Unknown(data) => data,
        }
    }
}

#[inline(always)]
fn matches_status(input: &u8, status: u8) -> bool {
    input & 0b11110000 == status << 4u8
}

/// Calculate the status byte for a given channel no.
#[inline(always)]
fn status_byte(status: u8, channel: Channel) -> u8 {
    (status & 0b00001111) * 16 + (channel as u8)
}

const NOTE_OFF: u8 = 8;
const NOTE_ON: u8 = 9;
const POLYPHONIC_PRESSURE: u8 = 10;
const CONTROL_CHANGE: u8 = 11;
const PROGRAM_CHANGE: u8 = 12;
const CHANNEL_PRESSURE: u8 = 13;
const PITCH_BEND: u8 = 14;

const SYSEX: u8 = 240;
const MTC_QUARTER_FRAME: u8 = 241;
const SONG_POSITION_POINTER: u8 = 242;
const SONG_SELECT: u8 = 243;
const TUNE_REQUEST: u8 = 246;
const SYSEX_EOX: u8 = 247;
const TIMING_CLOCK: u8 = 248;
const START: u8 = 250;
const CONTINUE: u8 = 251;
const STOP: u8 = 252;
const ACTIVE_SENSING: u8 = 254;
const SYSTEM_RESET: u8 = 255;

const CC_RPN_MSB: u8 = 101;
const CC_RPN_LSB: u8 = 100;
const CC_NRPN_MSB: u8 = 99;
const CC_NRPN_LSB: u8 = 98;
const CC_DATA_ENTRY_MSB: u8 = 6;
const CC_DATA_ENTRY_LSB: u8 = 38;

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use crate::message::{Channel, MidiMessage};

    #[test]
    fn deserialize_note_off_ch1_0_0() {
        let data: &[u8] = &vec![128, 0, 0];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::NoteOff(Channel::Ch1, 0, 0));
    }

    #[test]
    fn serialize_note_off_ch1_0_0() {
        let msg = MidiMessage::NoteOff(Channel::Ch1, 0, 0);
        let expected = vec![128, 0, 0];

        let result: Vec<u8> = msg.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn deserialize_note_off_ch2_127_127() {
        let data: &[u8] = &vec![129, 127, 127];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::NoteOff(Channel::Ch2, 127, 127));
    }

    #[test]
    fn serialize_note_off_ch2_127_127() {
        let expected = vec![129, 127, 127];
        let msg = MidiMessage::NoteOff(Channel::Ch2, 127, 127);

        let result: Vec<u8> = msg.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn deserialize_note_off_ch3_0_0() {
        let data: &[u8] = &vec![130, 0, 0];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::NoteOff(Channel::Ch3, 0, 0));
    }

    #[test]
    fn deserialize_note_on_ch1_0_0() {
        let data: &[u8] = &vec![144, 0, 0];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::NoteOn(Channel::Ch1, 0, 0));
    }

    #[test]
    fn deserialize_note_on_ch2_127_127() {
        let data: &[u8] = &vec![145, 127, 127];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::NoteOn(Channel::Ch2, 127, 127));
    }

    #[test]
    fn deserialize_note_on_ch3_0_0() {
        let data: &[u8] = &vec![146, 0, 0];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::NoteOn(Channel::Ch3, 0, 0));
    }

    #[test]
    fn deserialize_cc_ch1_0_0() {
        let data: &[u8] = &vec![176, 0, 0];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::ControlChange(Channel::Ch1, 0, 0));
    }

    #[test]
    fn deserialize_cc_ch1_0_127() {
        let data: &[u8] = &vec![176, 0, 127];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::ControlChange(Channel::Ch1, 0, 127));
    }

    #[test]
    fn deserialize_cc_ch2_127_0() {
        let data: &[u8] = &vec![177, 127, 0];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::ControlChange(Channel::Ch2, 127, 0));
    }

    #[test]
    fn deserialize_sysex() {
        let data: &[u8] = &vec![240, 0, 32, 41, 2, 10, 119, 2, 247];

        let msg = MidiMessage::try_from(data).unwrap();

        assert_eq!(msg, MidiMessage::Sysex((0, 32, 41), 2, vec![10, 119, 2]));
    }

    #[test]
    fn serialize_sysex() {
        let expected: [u8; 9] = [240, 0, 32, 41, 2, 10, 119, 2, 247];
        let msg = MidiMessage::Sysex((0, 32, 41), 2, vec![10, 119, 2]);

        let data: Vec<u8> = msg.into();

        assert_eq!(data, expected);
    }
}
