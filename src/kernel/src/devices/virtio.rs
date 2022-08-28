bitflags::bitflags! {
    pub struct DescriptorFlags: u16 {
        const NEXT = 1 << 0;
        const WRITE = 1 << 1;
        const INDIRECT = 1 << 2;
        const RING_INDIRECT_DESC = 0b111000;
        const RING_EVENT_IDX = 0;
    }
}

/// Holds the data that we need to send to the device.
#[repr(C)]
pub struct Descriptor {
    /// This is the physical address, NOT the virtual address.
    pub address: u64,
    /// Length in bytes.
    pub length: u32,
    /// Flags.
    pub flags: u16,
    /// Pointer to chained descriptor (only if DescriptorFlags::NEXT set).
    pub next: u16,
}


