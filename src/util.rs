/*!
* Function of PortMidi util.
*/

#[allow(non_camel_case_types)]


use std::{ptr};
use midi;

mod ffi {
    use libc::{c_void};

    pub type C_PmQueue = c_void; 
    pub type C_util_PmMessage = i32 ; 

    #[link(name = "portmidi")]
    extern "C" {
    	pub fn Pm_QueueCreate(num_msgs: i64, bytes_per_msg : u32) -> *const C_PmQueue;
    	pub fn Pm_QueueDestroy(queue : *const C_PmQueue) -> i32;
    	pub fn Pm_Dequeue(queue : *const C_PmQueue, mess :*mut C_util_PmMessage) -> i32;
    	pub fn Pm_Enqueue(queue : *const C_PmQueue, mess :*const C_util_PmMessage) -> i32;
    	pub fn Pm_QueueFull(queue : *const C_PmQueue) -> i32;
    	pub fn Pm_QueueEmpty(queue : *const C_PmQueue) -> i32;
    	pub fn Pm_QueuePeek(queue : *const C_PmQueue) -> *const C_util_PmMessage;
    	pub fn Pm_SetOverflow(queue : *const C_PmQueue) -> i32;
    }
}

/*
    A single-reader, single-writer queue is created by
    Pm_QueueCreate(), which takes the number of messages and
    the message size as parameters. The queue only accepts
    fixed sized messages. Returns NULL if memory cannot be allocated.

    This queue implementation uses the "light pipe" algorithm which
    operates correctly even with multi-processors and out-of-order
    memory writes. (see Alexander Dokumentov, "Lock-free Interprocess
    Communication," Dr. Dobbs Portal, http://www.ddj.com/, 
    articleID=189401457, June 15, 2006. This algorithm requires
    that messages be translated to a form where no words contain
    zeros. Each word becomes its own "data valid" tag. Because of
    this translation, we cannot return a pointer to data still in 
    the queue when the "peek" method is called. Instead, a buffer 
    is preallocated so that data can be copied there. Pm_QueuePeek() 
    dequeues a message into this buffer and returns a pointer to 
    it. A subsequent Pm_Dequeue() will copy from this buffer.

    This implementation does not try to keep reader/writer data in
    separate cache lines or prevent thrashing on cache lines. 
    However, this algorithm differs by doing inserts/removals in
    units of messages rather than units of machine words. Some
    performance improvement might be obtained by not clearing data
    immediately after a read, but instead by waiting for the end
    of the cache line, especially if messages are smaller than
    cache lines. See the Dokumentov article for explanation.

    The algorithm is extended to handle "overflow" reporting. To report
    an overflow, the sender writes the current tail position to a field.
    The receiver must acknowlege receipt by zeroing the field. The sender
    will not send more until the field is zeroed.
    
    Pm_QueueDestroy() destroys the queue and frees its storage.
 */

pub struct PmQueue {
	queue : *const ffi::C_PmQueue,
}

impl PmQueue{
    /**
    * Constructor for PmInputPort.
    *
    * Return a new PmInputPort.
    */
    pub fn new() -> PmQueue {
        PmQueue {
            queue : ptr::null(),
        }
    }

    pub fn create(&mut self, num_msgs : int, bytes_per_msg: uint)	{
    	self.queue = unsafe {ffi::Pm_QueueCreate(num_msgs as i64, bytes_per_msg as u32)};
    }

    pub fn destroy(&mut self) -> midi::PmError	{
    	unsafe	{
    		FromPrimitive::from_i64(ffi::Pm_QueueDestroy(self.queue)as i64).unwrap()
    	}
    }

  /* 
    Pm_Dequeue() removes one item from the queue, copying it into msg.
    Returns the message if successful, and pmNoError if the queue is empty.
    Returns pmBufferOverflow if what would have been the next thing
    in the queue was dropped due to overflow. (So when overflow occurs,
    the receiver can receive a queue full of messages before getting the
    overflow report. This protocol ensures that the reader will be 
    notified when data is lost due to overflow.
 */
   pub fn dequeue(&mut self) -> Result<midi::PmMessage, midi::PmError>	{
   		let mut cmes : ffi::C_util_PmMessage = 0;
        let retdata : midi::PmError = unsafe {
            FromPrimitive::from_i64(ffi::Pm_Dequeue(self.queue, &mut cmes) as i64).unwrap()
        };
        match retdata {
            midi::PmError::PmNoError => Err(midi::PmError::PmNoError),
            midi::PmError::PmGotData => Ok(midi::PmMessage::wrap(cmes)),
            _ => Err(retdata)
        }
   }

