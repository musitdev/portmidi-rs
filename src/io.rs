use ffi;
use std::ptr;
use types::*;
use ffi::MaybeError;

// Input
// -----
/// Representation of an input midi port
#[allow(missing_copy_implementations)]
pub struct InputPort {
    stream: *const ffi::PortMidiStream,
    input_device: ffi::PmDeviceId,
    buffer_size: i32,
}

const EVENT_BUFFER_SIZE: usize = 128;
impl InputPort {
    /// Construct a new `InputPort` for `input_device`
    pub fn new(input_device: PortMidiDeviceId, buffer_size: i32) -> InputPort {
        InputPort {
            stream: ptr::null(),
            input_device: input_device,
            buffer_size: buffer_size,
        }
    }

    /// Open the port returning an error if there is a problem
    pub fn open(&mut self) -> Result<()> {
        Result::from(unsafe {
            ffi::Pm_OpenInput(&self.stream,
                              self.input_device,
                              ptr::null(),
                              self.buffer_size,
                              ptr::null(),
                              ptr::null())
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
            let mut events = (0..res as usize)
                                 .map(|i| MidiEvent::wrap(event_buffer[i].clone()))
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

    /// Closes the input, flushing any pending buffers
    ///
    /// PortMidi attempts to close open streams when the application
    /// exits, but this can be difficult under Windows
    /// (according to the PortMidi documentation).
    pub fn close(&mut self) -> Result<()> {
        Result::from(unsafe { ffi::Pm_Close(self.stream) })
    }
}


// Output
// ------
/// Representation of an output midi port
#[allow(missing_copy_implementations)]
pub struct OutputPort {
    stream: *const ffi::PortMidiStream,
    output_device: ffi::PmDeviceId,
    buffer_size: i32,
}
impl OutputPort {
    /// Construct a new `InputPort` for `input_device`
    pub fn new(output_device: PortMidiDeviceId, buffer_size: i32) -> OutputPort {
        OutputPort {
            stream: ptr::null(),
            output_device: output_device,
            buffer_size: buffer_size,
        }
    }

    /// Open the port returning an error if there is a problem
    pub fn open(&mut self) -> Result<()> {
        Result::from(unsafe {
            ffi::Pm_OpenOutput(&self.stream,
                               self.output_device,
                               ptr::null(),
                               self.buffer_size,
                               ptr::null(),
                               ptr::null(),
                               0)
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

    /// Closes the midi stream, flushing any pending buffers
    ///
    /// PortMidi attempts to close open streams when the application
    /// exits, but this can be difficult under Windows
    /// (according to the PortMidi documentation).
    pub fn close(&mut self) -> Result<()> {
        Result::from(unsafe { ffi::Pm_Close(self.stream) })
    }

    /// Write a single `MidiEvent`
    pub fn write_event(&mut self, midi_event: MidiEvent) -> Result<()> {
        let event = midi_event.unwrap();
        Result::from(unsafe { ffi::Pm_Write(self.stream, &event, 1) })
    }

    /// Write a single `MidiMessage` immediately
    pub fn write_message(&mut self, midi_message: MidiMessage) -> Result<()> {
        let message = midi_message.unwrap();
        Result::from(unsafe { ffi::Pm_WriteShort(self.stream, 0, message) })
    }
}

