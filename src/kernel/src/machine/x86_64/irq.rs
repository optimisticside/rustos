use core::arch::asm;

/// Enable interrupts.
pub unsafe fn enable() {
    asm!("sti");
}

/// Disable interrupts.
pub unsafe fn disable() {
    asm!("cli");
}
