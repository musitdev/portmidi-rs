use std::error;
use std::os::raw::c_int;
use std::convert::{From, Into};
use std::result;
use std::fmt;

use ffi;

pub type PortMidiDeviceId = c_int;

/// PortMidi result type.
pub type Result<T> = result::Result<T, Error>;
impl From<ffi::PmError> for Result<()> {
    fn from(err: ffi::PmError) -> Self {
        match err {
            ffi::PmError::PmNoError | ffi::PmError::PmGotData => Ok(()),
            _ => Err(Error::PortMidi(err)),
        }
    }
}

/// PortMidi error type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    PortMidi(ffi::PmError),
    Unknown,
    Unimplemented,
    NoDefaultDevice,
    NotAnInputDevice,
    NotAnOutputDevice,
    Invalid,
}
impl From<ffi::PmError> for Error {
    fn from(err: ffi::PmError) -> Self {
        match err {
            err @ _ => Error::PortMidi(err),
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::PortMidi(pm_err) => write!(f, "{}", pm_err),
            err @ _ => write!(f, "{:?}", err),
        }
    }
}
impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::PortMidi(pm_error)   => match pm_error {
                ffi::PmError::PmNoError             => "",
                ffi::PmError::PmGotData             => "PortMidi: `Illegal error number'",
                ffi::PmError::PmHostError           => "PortMidi: `Host error'",
                ffi::PmError::PmInvalidDeviceId     => "PortMidi: `Invalid device ID'",
                ffi::PmError::PmInsufficientMemory  => "PortMidi: `Insufficient memory'",
                ffi::PmError::PmBufferTooSmall      => "PortMidi: `Buffer too small'",
                ffi::PmError::PmBufferOverflow      => "PortMidi: `Buffer overflow'",
                ffi::PmError::PmBadPtr              => "PortMidi: `Bad pointer'",
                ffi::PmError::PmBadData             => "PortMidi: `Invalid MIDI message Data'",
                ffi::PmError::PmInternalError       => "PortMidi: `Internal PortMidi Error'",
                ffi::PmError::PmBufferMaxSize       => "PortMidi: `Buffer cannot be made larger'"
            },
            Error::Unknown              => "portmidi-rs: Unknown",
            Error::Unimplemented        => "portmidi-rs: Unimplemented",
            Error::NoDefaultDevice      => "portmidi-rs: No default device",
            Error::NotAnInputDevice     => "portmidi-rs: Not an input device",
            Error::NotAnOutputDevice    => "portmidi-rs: Not an output device",
            Error::Invalid              => "portmidi-rs: Invalid"
        }
    }
}


/// Represents a Midi message.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
    pub data3: u8,
}
impl From<[u8; 4]> for MidiMessage {
    fn from(raw: [u8; 4]) -> Self {
        MidiMessage {
            status: raw[0],
            data1: raw[1],
            data2: raw[2],
            data3: raw[3],
        }
    }
}
impl fmt::Display for MidiMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "status: {}, data: {}, {}, {}",
               self.status,
               self.data1,
               self.data2,
               self.data3)
    }
}
/// Converts a `PmMessage` to a `MidiMessage.
/// This can be used for `c_int` as well as `i32` because these are only type aliases.
impl From<ffi::PmMessage> for MidiMessage {
    fn from(raw: u32) -> Self {
        MidiMessage {
            status: (raw & 0x00_00_00_FF) as u8,
            data1: ((raw & 0x00_00_FF_00) >> 8) as u8,
            data2: ((raw & 0x00_FF_00_00) >> 16) as u8,
            data3: ((raw & 0xFF_00_00_00) >> 24) as u8,
        }
    }
}
impl Into<ffi::PmMessage> for MidiMessage {
    fn into(self) -> u32 {
        (((self.data3 as u32) << 24)) | (((self.data2 as u32) << 16)) | (((self.data1 as u32) << 8)) | self.status as u32
    }
}

/// Represents a time stamped midi event. See also `MidiMessage`
///
/// See the PortMidi documentation for how SysEx and midi realtime messages
/// are handled
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiEvent {
    pub message: MidiMessage,
    pub timestamp: ffi::PmTimestamp,
}
impl From<ffi::PmEvent> for MidiEvent {
    fn from(raw: ffi::PmEvent) -> Self {
        MidiEvent {
            message: MidiMessage::from(raw.message),
            timestamp: raw.timestamp,
        }
    }
}
impl From<MidiMessage> for MidiEvent {
    fn from(msg: MidiMessage) -> Self {
        MidiEvent {
            message: msg,
            timestamp: 0,
        }
    }
}
impl Into<ffi::PmEvent> for MidiEvent {
    fn into(self) -> ffi::PmEvent {
        ffi::PmEvent {
            message: MidiMessage::into(self.message),
            timestamp: self.timestamp,
        }
    }
}
