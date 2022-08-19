use core::arch::asm;
use crate::machine::ctrlregs;

/// Invalidate the given address in the TLB through the `invlpg` instruction.
pub unsafe fn flush(addr: usize) {
    asm!("invlpg [{}]", in(reg) addr, options(nostack, preserves_flags));
}

/// Invalidates the entire TLB by setting the CR3 register to itself.
pub unsafe fn flush_all() {
    ctrlregs::write_cr3(ctrlregs::cr3());
}
