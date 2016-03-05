use ffi;
use types::*;

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Direction {
    Input,
    Output,
}

/// Represents what we know about a device
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    /// The `PortMidiDeviceId` used with `OutputPort::new` and `InputPort::new`
    id: PortMidiDeviceId,
    /// The name of the device
    name: String,
    dir: Direction,
}
impl DeviceInfo {
    // TODO: return a Result with an error if `dev_inf_ptr` is NULL (invalid id)
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
                id: id,
                name: name,
                dir: direction,
            })
        }
    }

    pub fn is_input(&self) -> bool {
        match self.dir {
            Direction::Input => true,
            _ => false,
        }
    }

    pub fn is_output(&self) -> bool {
        !self.is_input()
    }

    pub fn name(self) -> String {
        self.name
    }

    pub fn id(&self) -> PortMidiDeviceId {
        self.id
    }
}
