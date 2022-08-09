use crate::devices::Device;

/// A character device is one that only read and write one character at a time.
pub trait CharDevice {
    /// Write a single character to the device.
    pub fn get_char(&self, byte: u8) -> Result<()>,

    /// Read a single character from the device.
    pub fn put_char(&self, byte: u8) -> Result<()>,
}

impl<D: CharDevice> Device for D {
    /// Read the given number of bytes (based on the size of the buffer array).
    fn read(&self, position: usize, buffer: &[u8]) {
        // We can ignore the position parameter, which is better than reading them just to skip
        // over them.
        for byte in buffer.iter_mut() {
            *byte = self.get_char()?;
        }
    }

    /// Write all the given bytes to the device.
    fn write(&mut self, position: usize, buffer: &[u8]) {
        for &byte in buffer {
            self.put_char(byte)?;
        }
    }
}
