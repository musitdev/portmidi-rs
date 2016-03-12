use ffi;
use std::os::raw::c_int;
use types::{Result, PortMidiDeviceId, Error};
use io::{InputPort, OutputPort};
use device::DeviceInfo;

/// The PortMidi base struct.
/// Initializes PortMidi on creation and terminates it on drop.
pub struct PortMidi {
    device_cnt: u32,
}
impl PortMidi {
    /// Initializes the underlying PortMidi C library.
    /// PortMidi does not support *hot plugging*, this means
    /// that devices that are connect after calling `new`
    /// are not picked up.
    pub fn new() -> Result<Self> {
        try!(Result::from(unsafe { ffi::Pm_Initialize() }));
        let device_cnt = unsafe { ffi::Pm_CountDevices() };
        if device_cnt >= 0 {
            Ok(PortMidi { device_cnt: device_cnt as u32 })
        } else {
            Err(Error::Invalid)
        }
    }

    /// Return the number of devices. This number will not change during the lifetime
    /// of the program.
    pub fn device_cnt(&self) -> PortMidiDeviceId {
        self.device_cnt as c_int
    }

    /// Returns the `PortMidiDeviceId` for the default input device, or an `Error::NoDefaultDevice` if
    /// there is no available.
    pub fn default_input_device_id(&self) -> Result<PortMidiDeviceId> {
        match unsafe { ffi::Pm_GetDefaultInputDeviceID() } {
            ffi::PM_NO_DEVICE => Err(Error::NoDefaultDevice),
            id @ _ => Ok(id),
        }
    }

    /// Returns the `PortMidiDeviceId` for the default output device, or an `Error::NoDefaultDevice` if
    /// there is no available.
    pub fn default_output_device_id(&self) -> Result<PortMidiDeviceId> {
        match unsafe { ffi::Pm_GetDefaultOutputDeviceID() } {
            ffi::PM_NO_DEVICE => Err(Error::NoDefaultDevice),
            id @ _ => Ok(id),
        }
    }

    /// Returns the `DeviceInfo` for the given device id or an `Error::PortMidi(_)` if
    /// the given id is invalid.
    pub fn device(&self, id: PortMidiDeviceId) -> Result<DeviceInfo> {
        DeviceInfo::new(id)
    }

    /// Returns a `Vec<DeviceInfo>` containing all known device infos.
    /// An `Error::PortMidi(_)` is returned if the info for a device can't be obtained.
    pub fn devices(&self) -> Result<Vec<DeviceInfo>> {
        let mut devices = Vec::with_capacity(self.device_cnt() as usize);
        for res in (0..self.device_cnt()).map(|id| self.device(id)) {
            match res {
                Ok(device) => devices.push(device),
                Err(err) => return Err(err),
            }
        }
        Ok(devices)
    }

    /// Creates an `InputPort` instance with the given buffer size for the default input device.
    pub fn default_input_port(&self, buffer_size: usize) -> Result<InputPort> {
        let info = try!(self.default_input_device_id().and_then(|id| self.device(id)));
        InputPort::new(info, buffer_size)
    }

    /// Creates an `InputPort` instance for the given device and buffer size.
    /// If the given device is not an input device an `Error::NotAnInputDevice` is returned.
    pub fn input_port(&self, device: DeviceInfo, buffer_size: usize) -> Result<InputPort> {
        if device.is_input() {
            InputPort::new(device, buffer_size)
        } else {
            Err(Error::NotAnInputDevice)
        }
    }

    /// Creates an `OutputPort` instance with the given buffer size for the default output device.
    pub fn default_output_port(&self, buffer_size: usize) -> Result<OutputPort> {
        let info = try!(self.default_output_device_id().and_then(|id| self.device(id)));
        OutputPort::new(info, buffer_size)
    }

    /// Creates an `OutputPort` instance for the given device and buffer size.
    /// If the given device is not an output device an `Error::NotAnOutputDevice` is returned.
    pub fn output_port(&self, device: DeviceInfo, buffer_size: usize) -> Result<OutputPort> {
        if device.is_output() {
            OutputPort::new(device, buffer_size)
        } else {
            Err(Error::NotAnOutputDevice)
        }
    }
}
impl Drop for PortMidi {
    fn drop(&mut self) {
        Result::from(unsafe { ffi::Pm_Terminate() })
            .map_err(|err| println!("Could not terminate: {}", err)).unwrap();
    }
}
