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
