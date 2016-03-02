use std::os::raw::{c_char, c_void};

pub type PmDeviceId = i32;
pub type PortMidiStream = c_void;
pub type PmMessage = i32;
pub type PmTimestamp = u32;
pub const PM_NO_DEVICE: PmDeviceId = -1;

#[repr(C)]
pub struct PmEvent {
    pub message: PmMessage,
    pub timestamp: PmTimestamp,
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
    PmNoError = 0,
    PmGotData = 1, // < A "no error" return that also indicates data available
    PmHostError = -10000,
    PmInvalidDeviceId = -9999, /* out of range or
                                * output device when input is requested or
                                * input device when output is requested or
                                * device is already opened
                                * */
    PmInsufficientMemory = -9998,
    PmBufferTooSmall = -9997,
    PmBufferOverflow = -9996,
    PmBadPtr = -9995, /* PortMidiStream parameter is NULL or
                       * stream is not opened or
                       * stream is output when input is required or
                       * stream is input when output is required */
    PmBadData = -9994, // illegal midi data, e.g. missing EOX
    PmInternalError = -9993,
    PmBufferMaxSize = -9992, // buffer is already as large as it can be
}

// while we wait for FromPrimitive to stabilise
pub fn tmp_from_primitive(i: i32) -> Option<PmError> {
    match i {
        0 => Some(PmError::PmNoError),
        1 => Some(PmError::PmGotData),
        -10000 => Some(PmError::PmHostError),
        -9999 => Some(PmError::PmInvalidDeviceId),
        -9998 => Some(PmError::PmInsufficientMemory),
        -9997 => Some(PmError::PmBufferTooSmall),
        -9996 => Some(PmError::PmBufferOverflow),
        -9995 => Some(PmError::PmBadPtr),
        -9994 => Some(PmError::PmBadData),
        -9993 => Some(PmError::PmInternalError),
        -9992 => Some(PmError::PmBufferMaxSize),
        _ => None,
    }
}
