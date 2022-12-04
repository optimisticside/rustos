use crate::device::{Device, DeviceError};

/// A block device is one that does operations on blocks, at random access. Each block is a unit of
/// data of an arbitrary size.
pub trait BlockDeviceSwitch {
    /// Retrieve the size of each block in the device.
    fn block_size(&self) -> usize;
    /// Reads data from a block into the given buffer.
    fn read_block(&mut self, block_num: usize, buffer: &[u8]) -> Result<(), DeviceError>;
    /// Writes to a given block.
    fn write_block(&mut self, block_num: usize, buffer: &[u8]) -> Result<(), DeviceError>;
}

/// Represents a routine that can perform an operation on a block-device. This type applies to the
/// `read` and `write` methods of [`BlockDeviceSwitch`]es.
type StrategyRoutine =
    fn(&mut dyn BlockDeviceSwitch, block_num: usize, buffer: &[u8]) -> Result<(), DeviceError>;

/// Wrapper for block devices so that they can be treated as generic devices (this works with
/// both character and block devices). Note that operations done on blocks must involve changing
/// the entire block.
pub struct BlockDevice {
    /// Inner block device switch.
    inner: dyn BlockDeviceSwitch,
}

impl BlockDevice {
    /// Perform an operation on the block device.
    /// Called by read and write routines since they both do the same thing, other than calling a
    /// different function in the [`BlockDeviceSwitch`].
    pub(self) fn strategy(
        &mut self,
        position: usize,
        buffer: &[u8],
        routine: StrategyRoutine,
    ) -> Result<usize, DeviceError> {
        let block_size = self.block_size();
        if position % block_size != 0 || buffer.len() % block_size != 0 {
            Err(DeviceError::InvalidBounds)
        }

        let mut read = 0;
        let start_block = position / block_size;

        for block in start_block..(start_block + buffer.len() / block_size) {
            let start = block_size * (block - start_block);
            self.inner
                .read_block(block, &buffer[start..start + block_size])?;
            read += block_size;
        }

        Ok(read)
    }
}

impl Device for BlockDevice {
    /// Read the given number of bytes (based on the size of the buffer array).
    fn read(&self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        self.strategy(position, buffer, self.inner.read_block)
    }

    /// Write all the given bytes to the device.
    fn write(&mut self, position: usize, buffer: &[u8]) -> Result<usize, DeviceError> {
        self.strategy(position, buffer, self.inner.write_block)
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
