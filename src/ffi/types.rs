use ffi;
use std::default::Default;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_void};

pub type PmDeviceId = c_int;
pub type PortMidiStream = c_void;
pub type PmMessage = c_uint;

pub type PmTimestamp = u32;
pub const PM_NO_DEVICE: PmDeviceId = -1;
pub const MIDI_EOX: u8 = 0xf7;

#[derive(Copy, Clone, Debug)]
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
    /// this internal structure version
    pub struct_version: c_int,
    /// underlying MIDI API, e.g. MMSystem or DirectX
    pub interf: *const c_char,
    /// device name, e.g. USB MidiSport 1x1
    pub name: *const c_char,
    /// true iff input is available
    pub input: c_int,
    /// true iff output is available
    pub output: c_int,
    /// used by generic PortMidi code to do error checking on arguments
    pub opened: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
impl fmt::Display for PmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut host_error_text: [c_char; 1024] = [0; 1024];
        let str_ptr = match *self {
            PmError::PmHostError => unsafe {
                ffi::Pm_GetHostErrorText(
                    host_error_text.as_mut_ptr(),
                    host_error_text.len() as c_int,
                );
                host_error_text.as_ptr()
            },
            _ => unsafe { ffi::Pm_GetErrorText(*self) },
        };
        write!(f, "{}", ffi::ptr_to_string(str_ptr).unwrap())
    }
}
pub trait MaybeError<T> {
    fn try_from(err_code: T) -> Result<T, PmError>;
}
impl MaybeError<c_int> for PmError {
    fn try_from(err_code: c_int) -> Result<c_int, PmError> {
        match err_code {
            -10_000..=-9992 | 0 => unsafe { Err(mem::transmute(err_code)) },
            -9989 => Err(PmError::PmInvalidDeviceId),
            _ => Ok(err_code),
        }
    }
}
