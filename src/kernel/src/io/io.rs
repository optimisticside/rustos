use core::cmp::PartialEq;
use core::ops::{BitAnd, BitOr, Not};

/// An I/O vector is a way to read and write data from and to a physical device. This can be
/// implemented internally through a number of ways (including ports, memory-mappings,etc).
pub trait IoVec {
    type Value: Copy
        + PartialEq
        + BitAnd<Output = Self::Value>
        + BitOr<Output = Self::Value>
        + Not<Output = Self::Value>;

    /// Read a piece of data from the physical device.
    fn read(&self) -> Self::Value;

    /// Write a piece of data to the physical device.
    fn write(&self, value: Self::Value);
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

    /// Read from the physical device.
    #[inline(always)]
    pub fn read(&self) -> I::Value {
        self.inner.read()
    }
}

/// Represents an I/O vector that can only be written to.
pub struct WriteOnly<I> {
    /// Internal I/O vector that is used to write to the physical device.
    inner: I,
}

impl<I: IoVec> ReadOnly<I> {
    pub const fn new(inner: I) -> Self {
        Self {
            inner
        }
    }

    /// Write to the physical device.
    #[inline(always)]
    pub fn write(&mut self, value: I::Value) {
        self.inner.write(value)
    }
}
