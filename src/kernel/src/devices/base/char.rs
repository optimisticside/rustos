use alloc::collections::vec_deque::VecDeque;
use alloc::sync::Arc;
use core::fmt;
use crate::sync::rwlock::RwLock;
use crate::sync::Yield;
use crate::devices::{Device, DeviceSwitch, DeviceError};

/// A character device is one that only read and write one character at a time.
pub trait CharDeviceSwitch: DeviceSwitch {
    /// Write a single character to the device.
    fn get_char(&self) -> Result<u8, DeviceError>;
    /// Read a single character from the device.
    fn put_char(&mut self, byte: u8) -> Result<(), DeviceError>;
}

/// Wrapper for character devices so that they can be treated as generic devices (this works with
/// both character and block devices).
pub struct CharDevice {
    /// Inner character device switch.
    inner: dyn CharDeviceSwitch,
    /// Queue for characters to be written to the device.
    queue: Arc<RwLock<VecDeque<u8>, Yield>>,
}

impl Device for CharDevice {
    /// Read the given number of bytes (based on the size of the buffer array).
    fn read(&self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        // We can ignore the position parameter, which is better than reading them just to skip
        // over them.
        let mut queue = Arc::try_unwrap(self.queue).map(RwLock::into_inner)?;
        for byte in buffer.iter_mut() {
            *byte = match self.queue.pop_front() {
                Some(character) => character,
                None => self.inner.get_char()?,
            };
        }

        Ok(buffer.len())
    }

    /// Write all the given bytes to the device.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        let mut queue = Arc::try_unwrap(self.queue).map(RwLock::into_inner)?;
        if queue.len() > 0 {
            let mut to_append: VecDeque<u8> = buffer.into();
            queue.append(to_append);
        } else {
            for &byte in buffer {
                self.inner.put_char(byte)?;
            }
        }

        Ok(buffer.len())
    }

    /// Perform an I/O control operation.
    fn io_control(&mut self, command: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        self.inner.io_control(command, buffer)
    }
}

impl CharDeviceSwitch for CharDevice {
    /// Wrapper for CharDeviceSwitch::get_char.
    fn get_char(&self) -> Result<u8, DeviceError> {
        let mut queue = Arc::try_unwrap(self.queue).map(RwLock::into_inner)?;

        Ok(match queue.pop_front() {
            Some(queued) => queued,
            None => self.inner.get_char()
        })
    }

    /// Wrapper for CharDeviceSwitch::put_char.
    fn put_char(&mut self, byte: u8) -> Result<(), DeviceError> {
        if let Err(_) = self.inner.put_char(byte) {
            Arc::try_unwrap(self.queue)
                .map(RwLock::into_inner)?
                .push_back(byte);

        }

        Ok(())
    }
}

impl fmt::Write for CharDevice {
    /// Write a string to the character device, and unlike the character device, use UNIX-LF
    /// line-endings where `\n` translates to `\r\n`.
    fn write_str(&mut self, string: &str) -> fmt::Result {
        // TODO: Match result of [`CharDevice::put_char`] to handle any device errors.
        for byte in string.bytes() {
            if byte == '\n' as u8 {
                self.put_char('\r' as u8);
            }

            self.put_char(byte);
        }

        Ok(())
    }
}
