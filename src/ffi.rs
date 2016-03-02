// Copyright 2014-2015 Sam Doshi (sam@metal-fish.co.uk)
// Copyright 2013-2014 Philippe Delrieu (philippe.delrieu@free.fr)
//
// Licensed under either of
//          Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
//          MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT).
// This file may not be copied, modified, or distributed except according to those terms.

use std::os::raw::{c_char, c_void};

pub type PmDeviceId = i32;

pub type PortMidiStream = c_void;

#[doc(hidden)]
pub type PmMessage = i32;

pub type PmTimestamp = u32;

pub const PM_NO_DEVICE : PmDeviceId = -1;

#[doc(hidden)]
#[repr(C)]
pub struct PmEvent {
    pub message : PmMessage,
    pub timestamp : PmTimestamp,
}

#[repr(C)]
pub struct PmDeviceInfo {
    pub struct_version: i32, /* < this internal structure version */
    pub interf : *const c_char, /* < underlying MIDI API, e.g. MMSystem or DirectX */
    pub name : *const c_char,    /* < device name, e.g. USB MidiSport 1x1 */
    pub input : i32, /* < true iff input is available */
    pub output : i32, /* < true iff output is available */
    pub opened : i32, /* < used by generic PortMidi code to do error checking on arguments */
}

#[derive(Debug)]
#[repr(C)]
pub enum PmError {
    PmNoError = 0,
    PmGotData = 1, /* < A "no error" return that also indicates data available */
    PmHostError = -10000,
    PmInvalidDeviceId = -9999, /* out of range or
                                * output device when input is requested or
                                * input device when output is requested or
                                * device is already opened
                                */
    PmInsufficientMemory = -9998,
    PmBufferTooSmall = -9997,
    PmBufferOverflow = -9996,
    PmBadPtr = -9995, /* PortMidiStream parameter is NULL or
                       * stream is not opened or
                       * stream is output when input is required or
                       * stream is input when output is required */
    PmBadData = -9994, /* illegal midi data, e.g. missing EOX */
    PmInternalError = -9993,
    PmBufferMaxSize = -9992, /* buffer is already as large as it can be */
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
        _ => None
    }
}

#[link(name = "portmidi")]
extern "C" {
    pub fn Pm_Initialize() -> PmError;
    pub fn Pm_Terminate()-> PmError;
    pub fn Pm_HasHostError(stream: *const PortMidiStream) -> i32;
    pub fn Pm_GetErrorText(errorCode: PmError) -> *const c_char;
    pub fn Pm_GetHostErrorText(msg: *const c_char, len: i32);
    pub fn Pm_CountDevices() -> i32;
    pub fn Pm_GetDefaultInputDeviceID() -> PmDeviceId;
    pub fn Pm_GetDefaultOutputDeviceID() -> PmDeviceId;
    pub fn Pm_GetDeviceInfo(id: PmDeviceId) -> *const PmDeviceInfo;
    pub fn Pm_OpenInput(stream: *const *const PortMidiStream, inputDevice: PmDeviceId,
                        inputDriverInfo: *const c_void, bufferSize: i32,
                        time_proc: *const c_void, time_info: *const c_void) -> PmError;
    pub fn Pm_OpenOutput(stream: *const *const PortMidiStream, outputDevice: PmDeviceId,
                         inputDriverInfo: *const c_void, bufferSize: i32,
                         time_proc: *const c_void, time_info: *const c_void,
                         latency: i32) -> PmError;
    pub fn Pm_Read(stream: *const PortMidiStream, buffer : *mut PmEvent, length: i32) -> i32;
    pub fn Pm_Abort(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Close(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Poll(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Write(stream: *const PortMidiStream, buffer: *const PmEvent, length: i32) -> PmError;
    pub fn Pm_WriteShort(stream: *const PortMidiStream, timestamp: PmTimestamp,
                         message: PmMessage) -> PmError;
}

