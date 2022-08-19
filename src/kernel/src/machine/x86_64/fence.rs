use core::arch::asm;

/// Memory Fence
///
/// Performs a serializing operation on all load-from-memory and store-to-memory instructions that
/// were issued prior the MFENCE instruction.
pub unsafe fn memory() {
    asm!("mfence");
}

/// Store Fence
///
/// Orders processor execution relative to all memory stores prior to the SFENCE instruction. The
/// processor ensures that every store prior to SFENCE is globally visible before any store after
/// SFENCE becomes globally visible.
pub fn store() {
    asm!("sfence");
}

/// Load Fence
///
/// Performs a serializing operation on all load-from-memory instructions that were issued prior
/// the LFENCE instruction. Specifically, LFENCE does not execute until all prior instructions
/// have completed locally, and no later instruction begins execution until LFENCE completes.
pub unsafe fn load() {
    asm!("lfence");
}
