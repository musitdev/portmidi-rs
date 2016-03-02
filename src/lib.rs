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
mod device;
pub use device::*;
mod io;
pub use io::*;
pub use ffi::PmError;
pub mod types;
pub use types::*;

// Global fns
// ----------
/// `initialize` initalizes the underlying PortMidi C library, call this
/// before using the library.
///
/// Once initialized, PortMidi will no longer pickup any new Midi devices that are
/// connected, i.e. it does not support hot plugging.
pub fn initialize() -> Result<()> {
    Result::from(unsafe { ffi::Pm_Initialize() })
}

/// `terminate` terminates the underlying PortMidi C library, call this
/// after using the library.
pub fn terminate() -> Result<()> {
    Result::from(unsafe { ffi::Pm_Terminate() })
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





pub const HDRLENGTH: i32 = 50;
pub const PM_HOST_ERROR_MSG_LEN: i32 = 256;
