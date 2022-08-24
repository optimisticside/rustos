/// A wrapper for physical addresses.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PhysAddr(pub u64);

impl PhysAddr {
    /// Converts the physical address to a `u64`.
    pub fn to_u64(self) -> u64 {
        self.0
    }

    /// Splits the physical address into lower and higher 32-bits.
    pub fn split(&self) -> (u32, u32) {
        (self.0 as u32, (self.0 >> 32) as u32)
    }
}
