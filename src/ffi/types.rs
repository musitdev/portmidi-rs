use std::os::raw::{c_char, c_void};
use std::mem;
use std::default::Default;

pub type PmDeviceId = i32;
pub type PortMidiStream = c_void;
pub type PmMessage = i32;
pub type PmTimestamp = u32;
pub const PM_NO_DEVICE: PmDeviceId = -1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PmEvent {
    pub message: PmMessage,
    pub timestamp: PmTimestamp,
}
impl Default for PmEvent {
    fn default() -> Self {
        PmEvent {
            message: 0,
            timestamp: 0,
        }
    }
}

#[repr(C)]
pub struct PmDeviceInfo {
    pub struct_version: i32, // < this internal structure version
    pub interf: *const c_char, // < underlying MIDI API, e.g. MMSystem or DirectX
    pub name: *const c_char, // < device name, e.g. USB MidiSport 1x1
    pub input: i32, // < true iff input is available
    pub output: i32, // < true iff output is available
    pub opened: i32, // < used by generic PortMidi code to do error checking on arguments
}

#[derive(Debug)]
#[repr(C)]
pub enum PmError {
    /// "no error" return that also indicates data available
    PmNoError = 0,
    /// "no error" return that also indicates data available
    PmGotData = 1,
    PmHostError = -10000,
    /// out of range or
    /// output device when input is requested or
    /// input device when output is requested or
    /// device is already opened
    PmInvalidDeviceId = -9999,
    PmInsufficientMemory = -9998,
    PmBufferTooSmall = -9997,
    PmBufferOverflow = -9996,
    /// PortMidiStream parameter is NULL or
    /// stream is not opened or
    /// stream is output when input is required or
    /// stream is input when output is required
    PmBadPtr = -9995,
    /// illegal midi data, e.g. missing EOX
    PmBadData = -9994,
    PmInternalError = -9993,
    /// buffer is already as large as it can be
    PmBufferMaxSize = -9992,
}
pub trait MaybeError<T> {
    fn try_from(err_code: T) -> Result<T, PmError>;
}
impl MaybeError<i32> for PmError {
    fn try_from(err_code: i32) -> Result<i32, PmError> {
        match err_code {
            -10_000...-9992 | 0 | 1 => unsafe { Err(mem::transmute(err_code)) },
            _ => Ok(err_code),
        }
    }
}
