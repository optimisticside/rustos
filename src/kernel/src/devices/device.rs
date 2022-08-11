use crate::devices::DeviceError;

/// A device is an abstraction over a physical device's driver, which lets the caller read and
/// write bytes to and from the device at any position they want.
///
/// Generally, this trait is implemented by device-wrappers that hold an internal device "switch."
/// A device switch is the trait that implements all the I/O routines specific to that type of
/// device, which are implemented by the device's drivers.
pub trait Device {
    /// Read the given number of bytes (through the buffer length) into the provided buffer, from
    /// the given location.
    fn read(&self, position: usize, buffer: &[u8]) -> Result<(), DeviceError>;

    /// Write the given buffer of points at the given location.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<(), DeviceError>;
}
