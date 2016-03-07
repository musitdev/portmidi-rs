use ffi;
use types::{Result, PortMidiDeviceId, Error};
use io::{InputPort, OutputPort};
use device::DeviceInfo;

/// The elements are static after initializing
pub struct PortMidi {
    device_cnt: i32,
    buffer_size: i32,
}
impl PortMidi {
    /// Initializes the underlying PortMidi C library.
    /// It does not support *hot plugging*, this means
    /// that devices that are connect after calling `new`
    /// are not picked up by PortMidi.
    pub fn new() -> Result<Self> {
        try!(Result::from(unsafe { ffi::Pm_Initialize() }));

        Ok(PortMidi {
            device_cnt: unsafe { ffi::Pm_CountDevices() },
            buffer_size: 1024, // TODO: argument
        })
    }

    /// Return the number of devices. This number will not change during the lifetime
    /// of the program.
    pub fn device_cnt(self) -> PortMidiDeviceId {
        self.device_cnt
    }

    /// Gets the `PortMidiDeviceId` for the default input, or `None` if
    /// there isn't one
    ///
    /// See the PortMidi documentation for details of how to set the default device
    pub fn default_input_device_id(&self) -> Result<PortMidiDeviceId> {
        match unsafe { ffi::Pm_GetDefaultInputDeviceID() } {
            ffi::PM_NO_DEVICE => Err(Error::NoDefaultDevice),
            id @ _ => Ok(id),
        }
    }

    /// Gets the `PortMidiDeviceId` for the default output, or `None` if
    /// there isn't one
    ///
    /// See the PortMidi documentation for details of how to set the default device
    pub fn default_output_device_id(&self) -> Result<PortMidiDeviceId> {
        match unsafe { ffi::Pm_GetDefaultOutputDeviceID() } {
            ffi::PM_NO_DEVICE => Err(Error::NoDefaultDevice),
            id @ _ => Ok(id),
        }
    }

    pub fn device_info(&self, id: PortMidiDeviceId) -> Result<DeviceInfo> {
        DeviceInfo::new(id)
    }

    pub fn default_input_port(&self) -> Result<InputPort> {
        let info = try!(self.default_input_device_id().and_then(|id| self.device_info(id)));
        InputPort::new(info, self.buffer_size)
    }

    pub fn input_port(&self, device: DeviceInfo) -> Result<InputPort> {
        if device.is_input() {
            InputPort::new(device, self.buffer_size)
        } else {
            Err(Error::Invalid)
        }
    }

    pub fn default_output_port(&self) -> Result<OutputPort> {
        let info = try!(self.default_output_device_id().and_then(|id| self.device_info(id)));
        OutputPort::new(info, self.buffer_size)
    }

    pub fn output_port(&self, device: DeviceInfo) -> Result<InputPort> {
        if device.is_output() {
            InputPort::new(device, self.buffer_size)
        } else {
            Err(Error::Invalid)
        }
    }
}
impl Drop for PortMidi {
    fn drop(&mut self) {
        Result::from(unsafe { ffi::Pm_Terminate() })
            .map_err(|err| println!("Could not terminate library: {}", err));
    }
}