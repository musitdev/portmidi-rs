use ffi;
use types::*;
use std::ffi::CStr;
use std::str;

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
    // TODO: return a Result with an error if `dev_inf_ptr` is NULL (invalid id)
    pub fn new(id: PortMidiDeviceId) -> Option<Self> {
        let dev_inf_ptr = unsafe { ffi::Pm_GetDeviceInfo(id) };
        if dev_inf_ptr.is_null() {
            None
        } else {
            // TODO: use ptr_to_string
            let name = unsafe { ffi::ptr_to_string((*dev_inf_ptr).name).unwrap() };
            // TODO: Replace this by an enum and create convenience function, `is_{in,out}put`
            let input = unsafe { (*dev_inf_ptr).input };
            let output = unsafe { (*dev_inf_ptr).output };

            Some(DeviceInfo {
                device_id: id,
                name: name,
                input: input > 0,
                output: output > 0,
            })
        }
    }
}
