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
        (self.0 >> 12) & 0x1FF
    }

    /// Retrieve the 1th PPN.
    pub fn ppn1(&self) -> usize {
        (self.0 >> 21) & 0x1FF
    }

    /// Retrieve the 2nd PPN.
    pub fn ppn2(&self) -> usize {
        (self.0 >> 30) & 0x3FF_FFFF
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
        const ACCESSED = 1 << 4;
        /// The entry has been written to.
        const DIRTY = 1 << 5;
    }
}

/// Page-table structure. The only thing that it holds is an array of entries. (I question why I
/// even made this a structure instead of just an array).
#[repr(C, packed)]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl PageTable {
    /// Maps a physical address to a virtual address. Called internally by PageTable::map.
    pub fn map_address(
        &mut self,
        virt_addr: usize,
        phys_addr: usize,
        flags: PageTableEntryFlags,
        level: usize
    ) {
        let virt_addr = VirtAddr(virt_addr);
        let mut entry = &mut self.entries[virt_addr.vpn2()];

        for level in (level..2).rev() {
            if entry & PageTableEntryFLags::VISIBLE as usize != 0 {
                let page = PageTable::new();
                *entry = PageTableEntry::new(page as usize, PageTableEntryFlags::VISIBLE);
            }

            let new_entry = entry.phys_addr().0 as *mut PageTableEntry;
            *entry = unsafe { new_entry.add(virt_addr.index(level)).as_mut().unwrap() };
        }

        *entry = PageTableEntry::new(phys_addr, (flags | PageTableEntryFlags::VISIBLE).bits());
    }
}
