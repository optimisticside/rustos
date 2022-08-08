use std::marker::PhantomData;
use super::IoVec;

/// Generic port I/O. This is when part of the CPU's address map is reserved for ports, which can
/// be written and read from through special instructions (which in the case of x86 are `in` and
/// `out`.
#[derive(Copy, Clone)]
pub struct PortIo<T> {
    port: u16,
    value: PhantomData<T>,
}

impl<T> PortIo<T> {
    /// Create a port I/O vector at the given port address.
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            value: PhantomData,
        }
    }
}

/// Implementation for port I/O that involves bytes.
impl IoVec for PortIo<u8> {
    type Value = u8;

    /// Reads a byte from the port.
    #[inline(always)]
    fn read(&self) -> u8 {
        let value: u8;
        unsafe {
            asm!(
                "in al, dx",
                in("dx") self.port,
                out ("al") value,
                options(nostack, nomem, preserve_flags)
            );
        }
        value
    }

    /// Write a byte to the port.
    #[inline(always)]
    fn write(&self, value: u8) {
        unsafe {
            asm!(
                "out dx, al",
                in("dx") self.port,
                in("al") value,
                options(nostack, nomem, preserves_flags)
            );
        }
    }
}
