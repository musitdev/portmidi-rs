use ffi;
use std::fmt;
use types::*;

/// Device event direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Input,
    Output,
}

/// Represents a PortMidi device.
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    id: PortMidiDeviceId,
    /// The device name
    name: String,
    /// Event direction
    dir: Direction,
}
impl DeviceInfo {
    /// Creates a new `DeviceInfo` instance for the given device id.
    /// Returns an `Error::PortMidi(_)` if the given id is invalid.
    pub fn new(id: PortMidiDeviceId) -> Result<Self> {
        let dev_inf_ptr = unsafe { ffi::Pm_GetDeviceInfo(id) };
        if dev_inf_ptr.is_null() {
            Err(Error::PortMidi(ffi::PmError::PmInvalidDeviceId))
        } else {
            let name = unsafe { ffi::ptr_to_string((*dev_inf_ptr).name).unwrap() };
            let direction = if unsafe { (*dev_inf_ptr).input != 0 } {
                Direction::Input
            } else {
                Direction::Output
            };

            Ok(DeviceInfo {
                id,
                name,
                dir: direction,
            })
        }
    }

    /// Returns `true` for an input device.
    pub fn is_input(&self) -> bool {
        match self.dir {
            Direction::Input => true,
            _ => false,
        }
    }

    /// Returns `true` for an output device.
    pub fn is_output(&self) -> bool {
        !self.is_input()
    }

    /// Returns the device name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the device event direction.
    pub fn direction(&self) -> Direction {
        self.dir
    }

    /// Returns the device id.
    pub fn id(&self) -> PortMidiDeviceId {
        self.id
    }
}
impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}) {:?}: {}", self.id(), self.direction(), self.name())
    }
}
