use alloc::collections::vec_queue::VecQueue;
use alloc::vec::Vec;

/// Representation of an IPC message payload. Paylods can be of an arbritrary size, and are stored
/// in a vector. These are used in the request-response IPC system, as well as sockets.
#[derive(Debug, Default)]
pub struct Message {
    /// Message payload.
    data: Vec<u8>,
}

impl Message {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

/// Stores a queue of messages. Allows writing and reading messages from and into provided buffers.
pub struct MessageQueue {
    /// Queue of messages.
    messages: VecQueue<Message>,
}

impl MessageQueue {
    /// Determines whether the message queue is empty or not.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Reads a message from the message queue into the provided buffer, and returns the number of
    /// bytes that were read.
    pub fn read(&mut self, buffer: &[u8]) -> usize {
        if let Some(message) = self.messages.pop_front() {
            let message_len = message.data.len();
            assert!(buffer.len() >= message_len);

            buffer[..message_len].copy_from_slice(message.data_as_slice());
            message_len
        }
    }

    /// Creates a new message with the data provided in the given buffer and adds it to the queue.
    pub fn write(&mut self, buffer: &[u8]) {
        let message = Message::new(buffer.to_vec());
        self.messages.push_back(message);
    }
}
