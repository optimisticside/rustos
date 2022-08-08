/// A block device is one that does operations on blocks, at random access. Each block is a unit of
/// data of an arbitrary size.
pub trait BlockDevice {
    /// Retrieve the size of each block in the device.
    fn get_block_size(&self),

    /// Reads data from a block into the given buffer.
    fn read_block(&self, block_num: usize, buffer: &[u8]),

    /// Writes to a given block.
    fn write_block(&mut self, block_num: usize, buffer: &[u8]),
}

// We do not currently do any block caching, which we will need to impelement some time in the
// future if we remotely care about performance.
impl<D: BlockDevice> Device for D {
    pub fn read(&self, position: usize, buffer: &[u8]) {
        let block_size = self.get_block_size();
        let block_num = position / block_size;
        let offset = position % block_size;


    }
}
