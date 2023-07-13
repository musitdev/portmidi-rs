// Copyright 2014-2015 Sam Doshi (sam@metal-fish.co.uk)
// Copyright 2013-2014 Philippe Delrieu (philippe.delrieu@free.fr)
//
// Licensed under either of
//          Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
//          MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT).
// This file may not be copied, modified, or distributed except according to those terms.

use ffi::types::*;
use std::os::raw::{c_char, c_int, c_uchar, c_void};

#[allow(dead_code)]
#[link(name = "portmidi")]
extern "C" {
    pub fn Pm_Initialize() -> PmError;
    pub fn Pm_Terminate() -> PmError;
    fn Pm_HasHostError(stream: *const PortMidiStream) -> c_int;
    pub fn Pm_GetErrorText(errorCode: PmError) -> *const c_char;
    pub fn Pm_GetHostErrorText(msg: *mut c_char, len: c_int);
    pub fn Pm_CountDevices() -> c_int;
    pub fn Pm_GetDefaultInputDeviceID() -> PmDeviceId;
    pub fn Pm_GetDefaultOutputDeviceID() -> PmDeviceId;
    pub fn Pm_GetDeviceInfo(id: PmDeviceId) -> *const PmDeviceInfo;
    pub fn Pm_OpenInput(
        stream: *const *const PortMidiStream,
        inputDevice: PmDeviceId,
        inputDriverInfo: *const c_void,
        bufferSize: i32,
        time_proc: *const c_void,
        time_info: *const c_void,
    ) -> PmError;
    pub fn Pm_OpenOutput(
        stream: *const *const PortMidiStream,
        outputDevice: PmDeviceId,
        inputDriverInfo: *const c_void,
        bufferSize: i32,
        time_proc: *const c_void,
        time_info: *const c_void,
        latency: i32,
    ) -> PmError;
    pub fn Pm_CreateVirtualInput(
        name: *const c_char,
        interf: *const c_char,
        deviceInfo: *const c_void,
    ) -> PmError;
    pub fn Pm_CreateVirtualOutput(
        name: *const c_char,
        interf: *const c_char,
        deviceInfo: *const c_void,
    ) -> PmError;
    pub fn Pm_DeleteVirtualDevice(device: PmDeviceId) -> PmError;
    pub fn Pm_Read(stream: *const PortMidiStream, buffer: *mut PmEvent, length: c_int) -> c_int;
    fn Pm_Abort(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Close(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Poll(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_Write(
        stream: *const PortMidiStream,
        buffer: *const PmEvent,
        length: c_int,
    ) -> PmError;
    pub fn Pm_WriteShort(
        stream: *const PortMidiStream,
        timestamp: PmTimestamp,
        message: PmMessage,
    ) -> PmError;
    pub fn Pm_WriteSysEx(
        stream: *const PortMidiStream,
        when: PmTimestamp,
        msg: *const c_uchar,
    ) -> PmError;
}
