// Copyright 2014-2015 Sam Doshi (sam@metal-fish.co.uk)
// Copyright 2013-2014 Philippe Delrieu (philippe.delrieu@free.fr)
//
// Licensed under either of
//          Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
//          MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT).
// This file may not be copied, modified, or distributed except according to those terms.
use std::ptr;
use std::os::raw::c_char;

mod ffi;
use ffi::MaybeError;
pub mod types;
pub use types::*;

// Global fns
// ----------
/// `initialize` initalizes the underlying PortMidi C library, call this
/// before using the library.
///
/// Once initialized, PortMidi will no longer pickup any new Midi devices that are
/// connected, i.e. it does not support hot plugging.
pub fn initialize() -> PortMidiResult<()> {
    PortMidiResult::from(unsafe { ffi::Pm_Initialize() })
}

/// `terminate` terminates the underlying PortMidi C library, call this
/// after using the library.
pub fn terminate() -> PortMidiResult<()> {
    PortMidiResult::from(unsafe { ffi::Pm_Terminate() })
}

/// Return the number of devices. This number will not change during the lifetime
/// of the program.
pub fn count_devices() -> PortMidiDeviceId {
    unsafe { ffi::Pm_CountDevices() }
}

/// Gets the `PortMidiDeviceId` for the default input, or `None` if
/// there isn't one
///
/// See the PortMidi documentation for details of how to set the default device
pub fn get_default_input_device_id() -> Option<PortMidiDeviceId> {
    let id = unsafe { ffi::Pm_GetDefaultInputDeviceID() };
    if id == ffi::PM_NO_DEVICE {
        None
    } else {
        Some(id)
    }
}

/// Gets the `PortMidiDeviceId` for the default output, or `None` if
/// there isn't one
///
/// See the PortMidi documentation for details of how to set the default device
pub fn get_default_output_device_id() -> Option<PortMidiDeviceId> {
    let id = unsafe { ffi::Pm_GetDefaultOutputDeviceID() };
    if id == ffi::PM_NO_DEVICE {
        None
    } else {
        Some(id)
    }
}


// DeviceInfo
// ----------
/// Represents what we know about a device
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    /// The `PortMidiDeviceId` used with `OutputPort::new` and `InputPort::new`
    pub device_id: PortMidiDeviceId,
    /// The name of the device
    pub name: String,
    /// Is the device an input
    pub input: bool,
    /// Is the device an output
    pub output: bool,
}

impl DeviceInfo {
    fn wrap(device_id: PortMidiDeviceId, device_info: *const ffi::PmDeviceInfo) -> DeviceInfo {
        let name = unsafe {
            let bytes = std::ffi::CStr::from_ptr((*device_info).name).to_bytes();
            std::str::from_utf8_unchecked(bytes).to_string()
        };
        let input = unsafe { (*device_info).input };
        let output = unsafe { (*device_info).output };

        DeviceInfo {
            device_id: device_id,
            name: name,
            input: input > 0,
            output: output > 0,
        }
    }
}

/// Returns a `DeviceInfo` with information about a device, or `None` if
/// it does not exist
pub fn get_device_info(device_id: PortMidiDeviceId) -> Option<DeviceInfo> {
    let info = unsafe { ffi::Pm_GetDeviceInfo(device_id) };
    if info.is_null() {
        None
    } else {
        Some(DeviceInfo::wrap(device_id, info))
    }
}


// Midi events
// -----------
/// Represents a single midi message, see also `MidiEvent`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
}

impl MidiMessage {
    fn wrap(cmessage: ffi::PmMessage) -> MidiMessage {
        MidiMessage {
            status: ((cmessage) & 0xFF) as u8,
            data1: (((cmessage) >> 8) & 0xFF) as u8,
            data2: (((cmessage) >> 16) & 0xFF) as u8,
        }
    }

    fn unwrap(&self) -> ffi::PmMessage {
        ((((self.data2 as i32) << 16) & 0xFF0000) | (((self.data1 as i32) << 8) & 0xFF00) |
         ((self.status as i32) & 0xFF)) as i32
    }
}

/// Represents a time stamped midi event. See also `MidiMessage`
///
/// See the PortMidi documentation for how SysEx and midi realtime messages
/// are handled
///
/// TODO: what to do about the timestamp?
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiEvent {
    pub message: MidiMessage,
    pub timestamp: ffi::PmTimestamp,
}

