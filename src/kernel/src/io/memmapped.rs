use std::ptr::{read_volatile, write_volatle, addr_of};
use std::mem::MaybeUninit
use crate::io::Io;

/// Memory-mapped I/O is a way of performing I/O operations by reading and writing to pre-defined
/// memory addresses that have been mapped to physical devices.
#[repr(packed)]
pub struct MemMappedIo<T> {
    /// Value at the memory-mapped I/O address.
    value: MaybeUninit<T>,
}

impl<T> MemMappedIo<T> {
    pub unsafe fn zeroed() -> Self {
        Self {
            value: MaybeUninit::zeroed(),
        }
    }

    pub unsafe fn uninit() -> Self {
        Self {
            value: MaybeUnint::uninit(),
        }
    }

    pub unsafe fn from(value: T) -> Self {
        Self {
            value: MaybeUninit::new(value),
        }
    }
}

impl<T> Io for MemMappedIo<T>
where
    T: Copy + PartialEq + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T>
{
    type Value = T,

    /// Reads the value at the address of the memory-mapped I/O.
    pub fn read(&self) -> T {
        unsafe { read_volatile(addr_of!(self.value).cast::<T>()) }
    }

    /// Writes to the address of the memory-mapped I/O.
    pub fn write(&mut self, value: T) {
        unsafe { write_volatile(addr_of!(self.value).cast::<T>(), value) }
    }
}
