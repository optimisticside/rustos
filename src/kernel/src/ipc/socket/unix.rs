use alloc::collections::VecDeque;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;

use crate::filesys::FileSysError;
use crate::filesys::vnode::SocketInterface;
use crate::ipc::message::{MessageQueue, Message};

#[derive(Debug)]
pub enum UnixSocketState {
    /// Not connected.
    #[default]
    Disconnected,
    /// Socket is listening for new connections.
    Listening(AcceptQueue),
    /// Socket is connected to another peer.
    Connected(Arc<UnixSocket>),
}

impl SocketInterface for UnixSocket {
    fn listen(&self, backlog: usize) -> Result<(), FileSysError> {
        
    }
}
