// Copyright 2014-2015 Sam Doshi (sam@metal-fish.co.uk)
// Copyright 2013-2014 Philippe Delrieu (philippe.delrieu@free.fr)
//
// Licensed under either of
//          Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
//          MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT).
// This file may not be copied, modified, or distributed except according to those terms.

use std::os::raw::{c_char, c_void};
use ffi::types::*;

#[link(name = "portmidi")]
extern "C" {
    pub fn Pm_Initialize() -> PmError;
    pub fn Pm_Terminate() -> PmError;
    pub fn Pm_HasHostError(stream: *const PortMidiStream) -> i32;
    pub fn Pm_GetErrorText(errorCode: PmError) -> *const c_char;
    pub fn Pm_GetHostErrorText(msg: *const c_char, len: i32);
    pub fn Pm_CountDevices() -> i32;
    pub fn Pm_GetDefaultInputDeviceID() -> PmDeviceId;
    pub fn Pm_GetDefaultOutputDeviceID() -> PmDeviceId;
    pub fn Pm_GetDeviceInfo(id: PmDeviceId) -> *const PmDeviceInfo;
    pub fn Pm_OpenInput(stream: *const *const PortMidiStream,
                        inputDevice: PmDeviceId,
                        inputDriverInfo: Option<*const c_void>,
                        bufferSize: i32,
                        time_proc: Option<*const c_void>,
                        time_info: Option<*const c_void>)
                        -> PmError;
    pub fn Pm_OpenOutput(stream: *const *const PortMidiStream,
                         outputDevice: PmDeviceId,
                         inputDriverInfo: Option<*const c_void>,
                         bufferSize: i32,
                         time_proc: Option<*const c_void>,
                         time_info: Option<*const c_void>,
                         latency: i32)
                         -> PmError;
    pub fn Pm_Read(stream: *const PortMidiStream, buffer: *mut PmEvent, length: i32) -> i32;
    pub fn Pm_Abort(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Close(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Poll(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Write(stream: *const PortMidiStream, buffer: *const PmEvent, length: i32) -> PmError;
    pub fn Pm_WriteShort(stream: *const PortMidiStream,
                         timestamp: PmTimestamp,
                         message: PmMessage)
                         -> PmError;
}
