use alloc::collections::vec_deque::VecDeque;
use crate::devices::{CharDeviceSwitch, DeviceError};

/// Teletypes abstract over character devices, just like a buffered-device would abstract over a
/// block device.
pub struct Teletype {
    /// Internal character device.
    device: dyn CharDeviceSwitch,
    /// Output queue.
    output_queue: VecDeque<u8>,
    /// Canonical queue. This is when terminal input is processed in lines terminated by \n. No
    /// input can be read until the entire line has been read by the user.
    canonical_queue: VecDeque<u8>,
    /// Raw input quque.
    input_queue: VecDeque<u8>,
}

impl CharDeviceSwitch for Teletype {
    fn put_char(&mut self, byte: u8) -> Result<(), DeviceError> {

    }

    fn get_char(&mut self, byte: u8) -> Result<(), DeviceError> {
        
    }
}
