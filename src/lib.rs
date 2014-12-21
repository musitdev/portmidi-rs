extern crate libc;
extern crate core;
extern crate serialize;

use std::ptr;
use libc::c_char;

mod ffi;


// Types
// -----
pub type PortMidiDeviceId = i32;
pub type PortMidiResult<T> = Result<T, PortMidiError>;


// Errors
// ------
#[deriving(Copy, Show, PartialEq, Eq)]
pub enum PortMidiError {
    HostError,
    InvalidDeviceId,
    InsufficientMemory,
    BufferTooSmall,
    BufferOverflow,
    BadPtr,
    BadData,
    InternalError,
    BufferMaxSize
}

fn from_pm_error(pm_error: ffi::PmError) -> PortMidiResult<()> {
    match pm_error {
        ffi::PmError::PmNoError => Ok(()),
        ffi::PmError::PmGotData => Ok(()),
        ffi::PmError::PmHostError => Err(PortMidiError::HostError),
        ffi::PmError::PmInvalidDeviceId => Err(PortMidiError::InvalidDeviceId),
        ffi::PmError::PmInsufficientMemory => Err(PortMidiError::InsufficientMemory),
        ffi::PmError::PmBufferTooSmall => Err(PortMidiError::BufferTooSmall),
        ffi::PmError::PmBufferOverflow => Err(PortMidiError::BufferOverflow),
        ffi::PmError::PmBadPtr => Err(PortMidiError::BadPtr),
        ffi::PmError::PmBadData => Err(PortMidiError::BadData),
        ffi::PmError::PmInternalError => Err(PortMidiError::InternalError),
        ffi::PmError::PmBufferMaxSize => Err(PortMidiError::BufferMaxSize),
    }
}


// Global fns
// ----------
/// `initialize` initalizes the underlying PortMidi C library, call this
/// before using the library.
pub fn initialize() -> PortMidiResult<()> {
    from_pm_error(unsafe {
        ffi::Pm_Initialize()
    })
}

/// `terminate` terminates the underlying PortMidi C library, call this
/// after using the library.
pub fn terminate() -> PortMidiResult<()> {
    from_pm_error(unsafe {
        ffi::Pm_Terminate()
    })
}

/// Return the number of devices
pub fn count_devices() -> i32 {
    unsafe {
        ffi::Pm_CountDevices()
    }
}

/// Gets the `PortMidiDeviceId` for the default input, or `None` if
/// there isn't one
///
/// See the PortMidi documentation for details of how to set the default device
pub fn get_default_input_device_id() -> Option<PortMidiDeviceId> {
    let id = unsafe { ffi::Pm_GetDefaultInputDeviceID() };
    if id == ffi::PM_NO_DEVICE {
        None
    }
    else {
        Some(id)
    }
}

/// Gets the `PortMidiDeviceId` for the default output, or `None` if
/// there isn't one
///
/// See the PortMidi documentation for details of how to set the default device
pub fn get_default_output_device_id() -> Option<PortMidiDeviceId> {
    let id = unsafe { ffi::Pm_GetDefaultOutputDeviceID() };
    if id == ffi::PM_NO_DEVICE {
        None
    }
    else {
        Some(id)
    }
}


// DeviceInfo
// ----------
/// Represents what we know about a device
#[deriving(Clone, Show)]
pub struct DeviceInfo {
    pub name: String,
    pub input: bool,
    pub output: bool
}

impl DeviceInfo {
    fn wrap(device_info: *const ffi::CPmDeviceInfo) -> DeviceInfo {
        unsafe {
            DeviceInfo {
                name: String::from_raw_buf((*device_info).name as *const u8),
                input: (*device_info).input > 0,
                output: (*device_info).output > 0
            }
        }
    }
}

/// Returns a `DeviceInfo` with information about a device, or `None` if
/// it does not exist
pub fn get_device_info(device : PortMidiDeviceId) -> Option<DeviceInfo> {
    let c_info = unsafe { ffi::Pm_GetDeviceInfo(device) };
    if c_info.is_null() {
        None
    }
    else {
        Some(DeviceInfo::wrap(c_info))
    }
}


