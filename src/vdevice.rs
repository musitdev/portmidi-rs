use device::{DeviceInfo, Direction};
use ffi;
use ffi::MaybeError;
use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;
use types::*;

#[derive(Clone, Debug)]
pub struct VirtualDevice {
    info: DeviceInfo,
}

impl VirtualDevice {
    /// Creates a virtual input/output device depending on direction argument.
    /// Returns the device info of the created device or an Error.
    pub fn new(name: &str, direction: Direction) -> Result<Self> {
        let c_string = CString::new(name).unwrap();

        let id = match direction {
            Direction::Input => unsafe {
                ffi::Pm_CreateVirtualInput(c_string.as_ptr(), ptr::null(), ptr::null())
            },
            Direction::Output => unsafe {
                ffi::Pm_CreateVirtualOutput(c_string.as_ptr(), ptr::null(), ptr::null())
            },
        };

        let id = match ffi::PmError::try_from(id as c_int) {
            Err(ffi::PmError::PmNoError) => None,
            Err(ffi::PmError::PmInvalidDeviceId) => {
                panic!("Device name \"{}\" already exists or is invalid!", name)
            }
            Err(err) => return Err(Error::PortMidi(err)),
            Ok(id) => Some(id),
        };

        let id: PortMidiDeviceId = id.unwrap();

        let info = DeviceInfo::new(id)?;

        Ok(VirtualDevice { info })
    }

    pub fn id(&self) -> PortMidiDeviceId {
        self.info.id()
    }

    pub fn name(&self) -> &str {
        self.info.name()
    }

    pub fn is_input(&self) -> bool {
        self.info.is_input()
    }

    pub fn is_output(&self) -> bool {
        self.info.is_output()
    }
}

impl Drop for VirtualDevice {
    fn drop(&mut self) {
        Result::from(unsafe { ffi::Pm_DeleteVirtualDevice(self.id()) })
            .map_err(|err| println!("Error deleting virtual device: {}", err))
            .unwrap();
    }
}
