use crate::device::{Device, DeviceError};
use crate::network::Packet;

/// A network device is one that has the ability to send and recieve packets to and from given
/// destinations.
pub trait NetworkDeviceSwitch {
    /// Recieves a packet.
    fn recieve_packet(&mut self) -> Result<Packet, DeviceError>;
    /// Sends a packet.
    fn send_packet(&mut self, packet: &Packet) -> Result<(), DeviceError>;
}

/// Wrapper for network devices so that they can be treated as generic devices (this works with
/// all types of devices).
pub struct NetworkDevice {
    /// Inner network device switch.
    inner: dyn NetworkDeviceSwitch,
}

impl Device for NetworkDevice {
    /// Read the given number of bytes (based on the size of the buffer array).
    fn read(&self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        // We do not currently do any block caching, which we will need to impelement some time in the
        // future if we remotely care about performance.
        let block_size = self.block_size();
        let mut position = buffer % block_size;

        for block_num in (buffer / block_size) .. ((position + buffer.len()) / block_size) {
            // We need to allocate a buffer for the block if we are at the first or last block of
            // the read.
            let block = self.inner.read_block()
        }
        Ok(buffer.len())
    }

    /// Write all the given bytes to the device.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        Ok(buffer.len())
    }

    /// Perform an I/O control operation.
    fn io_control(&mut self, command: usize, buffer: &[u8]) -> Result<(), DeviceError> {
        self.inner.io_control(command, buffer)
    }
}

impl BlockDeviceSwitch for BlockDevice {
    /// Wrapper for BlockDeviceSwitch::block_size.
    fn block_size(&self) -> usize {
        self.inner.block_size()
    }

    /// Wrapper for BlockDeviceSwitch::read_block.
    fn read_block(&self, block_num: usize, buffer: &[u8]) -> Result<(), DeviceError> {
        self.inner.read_block(block_num, buffer)
    }

    /// Wrapper for BlockDeviceSwitch::write_char.
    fn write_block(&mut self, block_num: usize, buffer: &[u8]) -> Result<(), DeviceError> {
        self.inner.write_block(block_num, buffer)
    }
}
