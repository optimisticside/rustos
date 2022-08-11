use crate::devices::{Device, Error};

/// A character device is one that only read and write one character at a time.
pub trait CharDeviceSwitch {
    /// Write a single character to the device.
    fn get_char(&self) -> Result<u8, Error>;

    /// Read a single character from the device.
    fn put_char(&mut self, byte: u8) -> Result<(), Error>;
}

/// Wrapper for character devices so that they can be treated as generic devices (this works with
/// both character and block devices).
pub struct CharDevice {
    /// Inner character device switch.
    inner: dyn CharDeviceSwitch,
}

impl Device for CharDevice {
    /// Read the given number of bytes (based on the size of the buffer array).
    fn read(&self, position: usize, buffer: &[u8]) -> Result<(), Error> {
        // We can ignore the position parameter, which is better than reading them just to skip
        // over them.
        for byte in buffer.iter_mut() {
            *byte = self.inner.get_char()?;
        }

        Ok(())
    }

    /// Write all the given bytes to the device.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<(), Error> {
        for &byte in buffer {
            self.inner.put_char(byte)?;
        }

        Ok(())
    }
}

impl CharDeviceSwitch for CharDevice {
    /// Wrapper for CharDeviceSwitch::get_char.
    fn get_char(&self) -> Result<u8, Error> {
        self.inner.get_char()
    }

    /// Wrapper for CharDeviceSwitch::put_char.
    fn put_char(&mut self, byte: u8) -> Result<(), Error> {
        self.inner.put_char(byte)
    }
}
