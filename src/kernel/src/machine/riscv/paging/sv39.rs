/// Sv39 virtual addresses contain three 9-bit indices, called virtual Page Numbers (VPNs). These
/// index into an array of 512, 8-byte entries. They also containa page offset.
/// Bits 0-11: Page offset
/// Bits 12-20: VPN 0
/// Bits 21-29: VPN 1
/// Bits 30-38: VPN 2
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct VirtAddr(usize);

impl VirtAddr {
    /// Retrieve the page offset.
    pub fn page_offset(&self) -> usize {
        self.0 & 0x1FF
    }

    /// Retrieve the 0th VPN.
    pub fn vpn0(&self) -> usize {
        (self.0 >> 12) & 0x1FF
    }

    /// Retrieve the 1st VPN.
    pub fn vpn1(&self) -> usize {
        (self.0 >> 21) & 0x1FF
    }

    /// Retrieve the 2nd VPN.
    pub fn vpn2(&self) -> usize {
        (self.0 >> 30) & 0x1FF
    }
}

/// Like virtual addresses, Sv39 page table entries contain three 9-bit indices called physical
/// page numbers (PPNs). These index into an array of 512, 8-byte entries. The rest of the page
/// table entry contains flags and status-word data.
pub struct PageTableEntry(usize);

impl PageTableEntry {
    /// Retrieve the 0th PPN.
    pub fn ppn0(&self) -> usize {
        
    }
}

bitflags::bitflags! {
    pub struct TableEntryFlags: u8 {
        /// The entry can be accessed. Must be set for the entry to be used.
        const VISIBLE = 1 << 0;
        /// Entry type.
        const USER_EXECUTE = 1 << 3;
        const USER_WRITE = 2 << 3;
        const USER_READ = 3 << 3;
        const SUP_EXECUTE = 4 << 3;
        const SUP_WRITE = 5 << 3;
        const SUP_READ = 6 << 3;
        /// The entry has been read from.
        const REFERENCED = 1 << 4;
        /// The entry has been written to.
        const DIRTY = 1 << 5;
    }
}
