use std::fmt::{Display, Formatter, Error as FormatError};
use std::error::Error;

pub type PortMidiDeviceId = i32;
pub type PortMidiResult<T> = Result<T, PortMidiError>;

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
