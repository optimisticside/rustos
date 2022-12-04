use crate::io::{IoVec, MemMappedIo};

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

#[repr(C)]
pub struct Available {
    pub flags: u16,
    pub index: u16,
    /// The driver uses the available-ring to offer buffers to the device. Each ring refers to the
    /// head of a descriptor-chain. It is only written to and read from by the device itself.
    pub ring: [u16; VIRTIO_RING_SIZE],
    pub event: u16,
}

#[repr(usize)]
pub enum MmioOffsets {
    MagicValue = 0x000,
    Version = 0x004,
    DeviceId = 0x008,
    VendorId = 0x00c,
    HostFeatures = 0x010,
    HostFeaturesSelector = 0x014,
    GuestFeatures = 0x020,
    GuestFeaturesSelector = 0x024,
    GuestPageSize = 0x028,
    QueueSelector = 0x030,
    QueueNumberMax = 0x034,
    QueueNumber = 0x038,
    QueueAlign = 0x03c,
    QueuePfn = 0x040,
    QueueNotify = 0x050,
    InterruptStatus = 0x060,
    InterruptAcknowledge = 0x064,
    Status = 0x070,
    Config = 0x100,
}
