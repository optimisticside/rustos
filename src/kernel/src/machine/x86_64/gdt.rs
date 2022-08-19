use core::alloc::Layout;
use core::mem;

use crate::machine::segmentation::{self, Descriptor as SegmentDescriptor, SegmentSelector};
use crate::machine::dtables::{self, DescriptorTablePointer};
use crate::machine::Ring;

/// One allocated per entry in the global descriptor table (GDT).
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    /// Access flags.
    access: u8,
    limit_high_flags: u8,
    base_high: u8,
}

impl GdtEntry {
    /// Create a GDT entry based on its flags. Sets the base and limit to 0.
    const fn new(access: GdtAccessFlags, flags: GdtEntryFlags) -> Self {
        GdtEntry::from_raw(0, 0, access.bits(), flags.bits())
    }

    /// Create a GDT entry from all of its data.
    const fn from_raw(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        GdtEntry {
            limit_low: limit as u16,
            base_low: base as u16,
            base_middle: (base >> 16) as u8,
            access,
            limit_high_flags: flags & 0xF0 | ((limit >> 16) as u8) & 0x0F,
            base_high: (base >> 24) as u8,
        }
    }
}


static mut INIT_GDT: [GdtEntry; 4] = [
    // Null
    GdtEntry::new(GdtAccessFlags::NULL, GdtEntryFlags::NULL),
    // Kernel code
    GdtEntry::new(
        GdtAccessFlags::PRESENT
            | GdtAccessFlags::RING_0
            | GdtAccessFlags::SYSTEM
            | GdtAccessFlags::EXECUTABLE
            | GdtAccessFlags::PRIVILEDGE,
        GdtEntryFlags::LONG_MODE,
    ),
    // Kernel data
    GdtEntry::new(
        GdtAccessFlags::PRESENT
            | GdtAccessFlags::RING_0
            | GdtAccessFlags::SYSTEM
            | GdtAccessFlags::EXECUTABLE
            | GdtAccessFlags::PRIVILEDGE,
        GdtEntryFlags::LONG_MODE,
    ),
    // Kernel TLS
    GdtEntry::new(
        GdtAccessFlags::PRESENT
            | GdtAccessFlags::RING_0
            | GdtAccessFlags::SYSTEM
            | GdtAccessFlags::EXECUTABLE
            | GdtAccessFlags::PRIVILEDGE,
        GdtEntryFlags::LONG_MODE,
    ),
];

bitflags::bitflags! {
    struct GdtAccessFlags: u8 {
        const NULL = 0;
        /// Present bit. Must be set for every valid segment.
        const PRESENT: u8 = 1 << 7;
        /// Descriptor privilege level field. Contains the CPU privilege level of the segment.
        const RING_0 = 0 << 5;
        const RING_1 = 1 << 5;
        const RING_2 = 2 << 5;
        const RING_3 = 3 << 5;
        /// Descriptor type bit. System segment (like TSS) if clear, and code or data segment if set.
        const SYSTEM = 1 << 4;
        /// Executable bit. Data segment if clear, and code segment if set.
        const EXECUTABLE = 1 << 3;
        /// Data segments: Direction bit. Segment grows up if set and down if clear.
        /// Code segments: If set, can only be executed from ring level in DPL and if clear, can be
        /// executed from any ring level equal or lower.
        const DIRECTION_CONFORMING = 1 << 2;
        /// Readable/writiable bit.
        const PRIVILEGE = 1 << 1;
        /// Access bit. Automatically set by CPU when segment is accessed.
        const ACCESS = 1 << 0;
        const TSS_AVAIL = 9;
    }
}

pub struct GdtEntryType;
impl GdtEntryType {
    const NULL: u16 = 0;
    const KERNEL_CODE: u16 = 1;
    const KERNEL_DATA: u16 = 2;
    const KERNEL_TLS: u16 = 3;
    const USER_CODE32_UNUSED: u16 = 4;
    const USER_DATA: u16 = 5;
    const USER_CODE: 16 = 6;
    const TSS: u16 = 7;
    const TSS_HIGH: u16 = 8;
    const CPUID_CONTAINER: u16 = 9;
}

bitflags::bitflags! {
    struct GdtEntryFlags: u8 {
        const NULL = 0;
        const PROTECTED_MODE = 1 << 6;
        const LONG_MODE = 1 << 5;
    }
}

/// Initialize the global descriptor table.
pub unsafe fn init() {
    // Set up the initial GDT so that we can set up the actual GDT with a TLS. This means each CPU
    // will be assigned its own GDT.
    let limit = (INIT_GDT.len() * mem::size_of::<GdtEntry>() - 1)
        .try_into()
        .expect("Initial GDT is too large");
    let base = INIT_GDT as *const SegmentDescriptor;
    let init_gdtr = DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer::new(
        limit,
        base
    );

    // Load the initial GDT.
    dtables::load_gdt(&init_gdtr);

    // Load the segment descriptors.
    segmentation::load_cs(SegmentSelector::new(GdtEntryType::KERNEL_DATA, Ring::Ring0));
    segmentation::load_ds(SegmentSelector::new(GdtEntryType::KERNEL_DATA, Ring::Ring0));
    segmentation::load_es(SegmentSelector::new(GdtEntryType::KERNEL_DATA, Ring::Ring0));
    segmentation::load_fs(SegmentSelector::new(GdtEntryType::KERNEL_DATA, Ring::Ring0));
    segmentation::load_gs(SegmentSelector::new(GdtEntryType::KERNEL_DATA, Ring::Ring0));
    segmentation::load_ss(SegmentSelector::new(GdtEntryType::KERNEL_DATA, Ring::Ring0));
}
