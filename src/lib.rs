extern crate libc;
extern crate core;
extern crate serialize;

use std::ptr;
use core::mem::transmute;
use libc::c_char;

mod ffi;


// Types
// -----
pub type PortMidiDeviceId = i32;
pub type PortMidiResult<T> = Result<T, PortMidiError>;

// Errors
// ------
#[deriving(Copy, Show)]
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







#[deriving(Clone, Copy, PartialEq, Eq, Decodable, Encodable, Show)]
pub struct PmMessage { /**< see PmEvent */
    pub status : i8,
    pub data1 : i8,
    pub data2 : i8,
}

/**
    Pm_Message() encodes a short Midi message into a 32-bit word. If data1
    and/or data2 are not present, use zero.

    Pm_MessageStatus(), Pm_MessageData1(), and
    Pm_MessageData2() extract fields from a 32-bit midi message.
*/
impl PmMessage {
    fn wrap(cmessage : ffi::CPmMessage) -> PmMessage {
        PmMessage {
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


/**
   All midi data comes in the form of PmEvent structures. A sysex
   message is encoded as a sequence of PmEvent structures, with each
   structure carrying 4 bytes of the message, i.e. only the first
   PmEvent carries the status byte.

   Note that MIDI allows nested messages: the so-called "real-time" MIDI
   messages can be inserted into the MIDI byte stream at any location,
   including within a sysex message. MIDI real-time messages are one-byte
   messages used mainly for timing (see the MIDI spec). PortMidi retains
   the order of non-real-time MIDI messages on both input and output, but
   it does not specify exactly how real-time messages are processed. This
   is particulary problematic for MIDI input, because the input parser
   must either prepare to buffer an unlimited number of sysex message
   bytes or to buffer an unlimited number of real-time messages that
   arrive embedded in a long sysex message. To simplify things, the input
   parser is allowed to pass real-time MIDI messages embedded within a
   sysex message, and it is up to the client to detect, process, and
   remove these messages as they arrive.

   When receiving sysex messages, the sysex message is terminated
   by either an EOX status byte (anywhere in the 4 byte messages) or
   by a non-real-time status byte in the low order byte of the message.
   If you get a non-real-time status byte but there was no EOX byte, it
   means the sysex message was somehow truncated. This is not
   considered an error; e.g., a missing EOX can result from the user
   disconnecting a MIDI cable during sysex transmission.

   A real-time message can occur within a sysex message. A real-time
   message will always occupy a full PmEvent with the status byte in
   the low-order byte of the PmEvent message field. (This implies that
   the byte-order of sysex bytes and real-time message bytes may not
   be preserved -- for example, if a real-time message arrives after
   3 bytes of a sysex message, the real-time message will be delivered
   first. The first word of the sysex message will be delivered only
   after the 4th byte arrives, filling the 4-byte PmEvent message field.

   The timestamp field is observed when the output port is opened with
   a non-zero latency. A timestamp of zero means "use the current time",
   which in turn means to deliver the message with a delay of
   latency (the latency parameter used when opening the output port.)
   Do not expect PortMidi to sort data according to timestamps --
   messages should be sent in the correct order, and timestamps MUST
   be non-decreasing. See also "Example" for Pm_OpenOutput() above.

   A sysex message will generally fill many PmEvent structures. On
   output to a PortMidiStream with non-zero latency, the first timestamp
   on sysex message data will determine the time to begin sending the
   message. PortMidi implementations may ignore timestamps for the
   remainder of the sysex message.

   On input, the timestamp ideally denotes the arrival time of the
   status byte of the message. The first timestamp on sysex message
   data will be valid. Subsequent timestamps may denote
   when message bytes were actually received, or they may be simply
   copies of the first timestamp.

   Timestamps for nested messages: If a real-time message arrives in
   the middle of some other message, it is enqueued immediately with
   the timestamp corresponding to its arrival time. The interrupted
   non-real-time message or 4-byte packet of sysex data will be enqueued
   later. The timestamp of interrupted data will be equal to that of
   the interrupting real-time message to insure that timestamps are
   non-decreasing.
 */
#[deriving(Clone, Copy, PartialEq, Eq, Decodable, Encodable, Show)]
pub  struct PmEvent {
    pub message : PmMessage,
    pub timestamp : ffi::CPmTimestamp,
}

#[doc(hidden)]
impl PmEvent {
    fn wrap(cevent : ffi::CPmEvent) -> PmEvent {
        PmEvent {
            message:  PmMessage::wrap(cevent.message),
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


/// Representation of an input midi port.
#[allow(missing_copy_implementations)]
pub struct PmInputPort {
    c_pm_stream : *const ffi::CPortMidiStream,
    input_device : ffi::CPmDeviceID,
    buffer_size : i32,
}

impl PmInputPort {
    /**
    * Constructor for PmInputPort.
    *
    * Return a new PmInputPort.
    */
    pub fn new(input_device : PortMidiDeviceId, buffer_size: i32) -> PmInputPort {
        PmInputPort {
            c_pm_stream : ptr::null(),
            input_device : input_device,
            buffer_size : buffer_size,
        }
    }

    pub fn open(&mut self)  -> PmError {
        unsafe {
            PmError::unwrap(ffi::Pm_OpenInput(&self.c_pm_stream, self.input_device, ptr::null(), self.buffer_size, ptr::null(), ptr::null()))
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
        Read one midi note.
        Retur the note event if available or Err(pmNoError) if no midi event is avaible or Err() if an error occurs.

        Pm_Read() retrieves midi data into a buffer, and returns the number
        of events read. Result is a non-negative number unless an error occurs,
        in which case a PmError value will be returned.

        Buffer Overflow

        The problem: if an input overflow occurs, data will be lost, ultimately
        because there is no flow control all the way back to the data source.
        When data is lost, the receiver should be notified and some sort of
        graceful recovery should take place, e.g. you shouldn't resume receiving
        in the middle of a long sysex message.

        With a lock-free fifo, which is pretty much what we're stuck with to
        enable portability to the Mac, it's tricky for the producer and consumer
        to synchronously reset the buffer and resume normal operation.

        Solution: the buffer managed by PortMidi will be flushed when an overflow
        occurs. The consumer (Pm_Read()) gets an error message (pmBufferOverflow)
        and ordinary processing resumes as soon as a new message arrives. The
        remainder of a partial sysex message is not considered to be a "new
        message" and will be flushed as well.

    */
    pub fn read(&mut self) -> Result<PmEvent, PmError> {

        //get one note a the time
         let mut pevent : ffi::CPmEvent = ffi::CPmEvent {
            message : 0,
            timestamp : 0,
        };
//        println!("portmidi::midi before read In stream:{:?}", self.c_pm_stream);
        let nbnote : i16 = unsafe {
            ffi::Pm_Read(self.c_pm_stream, &mut pevent, 1)
        };
//        println!("portmidi::midi after read");
        match nbnote {
            y if y == 0 => Err(PmError::PmNoError),
            y if y > 0 => Ok(PmEvent::wrap(pevent)),
            _ => Err(unsafe { transmute::<i16, PmError>(nbnote) })
        }
    }

    /**
        Pm_Poll() tests whether input is available,
        returning pmGotData, pmNoError, or an error value.
    */
    pub fn poll(&self)  -> PmError  {
        unsafe {
            PmError::unwrap(ffi::Pm_Poll(self.c_pm_stream))
        }
    }

    /**
        Pm_Close() closes a midi stream, flushing any pending buffers.
        (PortMidi attempts to close open streams when the application
        exits -- this is particularly difficult under Windows.)
    */
    pub fn close(&mut self)  -> PmError  {
//      println!("portmidi::midi inport close");
        unsafe {
            PmError::unwrap(ffi::Pm_Close(self.c_pm_stream))
        }
    }
}


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
    pub fn write_event(&mut self, midievent : PmEvent)  -> PmError  {
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
    pub fn write_message(&mut self, midimessage : PmMessage)  -> PmError  {
        let cevent : ffi::CPmMessage = midimessage.unwrap();
        unsafe {
            PmError::unwrap(ffi::Pm_WriteShort(self.c_pm_stream, 0, cevent))
        }
    }
}