// Midi events
// -----------
/// Represents a single midi message, see also `MidiEvent`
///
/// TODO: should we use u8?
#[deriving(Clone, Copy, PartialEq, Eq, Decodable, Encodable, Show)]
pub struct MidiMessage {
    pub status : i8,
    pub data1 : i8,
    pub data2 : i8,
}

impl MidiMessage {
    fn wrap(cmessage : ffi::CPmMessage) -> MidiMessage {
        MidiMessage {
            status:  ((cmessage) & 0xFF) as i8,
            data1 : (((cmessage) >> 8) & 0xFF) as i8,
            data2 : (((cmessage) >> 16) & 0xFF) as i8,
        }
    }

    fn unwrap(&self) -> ffi::CPmMessage {
        ((((self.data2 as i32) << 16) & 0xFF0000) |
          (((self.data1 as i32) << 8) & 0xFF00) |
          ((self.status as i32) & 0xFF)) as i32
    }
}

/// Represents a time stamped midi event. See also `MidiMessage`
///
/// See the PortMidi documentation for how SysEx and midi realtime messages
/// are handled
///
/// TODO: what to do about the timestamp?
#[deriving(Clone, Copy, PartialEq, Eq, Decodable, Encodable, Show)]
pub  struct MidiEvent {
    pub message : MidiMessage,
    pub timestamp : ffi::CPmTimestamp,
}

impl MidiEvent {
    fn wrap(cevent : ffi::CPmEvent) -> MidiEvent {
        MidiEvent {
            message:  MidiMessage::wrap(cevent.message),
            timestamp : cevent.timestamp,
        }
    }

    fn unwrap(&self) -> ffi::CPmEvent {
        ffi::CPmEvent {
            message:  self.message.unwrap(),
            timestamp : self.timestamp,
        }
    }
}


// Input
// -----
/// Representation of an input midi port.
#[allow(missing_copy_implementations)]
pub struct InputPort {
    c_pm_stream : *const ffi::CPortMidiStream,
    input_device : ffi::CPmDeviceID,
    buffer_size : i32,
}

impl InputPort {
    /// Construct a new `InputPort` for `input_device`
    pub fn new(input_device : PortMidiDeviceId, buffer_size: i32) -> InputPort {
        InputPort {
            c_pm_stream : ptr::null(),
            input_device : input_device,
            buffer_size : buffer_size,
        }
    }

    /// Open the port returning an error if there is a problem
    pub fn open(&mut self)  -> PortMidiResult<()> {
        from_pm_error(unsafe {
            ffi::Pm_OpenInput(&self.c_pm_stream, self.input_device, ptr::null(),
                              self.buffer_size, ptr::null(), ptr::null())
        })
    }

    /// Reads a single `MidiEvent` if one is avaible
    ///
    /// A `Result` of `None` means no event was available.
    ///
    /// See the PortMidi documentation for information on how it deals with input
    /// overflows
    pub fn read(&mut self) -> PortMidiResult<Option<MidiEvent>> {
        use std::num::FromPrimitive;
        //get one note a the time
        let mut event = ffi::CPmEvent { message : 0, timestamp : 0 };
        let no_of_notes = unsafe { ffi::Pm_Read(self.c_pm_stream, &mut event, 1) };
        match no_of_notes {
            y if y == 0 => Ok(None),
            y if y > 0 => Ok(Some(MidiEvent::wrap(event))),
            _ => {
                // if it's negative it's an error, convert it
                let maybe_pm_error: Option<ffi::PmError> = FromPrimitive::from_i32(no_of_notes);
                if let Some(pm_error) = maybe_pm_error {
                    from_pm_error(pm_error).map(|_| None)
                }
                else {
                    // what should we do, if we can't convert the error no?
                    // should we panic?
                    Ok(None)
                }
            }
        }
    }