   /*
    Pm_Enqueue() inserts one item into the queue, copying it from msg.
    Returns pmNoError if successful and pmBufferOverflow if the queue was 
    already full. If pmBufferOverflow is returned, the overflow flag is set.
 */
   pub fn enqueue(&mut self, mess : midi::PmMessage) -> midi::PmError	{
   		unsafe	{
   			FromPrimitive::from_i64(ffi::Pm_Enqueue(self.queue, &mess.unwrap()) as i64).unwrap()
   		}
   }
/*
    Pm_QueueFull() returns non-zero if the queue is full
    Pm_QueueEmpty() returns non-zero if the queue is empty

    Either condition may change immediately because a parallel
    enqueue or dequeue operation could be in progress. Furthermore,
    Pm_QueueEmpty() is optimistic: it may say false, when due to 
    out-of-order writes, the full message has not arrived. Therefore,
    Pm_Dequeue() could still return 0 after Pm_QueueEmpty() returns
    false. On the other hand, Pm_QueueFull() is pessimistic: if it
    returns false, then Pm_Enqueue() is guaranteed to succeed. 

    Error conditions: Pm_QueueFull() returns pmBadPtr if queue is NULL.
    Pm_QueueEmpty() returns FALSE if queue is NULL.
 */
   pub fn is_full(&self) -> bool	{
   		unsafe	{
   			ffi::Pm_QueueFull(self.queue) > 0
   		}
   }

   pub fn is_empty(&self) -> bool	{
   		unsafe	{
   			ffi::Pm_QueueEmpty(self.queue) > 0
   		}
   }

/*
    Pm_QueuePeek() returns a pointer to the item at the head of the queue,
    or NULL if the queue is empty. The item is not removed from the queue.
    Pm_QueuePeek() will not indicate when an overflow occurs. If you want
    to get and check pmBufferOverflow messages, use the return value of
    Pm_QueuePeek() *only* as an indication that you should call 
    Pm_Dequeue(). At the point where a direct call to Pm_Dequeue() would
    return pmBufferOverflow, Pm_QueuePeek() will return NULL but internally
    clear the pmBufferOverflow flag, enabling Pm_Enqueue() to resume
    enqueuing messages. A subsequent call to Pm_QueuePeek()
    will return a pointer to the first message *after* the overflow. 
    Using this as an indication to call Pm_Dequeue(), the first call
    to Pm_Dequeue() will return pmBufferOverflow. The second call will
    return success, copying the same message pointed to by the previous
    Pm_QueuePeek().

    When to use Pm_QueuePeek(): (1) when you need to look at the message
    data to decide who should be called to receive it. (2) when you need
    to know a message is ready but cannot accept the message.

    Note that Pm_QueuePeek() is not a fast check, so if possible, you 
    might as well just call Pm_Dequeue() and accept the data if it is there.
 */
   pub fn peek(&mut self) -> Option<midi::PmMessage>	{
        let retdata : *const ffi::C_util_PmMessage = unsafe {
            ffi::Pm_QueuePeek(self.queue)
        };
        match retdata {
            y if y.is_null() => None,
            _ =>Some(unsafe {midi::PmMessage::wrap(*retdata)}),
        }
   }

/*
    Pm_SetOverflow() allows the writer (enqueuer) to signal an overflow
    condition to the reader (dequeuer). E.g. when transfering data from 
    the OS to an application, if the OS indicates a buffer overrun,
    Pm_SetOverflow() can be used to insure that the reader receives a
    pmBufferOverflow result from Pm_Dequeue(). Returns pmBadPtr if queue
    is NULL, returns pmBufferOverflow if buffer is already in an overflow
    state, returns pmNoError if successfully set overflow state.
 */
   pub fn set_overflow(&mut self) -> midi::PmError	{
   		unsafe	{
   			FromPrimitive::from_i64(ffi::Pm_SetOverflow(self.queue) as i64).unwrap()
   		}
   }
}


