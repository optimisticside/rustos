bitflags::bitflags! {
    struct GdtEntryFlags: u8 {
        const NULL = 0;
        const LONG_MODE = 1 << 5;
        const PROTECTED_MODE = 1 << 6;
    }
}

/// One allocated per entry in the GDT.
#[derive(Debug, Clone, Copy)]
pub(super) struct GdtEntry {
    /// Lower 2 bytes of the limit.
    low_limit: u16,

    /// Lower part of the base.
    low_base: u16,

    /// Third byte of lower part of the base.
    middle_base: u8,

    /// Access flags.
    /// Bits 1-5: segment type
    /// Bits 5-7: descriptor priority level
    /// Bit 7: flag that represents the segment being present
    access_flags: u8,

    /// Contains both the higher part of the limit and additional flags.
    /// Bits 1-4: higher part of limit.
    /// Bits 4-6: unused
    /// Bit 6: default 32 vs 16 bit size
    /// Bit 7: limit granularity (byte/page units)
    high_limit_flags: u8,

    /// Higher part of base.
    high_base: u8,
}

impl GdtEntry {
    const fn new(access_flags: u8, entry_flags: GdtEntryFlags) -> Self {
        Self {
            low_limit: 0x00,
            low_base: 0x00,
            middle_base: 0x00,
            access_flags,
            // Do not set any of the bits used for the high-limit.
            high_limit_flags: entry_flags.bits() & 0xF0,
            high_base: 0x00,
        }
    }

    fn set_offset(&mut self, offset: u32) {
        self.low_base = offset as u16;
        self.middle_base = (offset >> 16) as u8;
        self.high_base = (offset >> 24) as u8;
    }

    fn set_limit(&mut self, limit: u32) {
        self.low_limit = limit as u16;
        self.high_limit_flags = self.high_limit_flags & 0xF0 | ((limit >> 16) as u8) & 0x0F;
    }
}

/// Representation of the Task Segment Selector in long mode. Here it does not store info about the
/// task and instead stores the Interrupt Stack Table.
#[repr(C, packed)]
pub struct Tss {
    reserved: u32,

    /// Stack pointers to load the stack when a privilage change occurs from a from a lower one to
    /// a higher one.
    pub rsp: [u64; 3],
    resreved2: u64,

    /// Interrupt stack table. Contains stack pointers for when an entry in the IDT has an IST
    /// value other than 0.
    pub ist: [u64; 7],
    reserved3: u64,
    reserved4: u64,

    /// 16-bit offset to the I/O permission bitmap.
    pub iomap_base: u16,
}
