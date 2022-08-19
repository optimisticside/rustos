use crate::machine::segmentation::SegmentSelector;
use core::arch::asm;
use core::mem::size_of;

/// Describe a pointer to a descriptor table (GDT / IDT). Used to provide information about the
/// tables to the `lgdt` and `lidt` instructions.
#[repr(C, packed)]
pub struct DescriptorTablePointer<Entry> {
    /// Size of the descriptor table, subtracted by one.
    limit: u16,
    /// Pointer to the memory region containing the descriptor table.
    base: *const Entry,
}

impl<T> Default for DescriptorTablePointer<T> {
    fn default() -> Self {
        Self {
            limit: 0,
            base: core::ptr::null(),
        }
    }
}

impl<T> DescriptorTablePointer<T> {
    fn new(table: &T) -> Self {
        let length = size_of::<T>() - 1;
        assert!(length < 0x10000);
        Self {
            base: table as *const T,
            limit: length as u16,
        }
    }

    fn from_slice(table: &[T]) -> Self {
        let length = table.len() - 1;
        assert!(length < 0x100000);
        Self {
            base: table.as_ptr(),
            limit: length as u16,
        }
    }
}

/// Load the segment selector into the selector field of the local descriptor table register
/// (LDTR).
#[inline(always)]
pub unsafe fn store_ldtr(selector: SegmentSelector) {
    asm!("lldt {0:x}", in(reg) selector.bits());
}

/// Return the segment selector from the local descriptor table register (LDTR).
pub unsafe fn load_ldtr() -> SegmentSelector {
    let selector: u16;
    unsafe {
        asm!("sldt {0}", out(reg) selector);
    }
    SegmentSelector::from_raw(selector)
}

/// Load the GDTR register with the specified descriptor table pointer.
#[inline(always)]
pub unsafe fn load_gdt<T>(gdt: &DescriptorTablePointer<T>) {
    asm!("lgdt [{0}]", in(reg) gdt, options(nostack));
}

/// Retrieve the base and the limit from the GDTR register.
#[inline(always)]
pub unsafe fn store_gdt<T>(gdt: &mut DescriptorTablePointer<T>) {
    asm!("sgdt [{0}]", in(reg) gdt as *mut DescriptorTablePointer<T>, options(nostack));
}
