use std::fmt::{Display, Formatter, Error as FormatError};
use std::error::Error as StdError;
use std::convert::From;
use std::result;

use ffi;

pub type PortMidiDeviceId = i32;

pub type Result<T> = result::Result<T, Error>;
impl From<ffi::PmError> for Result<()> {
    fn from(err: ffi::PmError) -> Self {
        match err {
            ffi::PmError::PmNoError | ffi::PmError::PmGotData => Ok(()),
            _ => Err(Error::PortMidi(err)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    PortMidi(ffi::PmError),
    Unknown,
    Unimplemented,
}
impl From<ffi::PmError> for Error {
    fn from(err: ffi::PmError) -> Self {
        match err {
            err @ _ => Error::PortMidi(err),
        }
    }
}

// Midi events
// -----------
/// Represents a single midi message, see also `MidiEvent`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
}

impl MidiMessage {
    fn wrap(cmessage: ffi::PmMessage) -> MidiMessage {
        MidiMessage {
            status: ((cmessage) & 0xFF) as u8,
            data1: (((cmessage) >> 8) & 0xFF) as u8,
            data2: (((cmessage) >> 16) & 0xFF) as u8,
        }
    }

    pub fn unwrap(&self) -> ffi::PmMessage {
        ((((self.data2 as i32) << 16) & 0xFF0000) | (((self.data1 as i32) << 8) & 0xFF00) |
         ((self.status as i32) & 0xFF)) as i32
    }
}

/// Represents a time stamped midi event. See also `MidiMessage`
///
/// See the PortMidi documentation for how SysEx and midi realtime messages
/// are handled
///
/// TODO: what to do about the timestamp?
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiEvent {
    pub message: MidiMessage,
    pub timestamp: ffi::PmTimestamp,
}

impl MidiEvent {
    pub fn wrap(event: ffi::PmEvent) -> MidiEvent {
        MidiEvent {
            message: MidiMessage::wrap(event.message),
            timestamp: event.timestamp,
        }
    }

    pub fn unwrap(&self) -> ffi::PmEvent {
        ffi::PmEvent {
            message: self.message.unwrap(),
            timestamp: self.timestamp,
        }
    }
}