impl MidiEvent {
    fn wrap(event: ffi::PmEvent) -> MidiEvent {
        MidiEvent {
            message: MidiMessage::wrap(event.message),
            timestamp: event.timestamp,
        }
    }

    fn unwrap(&self) -> ffi::PmEvent {
        ffi::PmEvent {
            message: self.message.unwrap(),
            timestamp: self.timestamp,
        }
    }
}


// Input
// -----
/// Representation of an input midi port
#[allow(missing_copy_implementations)]
pub struct InputPort {
    stream: *const ffi::PortMidiStream,
    input_device: ffi::PmDeviceId,
    buffer_size: i32,
}

const EVENT_BUFFER_SIZE: usize = 128;
impl InputPort {
    /// Construct a new `InputPort` for `input_device`
    pub fn new(input_device: PortMidiDeviceId, buffer_size: i32) -> InputPort {
        InputPort {
            stream: ptr::null(),
            input_device: input_device,
            buffer_size: buffer_size,
        }
    }

    /// Open the port returning an error if there is a problem
    pub fn open(&mut self) -> PortMidiResult<()> {
        PortMidiResult::from(unsafe {
            ffi::Pm_OpenInput(&self.stream,
                              self.input_device,
                              ptr::null(),
                              self.buffer_size,
                              ptr::null(),
                              ptr::null())
        })
    }

    pub fn read_n(&mut self, cnt: usize) -> PortMidiResult<Option<Vec<MidiEvent>>> {
        let read_cnt = if cnt > EVENT_BUFFER_SIZE {
            EVENT_BUFFER_SIZE as i32
        } else {
            cnt as i32
        };
        let mut event_buffer = [ffi::PmEvent::default(); EVENT_BUFFER_SIZE];
        let res = unsafe {
            ffi::Pm_Read(self.stream,
                         event_buffer.as_mut_ptr(),
                         read_cnt)
        };
        if res < 0 {
            let err = ffi::PmError::try_from(res).unwrap();
            // TODO: Return the error
            println!("error: {:?}", err);
            return Ok(None);
        } else if res == 0 {
            Ok(None)
        } else {
            // remove mutability and replace return value
            let mut events = (0..res as usize)
                             .map(|i| MidiEvent::wrap(event_buffer[i].clone()))
                             .collect::<Vec<MidiEvent>>();
            Ok(Some(events))
        }
    }

