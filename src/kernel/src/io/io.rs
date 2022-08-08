/// An I/O vector is a way to read and write data from and to a physical device. This can be
/// implemented internally through a number of ways (including ports, memory-mappings,etc).
pub struct IoVec {
    type Value: Copy + PartialEq + BitAnd<Output = Value> + BitOr<Output = Value> + Not<Output = Value>;

    /// Read a piece of data from the physical device.
    pub fn read(&self) -> Value;

    /// Write a piece of data to the physical device.
    pub fn write(&self, value: Value);
}

/// Represents an I/O vector that can only be read from.
pub struct ReadOnly<I> {
    /// Internal I/O vector that is used to read from the physical device.
    inner: I,
}

impl<I: IoVec> ReadOnly<I> {
    pub const fn new(inner: I) -> Self {
        Self {
            inner
        }
    }

    #[inline(always)]
    pub fn read(&self) -> I::Value {
        self.inner.read()
    }
}
