use core::arch::asm;
use core::marker::PhantomData;

use crate::io::io::IoVec;

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
                options(nostack, nomem, preserves_flags)
            );
        }
        value
    }

    /// Write a byte to the port.
    #[inline(always)]
    fn write(&mut self, value: u8) {
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

/// Implementation for port I/O that involves words (two bytes).
impl IoVec for PortIo<u16> {
    type Value = u16;

    /// Reads a word from the port.
    #[inline(always)]
    fn read(&self) -> u16 {
        let value: u16;
        unsafe {
            asm!(
                "in ax, dx",
                in("dx") self.port,
                out ("ax") value,
                options(nostack, nomem, preserves_flags)
            );
        }
        value
    }

    /// Write a word to the port.
    #[inline(always)]
    fn write(&mut self, value: u16) {
        unsafe {
            asm!(
                "out dx, ax",
                in("dx") self.port,
                in("ax") value,
                options(nostack, nomem, preserves_flags)
            );
        }
    }
}

/// Implementation for port I/O that involves double-words (four bytes).
impl IoVec for PortIo<u32> {
    type Value = u32;

    /// Reads a double-word from the port.
    #[inline(always)]
    fn read(&self) -> u32 {
        let value: u32;
        unsafe {
            asm!(
                "in eax, dx",
                in("dx") self.port,
                out ("eax") value,
                options(nostack, nomem, preserves_flags)
            );
        }
        value
    }

    /// Write a double-word to the port.
    #[inline(always)]
    fn write(&mut self, value: u32) {
        unsafe {
            asm!(
                "out dx, eax",
                in("dx") self.port,
                in("eax") value,
                options(nostack, nomem, preserves_flags)
            );
        }
    }
}