    /// `poll` tests if there is input available, either returing a bool or an error
    pub fn poll(&self) -> PortMidiResult<bool> {
        let pm_error = unsafe { ffi::Pm_Poll(self.c_pm_stream) };
        match pm_error {
            ffi::PmError::PmNoError => Ok(false),
            ffi::PmError::PmGotData => Ok(true),
            other => from_pm_error(other).map(|_| false)
        }
    }

    /// Closes the input, flushing any pending buffers
    ///
    /// PortMidi attempts to close open streams when the application exists,
    /// but this can be difficult under Windows
    /// (according to the PortMidi documentation).
    pub fn close(&mut self) -> PortMidiResult<()> {
        from_pm_error(unsafe {
            ffi::Pm_Close(self.c_pm_stream)
        })
    }

    /*
    *    Test whether stream has a pending host error. Normally, the client finds
    *    out about errors through returned error codes, but some errors can occur
    *    asynchronously where the client does not
    *    explicitly call a function, and therefore cannot receive an error code.
    *    The client can test for a pending error using has_host_error(). If true,
    *    the error can be accessed and cleared by calling get_Error_text().
    *    Errors are also cleared by calling other functions that can return
    *    errors, e.g. open_input(), open_output(), read(), write(). The
    *    client does not need to call Pm_HasHostError(). Any pending error will be
    *    reported the next time the client performs an explicit function call on
    *    the stream, e.g. an input or output operation. Until the error is cleared,
    *    no new error codes will be obtained, even for a different stream.
    */
    pub fn has_host_error(&self) -> i32  {
        unsafe {
            ffi::Pm_HasHostError(self.c_pm_stream)
        }
    }
}



// Old code
// --------
#[deriving(Copy, Show, PartialEq, Eq, FromPrimitive)]
pub enum PmError {
    PmNoError = ffi::PmError::PmNoError as int,
    PmGotData = ffi::PmError::PmGotData as int, /* < A "no error" return that also indicates data available */
    PmHostError = ffi::PmError::PmHostError as int,
    PmInvalidDeviceId = ffi::PmError::PmInvalidDeviceId as int, /** out of range or
                        * output device when input is requested or
                        * input device when output is requested or
                        * device is already opened
                        */
    PmInsufficientMemory = ffi::PmError::PmInsufficientMemory as int,
    PmBufferTooSmall = ffi::PmError::PmBufferTooSmall as int,
    PmBufferOverflow = ffi::PmError::PmBufferOverflow as int,
    PmBadPtr = ffi::PmError::PmBadPtr as int, /* PortMidiStream parameter is NULL or
               * stream is not opened or
               * stream is output when input is required or
               * stream is input when output is required */
    PmBadData = ffi::PmError::PmBadData as int, /* illegal midi data, e.g. missing EOX */
    PmInternalError = ffi::PmError::PmInternalError as int,
    PmBufferMaxSize = ffi::PmError::PmBufferMaxSize as int, /* buffer is already as large as it can be */
    /* NOTE: If you add a new error type, be sure to update Pm_GetErrorText() */
}

impl PmError{
  fn unwrap(error: ffi::PmError) -> PmError  {
    FromPrimitive::from_i64(error as i64).unwrap()
  }

  fn wrap(&self) -> ffi::PmError  {
    FromPrimitive::from_i64(*self as i64).unwrap()
  }

}


/**  Translate portmidi error number into human readable message.
*    These strings are constants (set at compile time) so client has
*    no need to allocate storage
*/
pub fn get_error_text(error_code : PmError) -> String {
    unsafe {
        String::from_raw_buf((ffi::Pm_GetErrorText(error_code.wrap()) as *const u8))
    }
}

/**  Translate portmidi host error into human readable message.
    These strings are computed at run time, so client has to allocate storage.
    After this routine executes, the host error is cleared.
*/
pub fn get_host_error_text(msg : *const c_char , len : i32 ) {
    unsafe {
        ffi::Pm_GetHostErrorText(msg, len);
    }
}

