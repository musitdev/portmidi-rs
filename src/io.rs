use ffi;
use std::os::raw::c_int;
use std::ptr;
use types::*;
use ffi::MaybeError;
use device::DeviceInfo;

// Input
// -----
/// Representation of an input midi port
#[allow(missing_copy_implementations)]
pub struct InputPort {
    stream: *const ffi::PortMidiStream,
    device: ffi::PmDeviceId,
    buffer_size: i32, // TODO: replace with an usize
}

const EVENT_BUFFER_SIZE: usize = 128; // replace this with a parameter to InputPort new
impl InputPort {
    /// Construct a new `InputPort` for `input_device`
    pub fn new(device: DeviceInfo, buffer_size: i32) -> Result<InputPort> {
        if device.is_output() {
            return Err(Error::NotAnInputDevice);
        }
        let raw_stream: *const ffi::PortMidiStream = ptr::null();
        try!(Result::from(unsafe {
            ffi::Pm_OpenInput(&raw_stream as *const *const _,
                              device.id(),
                              ptr::null(), // *inputDriverInfo, not needed for normal operation
                              EVENT_BUFFER_SIZE as c_int,
                              ptr::null(), // PmTimeProcPtr, a procedure that returns time in ms,
                              ptr::null()) // time_info, a pointer passed to the time procedure
        }));

        Ok(InputPort {
            stream: raw_stream,
            device: device.id(),
            buffer_size: EVENT_BUFFER_SIZE as c_int,
        })
    }

    pub fn read_n(&mut self, cnt: usize) -> Result<Option<Vec<MidiEvent>>> {
        let read_cnt = if cnt > EVENT_BUFFER_SIZE {
            EVENT_BUFFER_SIZE as i32
        } else {
            cnt as i32
        };
        let mut event_buffer = [ffi::PmEvent::default(); EVENT_BUFFER_SIZE];
        let res = unsafe { ffi::Pm_Read(self.stream, event_buffer.as_mut_ptr(), read_cnt) };
        if res < 0 {
            let err = ffi::PmError::try_from(res).unwrap();
            // TODO: Return the error
            println!("error: {:?}", err);
            return Ok(None);
        } else if res == 0 {
            Ok(None)
        } else {
            // remove mutability and replace return value
            let events = (0..res as usize)
                             .map(|i| MidiEvent::from(event_buffer[i].clone()))
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
    pub fn read(&mut self) -> Result<Option<MidiEvent>> {
        match self.read_n(1) {
            Ok(Some(mut vec)) => Ok(vec.pop()),
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// `poll` tests if there is input available, either returing a bool or an error
    pub fn poll(&self) -> Result<bool> {
        let pm_error = unsafe { ffi::Pm_Poll(self.stream) };
        match pm_error {
            ffi::PmError::PmNoError => Ok(false),
            ffi::PmError::PmGotData => Ok(true),
            err @ _ => Err(Error::PortMidi(err)),
        }
    }
}
impl Drop for InputPort {
    fn drop(&mut self) {
        if let Err(err) = Result::from(unsafe { ffi::Pm_Close(self.stream) }) {
            println!("{}", err);
        }
    }
}


// Output
// ------
/// Representation of an output midi port
///
#[allow(missing_copy_implementations)]
pub struct OutputPort {
    stream: *const ffi::PortMidiStream,
    device: ffi::PmDeviceId,
    buffer_size: i32, // TODO: replace with an usize
}
impl OutputPort {
    /// Construct a new `OutputPort` for `input_device`
    pub fn new(device: DeviceInfo, buffer_size: i32) -> Result<OutputPort> {
        if device.is_input() {
            return Err(Error::NotAnOutputDevice);
        }
        let raw_stream: *const ffi::PortMidiStream = ptr::null();
        try!(Result::from(unsafe {
            ffi::Pm_OpenOutput(&raw_stream as *const *const _,
                               device.id(),
                               ptr::null(), // *inputDriverInfo, not needed for normal operation
                               EVENT_BUFFER_SIZE as c_int,
                               ptr::null(), // PmTimeProcPtr, a procedure that returns time in ms,
                               ptr::null(), // time_info, a pointer passed to the time procedure
                               0) //latency
        }));

        Ok(OutputPort {
            stream: raw_stream,
            device: device.id(),
            buffer_size: EVENT_BUFFER_SIZE as c_int,
        })
    }

    /// Terminates outgoing messages immediately
    ///
    /// The caller should immediately close the output port, this may
    /// result in transmission of a partial midi message. Note, not all platforms
    /// support abort.
    pub fn abort(&mut self) -> Result<()> {
        Result::from(unsafe { ffi::Pm_Abort(self.stream) })
    }

    /// Write a single `MidiEvent`
    pub fn write_event(&mut self, midi_event: MidiEvent) -> Result<()> {
        let event = midi_event.into();
        Result::from(unsafe { ffi::Pm_Write(self.stream, &event, 1) })
    }

    /// Write a single `MidiMessage` immediately
    pub fn write_message(&mut self, midi_message: MidiMessage) -> Result<()> {
        let message = midi_message.into();
        Result::from(unsafe { ffi::Pm_WriteShort(self.stream, 0, message) })
    }
}
impl Drop for OutputPort {
    fn drop(&mut self) {
        if let Err(err) = Result::from(unsafe { ffi::Pm_Close(self.stream) }) {
            println!("{}", err);
        }
    }
}
