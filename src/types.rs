use std::fmt::{Display, Formatter, Error as FormatError};
use std::error::Error;
use std::convert::From;

use ffi;

pub type PortMidiDeviceId = i32;
pub type PortMidiResult<T> = Result<T, PortMidiError>;
impl From<ffi::PmError> for PortMidiResult<()> {
    fn from(err: ffi::PmError) -> Self {
        match err {
            ffi::PmError::PmNoError => Ok(()),
            ffi::PmError::PmGotData => Ok(()),
            ffi::PmError::PmHostError => Err(PortMidiError::HostError),
            ffi::PmError::PmInvalidDeviceId => Err(PortMidiError::InvalidDeviceId),
            ffi::PmError::PmInsufficientMemory => Err(PortMidiError::InsufficientMemory),
            ffi::PmError::PmBufferTooSmall => Err(PortMidiError::BufferTooSmall),
            ffi::PmError::PmBufferOverflow => Err(PortMidiError::BufferOverflow),
            ffi::PmError::PmBadPtr => Err(PortMidiError::BadPtr),
            ffi::PmError::PmBadData => Err(PortMidiError::BadData),
            ffi::PmError::PmInternalError => Err(PortMidiError::InternalError),
            ffi::PmError::PmBufferMaxSize => Err(PortMidiError::BufferMaxSize),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PortMidiError {
    HostError,
    InvalidDeviceId,
    InsufficientMemory,
    BufferTooSmall,
    BufferOverflow,
    BadPtr,
    BadData,
    InternalError,
    BufferMaxSize,
}
impl PortMidiError {
    // replace this by a call to get_error_text
    fn msg(&self) -> &'static str {
        match *self {
            PortMidiError::HostError => "Host error",
            PortMidiError::InvalidDeviceId => "Invalid device ID",
            PortMidiError::InsufficientMemory => "Insufficent memory",
            PortMidiError::BufferTooSmall => "Buffer is too small",
            PortMidiError::BufferOverflow => "Buffer has overflow",
            PortMidiError::BadPtr => "Bad pointer was supplied",
            PortMidiError::BadData => "Invalid MIDI message data",
            PortMidiError::InternalError => "Portmidi internal error",
            PortMidiError::BufferMaxSize => "Buffer cannot be made larger",
        }
    }
}
impl Display for PortMidiError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "{}", self.msg())
    }
}
impl Error for PortMidiError {
    fn description(&self) -> &str {
        self.msg()
    }
}
