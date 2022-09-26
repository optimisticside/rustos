use crate::devices::BlockDeviceSwitch;
use crate::sync::{RwLock, Yield};

/// Each buffer represents the cached data of a physical block/sector on the disk.
pub struct Buffer: Send + Sync {
    /// Slice containing buffer's data.
    data: Arc<RwLock<&[u8], Yield>>,
    /// Size of buffer.
    size: usize,
    /// Flags
}

/// Buffered devices can be used just like block devices, but internally manage a buffer. They
/// implement the [`BlockDeviceSwitch`], and can be provided to [`BlockDevice`]s.
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