    /// Reads a single `MidiEvent` if one is avaible
    ///
    /// A `Result` of `None` means no event was available.
    ///
    /// See the PortMidi documentation for information on how it deals with input
    /// overflows
    /// TODO: call `read_n`
    pub fn read(&mut self) -> PortMidiResult<Option<MidiEvent>> {
        match self.read_n(1) {
            Ok(Some(mut vec)) => Ok(vec.pop()),
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// `poll` tests if there is input available, either returing a bool or an error
    pub fn poll(&self) -> PortMidiResult<bool> {
        let pm_error = unsafe { ffi::Pm_Poll(self.stream) };
        match pm_error {
            ffi::PmError::PmNoError => Ok(false),
            ffi::PmError::PmGotData => Ok(true),
            other => PortMidiResult::from(other).map(|_| false),
        }
    }

    /// Closes the input, flushing any pending buffers
    ///
    /// PortMidi attempts to close open streams when the application
    /// exits, but this can be difficult under Windows
    /// (according to the PortMidi documentation).
    pub fn close(&mut self) -> PortMidiResult<()> {
        PortMidiResult::from(unsafe { ffi::Pm_Close(self.stream) })
    }

    //    Test whether stream has a pending host error. Normally, the client finds
    //    out about errors through returned error codes, but some errors can occur
    //    asynchronously where the client does not
    //    explicitly call a function, and therefore cannot receive an error code.
    //    The client can test for a pending error using has_host_error(). If true,
    //    the error can be accessed and cleared by calling get_Error_text().
    //    Errors are also cleared by calling other functions that can return
    //    errors, e.g. open_input(), open_output(), read(), write(). The
    //    client does not need to call Pm_HasHostError(). Any pending error will be
    //    reported the next time the client performs an explicit function call on
    //    the stream, e.g. an input or output operation. Until the error is cleared,
    //    no new error codes will be obtained, even for a different stream.
    //
    fn has_host_error(&self) -> bool {
        unsafe { ffi::Pm_HasHostError(self.stream) > 0 }
    }
}


// Output
// ------
/// Representation of an output midi port
#[allow(missing_copy_implementations)]
pub struct OutputPort {
    stream: *const ffi::PortMidiStream,
    output_device: ffi::PmDeviceId,
    buffer_size: i32,
}
impl OutputPort {
    /// Construct a new `InputPort` for `input_device`
    pub fn new(output_device: PortMidiDeviceId, buffer_size: i32) -> OutputPort {
        OutputPort {
            stream: ptr::null(),
            output_device: output_device,
            buffer_size: buffer_size,
        }
    }

    /// Open the port returning an error if there is a problem
    pub fn open(&mut self) -> PortMidiResult<()> {
        PortMidiResult::from(unsafe {
            ffi::Pm_OpenOutput(&self.stream,
                               self.output_device,
                               ptr::null(),
                               self.buffer_size,
                               ptr::null(),
                               ptr::null(),
                               0)
        })
    }

    /// Terminates outgoing messages immediately
    ///
    /// The caller should immediately close the output port, this may
    /// result in transmission of a partial midi message. Note, not all platforms
    /// support abort.
    pub fn abort(&mut self) -> PortMidiResult<()> {
        PortMidiResult::from(unsafe { ffi::Pm_Abort(self.stream) })
    }

    /// Closes the midi stream, flushing any pending buffers
    ///
    /// PortMidi attempts to close open streams when the application
    /// exits, but this can be difficult under Windows
    /// (according to the PortMidi documentation).
    pub fn close(&mut self) -> PortMidiResult<()> {
        PortMidiResult::from(unsafe { ffi::Pm_Close(self.stream) })
    }

    /// Write a single `MidiEvent`
    pub fn write_event(&mut self, midi_event: MidiEvent) -> PortMidiResult<()> {
        let event = midi_event.unwrap();
        PortMidiResult::from(unsafe { ffi::Pm_Write(self.stream, &event, 1) })
    }

    /// Write a single `MidiMessage` immediately
    pub fn write_message(&mut self, midi_message: MidiMessage) -> PortMidiResult<()> {
        let message = midi_message.unwrap();
        PortMidiResult::from(unsafe { ffi::Pm_WriteShort(self.stream, 0, message) })
    }

    //    Test whether stream has a pending host error. Normally, the client finds
    //    out about errors through returned error codes, but some errors can occur
    //    asynchronously where the client does not
    //    explicitly call a function, and therefore cannot receive an error code.
    //    The client can test for a pending error using has_host_error(). If true,
    //    the error can be accessed and cleared by calling get_Error_text().
    //    Errors are also cleared by calling other functions that can return
    //    errors, e.g. open_input(), open_output(), read(), write(). The
    //    client does not need to call Pm_HasHostError(). Any pending error will be
    //    reported the next time the client performs an explicit function call on
    //    the stream, e.g. an input or output operation. Until the error is cleared,
    //    no new error codes will be obtained, even for a different stream.
    //
    pub fn has_host_error(&self) -> bool {
        unsafe { ffi::Pm_HasHostError(self.stream) > 0 }
    }
}


// Old code
// --------
/**  Translate portmidi error number into human readable message.
*    These strings are constants (set at compile time) so client has
*    no need to allocate storage
*/
pub fn get_error_text(error_code: ffi::PmError) -> String {
    unsafe {
        let error_text = ffi::Pm_GetErrorText(error_code);
        let bytes = std::ffi::CStr::from_ptr(error_text).to_bytes();
        std::str::from_utf8_unchecked(bytes).to_string()
    }
}

/**  Translate portmidi host error into human readable message.
     These strings are computed at run time, so client has to allocate storage.
     After this routine executes, the host error is cleared.
*/
pub fn get_host_error_text(msg: *const c_char, len: i32) {
    unsafe {
        ffi::Pm_GetHostErrorText(msg, len);
    }
}

pub const HDRLENGTH: i32 = 50;
pub const PM_HOST_ERROR_MSG_LEN: i32 = 256;
