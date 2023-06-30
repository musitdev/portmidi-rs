use device::DeviceInfo;
use ffi;
use ffi::MaybeError;
use io::{InputPort, OutputPort};
use std::os::raw::c_int;
use std::ptr;
use std::ffi::CString;
use std::sync::Mutex;
use types::{Error, PortMidiDeviceId, Result};

/// The PortMidi base struct.
/// Initializes PortMidi on creation and terminates it on drop.
pub struct PortMidi {
    device_count: u32,
    virtual_devs: Mutex<Vec<PortMidiDeviceId>>
}

impl PortMidi {
    /// Initializes the underlying PortMidi C library.
    /// PortMidi does not support *hot plugging*, this means
    /// that devices that are connect after calling `new`
    /// are not picked up.
    pub fn new() -> Result<Self> {
        Result::from(unsafe { ffi::Pm_Initialize() })?;
        let device_count = unsafe { ffi::Pm_CountDevices() };
        let virtual_devs = Mutex::new(vec![]);
        if device_count >= 0 {
            Ok(PortMidi {
                device_count: device_count as u32,
                virtual_devs,
            })
        } else {
            Err(Error::Invalid)
        }
    }

    /// Return the number of devices. This number will not change during the lifetime
    /// of the program.
    pub fn device_count(&self) -> PortMidiDeviceId {
        self.device_count as c_int
    }

    /// Return the number of virtual devices created in this instance.
    pub fn virtual_device_count(&self) -> PortMidiDeviceId {
	(*self.virtual_devs.lock().unwrap()).len() as c_int
    }

    /// Returns the `PortMidiDeviceId` for the default input device, or an `Error::NoDefaultDevice` if
    /// there is no available.
    pub fn default_input_device_id(&self) -> Result<PortMidiDeviceId> {
        match unsafe { ffi::Pm_GetDefaultInputDeviceID() } {
            ffi::PM_NO_DEVICE => Err(Error::NoDefaultDevice),
            id => Ok(id),
        }
    }

    /// Returns the `PortMidiDeviceId` for the default output device, or an `Error::NoDefaultDevice` if
    /// there is no available.
    pub fn default_output_device_id(&self) -> Result<PortMidiDeviceId> {
        match unsafe { ffi::Pm_GetDefaultOutputDeviceID() } {
            ffi::PM_NO_DEVICE => Err(Error::NoDefaultDevice),
            id => Ok(id),
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
        let mut devices = Vec::with_capacity(self.device_count() as usize);
        for res in (0..self.device_count()).map(|id| self.device(id)) {
            match res {
                Ok(device) => devices.push(device),
                Err(err) => return Err(err),
            }
        }
        Ok(devices)
    }

    /// Returns a `Vec<DeviceInfo>` containing all virtual device infos.
    /// An `Error::PortMidi(_)` is returned if the info for a virtual device can't be obtained.
    pub fn virtual_devices(&self) -> Result<Vec<DeviceInfo>> {
	let mut v_devs = Vec::with_capacity(self.virtual_device_count() as usize);
	let v_dev_vec: &Vec<PortMidiDeviceId> = &self.virtual_devs.lock().unwrap();
	for res in v_dev_vec.iter().map(|&id| self.device(id)) {
	    match res {
		Ok(device) => v_devs.push(device),
		Err(err) => return Err(err),
	    }
	}
	Ok(v_devs)
    }

    /// Creates an `InputPort` instance with the given buffer size for the default input device.
    pub fn default_input_port(&self, buffer_size: usize) -> Result<InputPort> {
        let info = self
            .default_input_device_id()
            .and_then(|id| self.device(id))?;
        InputPort::new(self, info, buffer_size)
    }

    /// Creates an `InputPort` instance for the given device and buffer size.
    /// If the given device is not an input device an `Error::NotAnInputDevice` is returned.
    pub fn input_port(&self, device: DeviceInfo, buffer_size: usize) -> Result<InputPort> {
        if device.is_input() {
            InputPort::new(self, device, buffer_size)
        } else {
            Err(Error::NotAnInputDevice)
        }
    }

    /// Creates an `OutputPort` instance with the given buffer size for the default output device.
    pub fn default_output_port(&self, buffer_size: usize) -> Result<OutputPort> {
        let info = self
            .default_output_device_id()
            .and_then(|id| self.device(id))?;
        OutputPort::new(self, info, buffer_size)
    }

    /// Creates an `OutputPort` instance for the given device and buffer size.
    /// If the given device is not an output device an `Error::NotAnOutputDevice` is returned.
    pub fn output_port(&self, device: DeviceInfo, buffer_size: usize) -> Result<OutputPort> {
        if device.is_output() {
            OutputPort::new(self, device, buffer_size)
        } else {
            Err(Error::NotAnOutputDevice)
        }
    }

    /// Creates a virtual input/output device depending on is_input argument.
    /// Returns the device info of the created device or throws an Error.
    fn create_virtual_device(&self, name: String, is_input: bool) -> Result<DeviceInfo> {
        let c_string = CString::new(name.clone()).unwrap();
        let id;
        if is_input {
            id = unsafe { ffi::Pm_CreateVirtualInput(c_string.as_ptr(), ptr::null(), ptr::null()) }
        } else {

            id = unsafe { ffi::Pm_CreateVirtualOutput(c_string.as_ptr(), ptr::null(), ptr::null()) };
        }

    	let id = match ffi::PmError::try_from(id as c_int) {
		Err(ffi::PmError::PmNoError) => None,
                Err(ffi::PmError::PmInvalidDeviceId) => panic!("Device name \"{}\" already exists or is invalid!", name),
		Err(err) => return Err(Error::PortMidi(err)),
		Ok(id) => Some(id),
        };

	let id: PortMidiDeviceId = id.unwrap();

	(*self.virtual_devs.lock().unwrap()).push(id);
	DeviceInfo::new(id)
    }

    /// Creates a virtual output device for the lifetime of the PortMidi instance.
    /// Returns the device info of the created device or throws an Error.
    pub fn create_virtual_input(&self, name: String) -> Result<DeviceInfo> {
        self.create_virtual_device(name, true)
    }

    /// Creates a virtual input device for the lifetime of the PortMidi instance.
    /// Returns the device info of the created device or throws an Error.
    pub fn create_virtual_output(&self, name: String) -> Result<DeviceInfo> {
        self.create_virtual_device(name, false)
    }

    /// Deletes a virtual device created by this instance.
    /// Returns Ok(()) or an Error, Error::Unknown when id is not an id of a virtual device.
    pub fn delete_virtual_device(&self, id: PortMidiDeviceId) -> Result<()> {
        let v_dev_vec = &mut (*self.virtual_devs.lock().unwrap());

        if v_dev_vec.contains(&id) {
            let index = v_dev_vec.iter().position(|pos| *pos == id).unwrap();
            v_dev_vec.remove(index);
            Result::from(unsafe { ffi::Pm_DeleteVirtualDevice(id) })
        } else {
            Err(Error::Unknown)
        }
    }

}
impl Drop for PortMidi {
    fn drop(&mut self) {

        while let Some(id) = (*self.virtual_devs.lock().unwrap()).pop() {
            Result::from(unsafe { ffi::Pm_DeleteVirtualDevice(id) })
                .map_err(|err| println!("Could not delete virtual device: {}", err))
                .unwrap();
        };

        Result::from(unsafe { ffi::Pm_Terminate() })
            .map_err(|err| println!("Could not terminate: {}", err))
            .unwrap();
    }
}
