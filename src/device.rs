use ffi;
use types::*;
use std::ffi::CStr;
use std::str;

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
            let bytes = CStr::from_ptr((*device_info).name).to_bytes();
            str::from_utf8_unchecked(bytes).to_string()
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
