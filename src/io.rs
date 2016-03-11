use ffi;
use std::os::raw::c_int;
use std::ptr;
use types::*;
use ffi::MaybeError;
use device::DeviceInfo;

/// Represents the input port of a PortMidi device.
pub struct InputPort {
    stream: *const ffi::PortMidiStream,
    device: ffi::PmDeviceId,
    buffer_size: usize,
}
impl InputPort {
    /// Construct a new `InputPort` for the given device and buffer size.
    ///
    /// If the `device` is not an input device an `Error::NotAnInputDevice` is returned.
    pub fn new(device: DeviceInfo, buffer_size: usize) -> Result<InputPort> {
        if device.is_output() {
            return Err(Error::NotAnInputDevice);
        }
        let raw_stream: *const ffi::PortMidiStream = ptr::null();
        try!(Result::from(unsafe {
            ffi::Pm_OpenInput(&raw_stream as *const *const _,
                              device.id(),
                              ptr::null(), // *inputDriverInfo, not needed for normal operation
                              buffer_size as c_int,
                              ptr::null(), // PmTimeProcPtr, a procedure that returns time in ms
                              ptr::null()) // time_info, a pointer passed to the time procedure
        }));

        Ok(InputPort {
            stream: raw_stream,
            device: device.id(),
            buffer_size: buffer_size,
        })
    }

    /// Returns a `Vec<MidiEvent>` with at most `cnt` elements.
    /// If there was no Midi event available, `None` is returned.
    /// If PortMidi fails to read from the device an `Error::PortMidi(_)` is returned.
    pub fn read_n(&self, cnt: usize) -> Result<Option<Vec<MidiEvent>>> {
        let read_cnt = if cnt > self.buffer_size {
            self.buffer_size as c_int
        } else {
            cnt as c_int
        };
        let mut event_buffer = vec![ffi::PmEvent::default(); self.buffer_size];
        let res = unsafe { ffi::Pm_Read(self.stream, event_buffer.as_mut_ptr(), read_cnt) };
        match ffi::PmError::try_from(res) {
            Ok(event_cnt) => {
                let events = (0..event_cnt as usize)
                                 .map(|i| MidiEvent::from(event_buffer[i].clone()))
                                 .collect::<Vec<MidiEvent>>();
                Ok(Some(events))
            }
            err @ Err(ffi::PmError::PmNoError) => Ok(None),
            Err(err) => return Err(Error::PortMidi(err)),
        }
    }

    /// Reads a single `MidiEvent` if one is avaible.
    ///
    /// A `Result` of `None` means no event was available.
    pub fn read(&mut self) -> Result<Option<MidiEvent>> {
        match self.read_n(1) {
            Ok(Some(mut vec)) => Ok(vec.pop()),
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Polls for available Midi events.
    /// Returns `true` if there are events available, otherwise `false` is returned.
    /// If the polling fails an `Error::PortMidi(_)` is returned.
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


/// Representation the output port of a PortMidi device.
pub struct OutputPort {
    stream: *const ffi::PortMidiStream,
    device: ffi::PmDeviceId,
    buffer_size: usize,
}
impl OutputPort {
    /// Construct a new `OutputPort` for the given device and buffer size.
    ///
    /// If the `device` is not an output device an `Error::NotAnOutputDevice` is returned.
    pub fn new(device: DeviceInfo, buffer_size: usize) -> Result<OutputPort> {
        if device.is_input() {
            return Err(Error::NotAnOutputDevice);
        }
        let raw_stream: *const ffi::PortMidiStream = ptr::null();
        try!(Result::from(unsafe {
            ffi::Pm_OpenOutput(&raw_stream as *const *const _,
                               device.id(),
                               ptr::null(), // *inputDriverInfo, not needed for normal operation
                               buffer_size as c_int,
                               ptr::null(), // PmTimeProcPtr, a procedure that returns time in ms,
                               ptr::null(), // time_info, a pointer passed to the time procedure
                               0) //latency
        }));

        Ok(OutputPort {
            stream: raw_stream,
            device: device.id(),
            buffer_size: buffer_size,
        })
    }

    /// Terminates outgoing messages immediately.
    ///
    /// The caller should immediately close the output port, this may
    /// result in transmission of a partial midi message. Note, not all platforms
    /// support abort.
    ///
    /// NOTE: This is unsafe, either mark the device as invalid or try to reopen it afterwards.
    fn abort(&mut self) -> Result<()> {
        Result::from(unsafe { ffi::Pm_Abort(self.stream) })
    }

    /// Write a single `MidiEvent`.
    /// Returns an `Error::PortMidi(_)` if something went wrong.
    pub fn write_event<T: Into<MidiEvent>>(&mut self, midi_event: T) -> Result<()> {
        self.write_events(vec![midi_event])
    }

    /// Write a buffer of midi events to the output port.
    /// Returns an `Error::PortMidi(_)` if something went wrong.
    pub fn write_events<T: Into<MidiEvent>>(&mut self, midi_events: Vec<T>) -> Result<()> {
        let events: Vec<ffi::PmEvent> = midi_events.into_iter()
                                                   .map(|event| event.into().into())
                                                   .collect();
        Result::from(unsafe { ffi::Pm_Write(self.stream, events.as_ptr(), events.len() as c_int) })
    }

    /// Write a single `MidiMessage`.
    /// Returns an `Error::PortMidi(_)` if something went wrong.
    pub fn write_message(&mut self, midi_message: MidiMessage) -> Result<()> {
        Result::from(unsafe { ffi::Pm_WriteShort(self.stream, 0, midi_message.into()) })
    }
}
impl Drop for OutputPort {
    fn drop(&mut self) {
        if let Err(err) = Result::from(unsafe { ffi::Pm_Close(self.stream) }) {
            println!("{}", err);
        }
    }
}
