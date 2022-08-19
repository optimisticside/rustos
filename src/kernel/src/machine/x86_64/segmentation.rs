use crate::machine::Ring;
use core::arch::asm;

bitflags::bitflags! {
    /// Specifies which element to load into a segment from descriptor tables (i.e., is a index to
    /// LDT or GDT table with some additional flags).
    ///
    /// See Intel 3a, Section 3.4.2 "Segment Selectors"
    pub struct SegmentSelector: u16 {
        /// Requestor Privilege Level
        const RPL_0 = 0b00;
        const RPL_1 = 0b01;
        const RPL_2 = 0b10;
        const RPL_3 = 0b11;

        /// Table Indicator (TI) 0 means GDT is used.
        const TI_GDT = 0 << 2;
        /// Table Indicator (TI) 1 means LDT is used.
        const TI_LDT = 1 << 2;
    }
}

impl SegmentSelector {
    /// Create a segment selector from its index and requestor privilege level.
    pub const fn new(index: u16, rpl: Ring) -> Self {
        Self {
            bits: index << 3 | (rpl as u16),
        }
    }

    /// Return the segment selector's index in the GDT or LDT.
    pub fn index(&self) -> u16 {
        self.bits >> 3
    }

    /// Create a segment selector from bits.
    pub fn from_raw(bits: u16) -> Self {
        Self { bits }
    }
}

/// Entry for the IDT, LDT, and GDT. Provides the size and location of a segment.
///
/// See Intel 3a, Section 3.4.5 "Segment Descriptors", and Section 3.5.2
#[derive(Copy, Clone, Debug, Default)]
#[repr(packed)]
pub struct Descriptor {
    pub lower: u32,
    pub upper: u32,
}

impl Descriptor {
    /// Create a new segment, TSS, or LDT descriptor by setting the base and limit fields of the
    /// descriptor.
    pub fn set_base_limit(&mut self, base: u32, limit: u32) {
        // Clear the base and limit fields.
        self.lower = 0;
        self.upper &= 0x00F0FF00;

        // Set the new base.
        self.lower |= base << 16;
        self.upper |= (base >> 16) & 0xFF;
        self.upper |= (base >> 24) << 24;

        // Set the new limit.
        self.lower |= limit & 0xFFFF;
        self.upper |= ((limit >> 16) & 0xFF) << 16;
    }

    /// Set the type of the descriptor. Indicates the segment or gate type and specifies the kinds
    /// of access and the direction of growth.
    pub fn set_type(&mut self, kind: u8) {
        // Clear the field before updating it.
        self.upper &= !(0x0F << 8);
        self.upper |= (kind as u32 & 0x0F) << 8;
    }

    /// Specify whether the segment descriptor if for a system segment (cleared) or a code or data
    /// segment (set).
    pub fn set_system(&mut self) {
        self.upper |= 1 << 12;
    }

    /// Specify the priviledge level of the segment. Used to control access to the segment.
    pub fn set_dpl(&mut self, ring: Ring) {
        assert!(ring as u32 <= 0b11);
        self.upper &= !(0b11 << 13);
        self.upper |= (ring as u32) << 13;
    }

    /// Set the present bit. Indicates whether the segment is in memory (set) or not present
    /// (clear). If clear, a segment-not-present exception is thrown.
    pub fn set_present(&mut self) {
        self.upper |= 1 << 15;
    }

    /// Set the AVL bit. Can be used by system software to store information.
    pub fn set_avl(&mut self) {
        self.upper |= 1 << 20;
    }

    /// Set the granularity bit. Determines the scaling of the segment limit field. The limit is
    /// interpreted in bytes when cleared and 4-KByte units when set.
    pub fn set_granularity(&mut self) {
        self.upper |= 1 << 23;
    }
}

/// Reload the code segment register.
pub unsafe fn load_cs(selector: SegmentSelector) {
    asm!("mov cs, {0:x}", in(reg) selector.bits(), options(nomem, nostack));
}

/// Reload the data segment register.
pub unsafe fn load_ds(selector: SegmentSelector) {
    asm!("mov ds, {0:x}", in(reg) selector.bits(), options(nomem, nostack));
}

/// Reload the stack segment register.
pub unsafe fn load_ss(selector: SegmentSelector) {
    asm!("mov ss, {0:x}", in(reg) selector.bits(), options(nomem, nostack));
}

/// Reload the extra segment register.
pub unsafe fn load_es(selector: SegmentSelector) {
    asm!("mov es, {0:x}", in(reg) selector.bits(), options(nomem, nostack));
}

/// Reload the fs segment register.
pub unsafe fn load_fs(selector: SegmentSelector) {
    asm!("mov fs, {0:x}", in(reg) selector.bits(), options(nomem, nostack));
}

/// Reload the gs segment register.
pub unsafe fn load_gs(selector: SegmentSelector) {
    asm!("mov gs, {0:x}", in(reg) selector.bits(), options(nomem, nostack));
}

/// Get the value of the code segment register.
pub fn cs() -> SegmentSelector {
    let segment: u16;
    unsafe {
        asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack));
    }
    SegmentSelector::from_raw(segment)
}

/// Get the value of the data segment register.
pub fn ds() -> SegmentSelector {
    let segment: u16;
    unsafe {
        asm!("mov {0:x}, ds", out(reg) segment, options(nomem, nostack));
    }
    SegmentSelector::from_raw(segment)
}

/// Get the value of the stack segment register.
pub fn ss() -> SegmentSelector {
    let segment: u16;
    unsafe {
        asm!("mov {0:x}, ss", out(reg) segment, options(nomem, nostack));
    }
    SegmentSelector::from_raw(segment)
}

/// Get the value of the extra segment register.
pub fn es() -> SegmentSelector {
    let segment: u16;
    unsafe {
        asm!("mov {0:x}, es", out(reg) segment, options(nomem, nostack));
    }
    SegmentSelector::from_raw(segment)
}

/// Get the value of the fs segment register.
pub fn fs() -> SegmentSelector {
    let segment: u16;
    unsafe {
        asm!("mov {0:x}, fs", out(reg) segment, options(nomem, nostack));
    }
    SegmentSelector::from_raw(segment)
}

/// Get the value of the gs segment register.
pub fn gs() -> SegmentSelector {
    let segment: u16;
    unsafe {
        asm!("mov {0:x}, gs", out(reg) segment, options(nomem, nostack));
    }
    SegmentSelector::from_raw(segment)
}
