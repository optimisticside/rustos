use crate::devices::DeviceError;

/// A device switch is the trait that implements all the I/O routines specific to that type of
/// device, which are implemented by the device's drivers, and use I/O Vectors to communicate with
/// physical devices (through port I/O, memory-mapped I/O, etc).
pub trait DeviceSwitch {
    /// Perform an I/O control operation.
    fn io_control(&mut self, _command: usize, _buffer: &[u8]) -> Result<(), DeviceError> {
        Ok(())
    }
}

/// A device is an abstraction over a physical device's driver, which lets the caller read and
/// write bytes to and from the device at any position they want.
///
/// Generally, this trait is implemented by device-wrappers that hold an internal
/// [`Deviceswitch`]es.
pub trait Device {
    /// Read the given number of bytes (through the buffer length) into the provided buffer, from
    /// the given location.
    fn read(&self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError>;
    /// Write the given buffer of points at the given location.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError>;
    /// Perform an I/O control operation. This is for operations that are specific to the device
    /// and cannot be expressed through read and write operationgs.
    fn io_control(&mut self, command: usize, buffer: &[u8]) -> Result<(), DeviceError>;
}
