/// One allocated per entry in the interrupt descriptor table.
#[repr(C, packed)]
pub(super) struct IdtEntry {
    /// Lower 2 bytes of the offset.
    low_offset: u16,

    /// Segment selector that identifies a segment.
    /// Bits 0-2: Privilage level of the selector.
    /// Bit 2: Specifies what descriptor to use (0 for GDT and 1 for LDT).
    /// Bits 3-15: Index of the table referenced by the selector.
    selector: u16,

    /// Flags about the entry.
    /// Bits 0-4: Gate type.
    /// Bit 4: Unused
    /// Bits 5-7: The privilege levels allowed to access the interrupt through the INT instruction.
    /// Bit 7: Present bit.
    type_attributes: u8,

    /// Middle 2 bytes of the offset.
    middle_offset: u16,

    /// Highest byte of the offset.
    high_offset: u8,
}

impl IdtEntry {
    fn set_flags(&mut self, flags: IdtEntryFlags) {
        self.type_attributes = flags;
    }

    /// Set the offset value of the IDT entry.
    fn set_offset(&mut self, selector: u16, base: usize) {
        self.selector = selector;
        self.low_offset = base as u16;
        self.middle_offset = (base >> 16) as u16;
        self.high_offset = (base >> 32) as u16;
    }

    /// Set the handler of the IDT entry.
    pub(crate) fn set_handler(&mut self, handler: *const u8) {
        self.set_flags(IdtEntryFlags::PRESENT | IdtEntryFlags::RING_0 | IdtEntryFlags::INTERRUPT);
        self.set_offset(8, handler as usize);
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct PreservedRegisters {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
}

/// Stores the registers automatically pushed onto the stack by the CPU (and needed to be pushed
/// onto the stack before calling the `iretq` instruction).
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct IretRegisters {
    pub rip: u64,
    pub cs: u64,
    pub rsp: u64,
    pub ss: u64,
}
