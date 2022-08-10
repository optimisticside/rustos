use crate::devices::{Device, Error};

/// A block device is one that does operations on blocks, at random access. Each block is a unit of
/// data of an arbitrary size.
pub trait BlockDeviceSwitch {
    /// Retrieve the size of each block in the device.
    fn get_block_size(&self);

    /// Reads data from a block into the given buffer.
    fn read_block(&self, block_num: usize, buffer: &[u8]);

    /// Writes to a given block.
    fn write_block(&mut self, block_num: usize, buffer: &[u8]);
}

/// Wrapper for block devices so that they can be treated as generic devices (this works with
/// both character and block devices).
pub struct BlockDevice {
    /// Inner character device.
    BlockDeviceSwitch inner;
}

impl BlockDevice for Device {
    /// Read the given number of bytes (based on the size of the buffer array).
    fn read(&self, position: usize, buffer: &[u8]) -> Result<(), Error> {
        // We do not currently do any block caching, which we will need to impelement some time in the
        // future if we remotely care about performance.
        Ok(())
    }

    /// Write all the given bytes to the device.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<(), Error> {
        Ok(())
    }
}

impl BlockDevice for BlockDeviceSwitch {
    /// Wrapper for BlockDeviceSwitch::read_block.
    fn read_block(&self, block_num: usize) -> Result<(), Error> {
        self.inner.read_block(block_num)
    }

    /// Wrapper for BlockDeviceSwitch::write_char.
    fn write_block(&mut self, block_num: usize, data: &[u8]) -> Result<(), Error> {
        self.inner.write_block(block_num, data)
    }
}
