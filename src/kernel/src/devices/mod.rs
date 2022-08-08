pub trait CharDevice {
    /// Read the specified number of bytes.
    pub fn read(&self, buffer: &[u8]) -> Result<()>,
    /// Write all the provided bytes.
    pub fn write(&self, buffer: &[u8]) -> Result<()>,
}
