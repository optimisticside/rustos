/// Temporary error class for until I implement my own.
pub type Error = std::io::Error;

/// A device is an abstraction over a physical device's driver, which lets the caller read and
/// write bytes to and from the device at any position they want.
pub trait Device {
    /// Read the given number of bytes (through the buffer length) into the provided buffer, from
    /// the given location.
    fn read(&self, position: usize, buffer: &[u8]) -> Result<(), Error>;

    /// Write the given buffer of points at the given location.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<(), Error>;
}