pub const HDRLENGTH : i32 = 50;

/* any host error msg will occupy less
than this number of characters */
pub const PM_HOST_ERROR_MSG_LEN : i32 = 256;













/// Representation of an output midi port.
#[allow(missing_copy_implementations)]
pub struct PmOutputPort {
    c_pm_stream : *const ffi::CPortMidiStream,
    output_device : ffi::CPmDeviceID,
    buffer_size : i32,
}

impl PmOutputPort {
    /**
    * Constructor for PmOutputPort.
    *
    * Return a new PmOutputPort.
    */
    pub fn new(output_device : PortMidiDeviceId, buffer_size: i32) -> PmOutputPort {
        PmOutputPort {
            c_pm_stream : ptr::null(),
            output_device : output_device,
            buffer_size : buffer_size,
        }
    }

    pub fn open(&mut self)  -> PmError {

        unsafe {
            PmError::unwrap(ffi::Pm_OpenOutput(&self.c_pm_stream, self.output_device, ptr::null(), self.buffer_size, ptr::null(), ptr::null(), 0))
        }
    }

    /**
    *    Test whether stream has a pending host error. Normally, the client finds
    *    out about errors through returned error codes, but some errors can occur
    *    asynchronously where the client does not
    *    explicitly call a function, and therefore cannot receive an error code.
    *    The client can test for a pending error using has_host_error(). If true,
    *    the error can be accessed and cleared by calling get_Error_text().
    *    Errors are also cleared by calling other functions that can return
    *    errors, e.g. open_input(), open_output(), read(), write(). The
    *    client does not need to call Pm_HasHostError(). Any pending error will be
    *    reported the next time the client performs an explicit function call on
    *    the stream, e.g. an input or output operation. Until the error is cleared,
    *    no new error codes will be obtained, even for a different stream.
    */
    pub fn has_host_error(&self) -> i32  {
        unsafe {
            ffi::Pm_HasHostError(self.c_pm_stream)
        }

    }

    /**
        Pm_Abort() terminates outgoing messages immediately
        The caller should immediately close the output port;
        this call may result in transmission of a partial midi message.
        There is no abort for Midi input because the user can simply
        ignore messages in the buffer and close an input device at
        any time.
     */
    pub fn abort(&mut self) -> PmError {
        unsafe {
            PmError::unwrap(ffi::Pm_Abort(self.c_pm_stream))
        }
    }

    /**
        Pm_Close() closes a midi stream, flushing any pending buffers.
        (PortMidi attempts to close open streams when the application
        exits -- this is particularly difficult under Windows.)
    */
    pub fn close(&mut self)  -> PmError  {
        unsafe {
            PmError::unwrap(ffi::Pm_Close(self.c_pm_stream))
        }
    }

    /**
        Pm_Write() writes midi data from a buffer. This may contain:
            - short messages
        or
            - sysex messages that are converted into a sequence of PmEvent
              structures, e.g. sending data from a file or forwarding them
              from midi input.

        Use Pm_WriteSysEx() to write a sysex message stored as a contiguous
        array of bytes.

        Sysex data may contain embedded real-time messages.
    */
    pub fn write_event(&mut self, midievent : MidiEvent)  -> PmError  {
        let cevent : ffi::CPmEvent = midievent.unwrap();
        unsafe {
            PmError::unwrap(ffi::Pm_Write(self.c_pm_stream, &cevent, 1))
        }
    }

    /**
        Pm_WriteShort() writes a timestamped non-system-exclusive midi message.
        Messages are delivered in order as received, and timestamps must be
        non-decreasing. (But timestamps are ignored if the stream was opened
        with latency = 0.)
    */
    pub fn write_message(&mut self, midimessage : MidiMessage)  -> PmError  {
        let cevent : ffi::CPmMessage = midimessage.unwrap();
        unsafe {
            PmError::unwrap(ffi::Pm_WriteShort(self.c_pm_stream, 0, cevent))
        }
    }
}

