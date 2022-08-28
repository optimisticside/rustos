use crate::devices::{Device, DeviceSwitch, DeviceError};
use core::fmt;

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
}

impl Device for CharDevice {
    /// Read the given number of bytes (based on the size of the buffer array).
    fn read(&self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        // We can ignore the position parameter, which is better than reading them just to skip
        // over them.
        for byte in buffer.iter_mut() {
            *byte = self.inner.get_char()?;
        }

        Ok(buffer.len())
    }

    /// Write all the given bytes to the device.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        for &byte in buffer {
            self.inner.put_char(byte)?;
        }

        Ok(buffer.len())
    }

    /// Perform an I/O control operation.
    fn ioctl(&mut self, command: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        self.inner.ioct(command, buffer)
    }
}

impl CharDeviceSwitch for CharDevice {
    /// Wrapper for CharDeviceSwitch::get_char.
    fn get_char(&self) -> Result<u8, DeviceError> {
        self.inner.get_char()
    }

    /// Wrapper for CharDeviceSwitch::put_char.
    fn put_char(&mut self, byte: u8) -> Result<(), DeviceError> {
        self.inner.put_char(byte)
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
