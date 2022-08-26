use crate::devices::BlockDeviceSwitch;

/// Each buffer represents the cached data of a physical block/sector on the disk.
pub struct Buffer {
    
}

/// Buffered devices can be used just like block devices, but have 
struct BufferedDevice {
    /// Internal block-device.
    device: dyn BlockDeviceSwitch,
}

// Lets the buffered-device be treated like a block device so that we do not have to implement the
// `Device` trait manually.
impl BlockDeviceSwitch for BufferedDevice {
    /// Get the block size of the device. Internally calls BlockDeviceSwitch::block_size.
    fn block_size(&self) -> usize {
        self.device.block_size()
    }

    /// Reads the block, 
    fn read_block(&self, block_num: usize, buffer: &[u8]) -> Result<(), DeviceError> {
        self.device.read_block(block_num, buffer)
    }

    fn write_block(&mut self, block_num: usize, buffer: &[u8]) -> Result<(), DeviceError> {
        self.device.write_block(block_num, buffer)
    }
}
