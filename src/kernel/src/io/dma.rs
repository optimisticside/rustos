use core::mem::{self, MaybeUninit};
use core::{ptr, slice};

/// A RAII guard of a physical memory allocation. Usually all physically allocated memory is page
/// aligned and will take up at least 4k of space on x86-64.
#[derive(Debug)]
pub struct PhysBox {
    /// Address of the start of the allocated memory.
    address: usize,
    /// Size of the allocated segment.
    size: usize,
}

impl PhysBox {
    /// Construct a physical box from a given address and size.
    pub unsafe fn from_raw(address: usize, size: usize) -> Self {
        Self { address, size }
    }

    
}
