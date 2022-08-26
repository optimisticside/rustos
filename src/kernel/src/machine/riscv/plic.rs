/// Addresses of registers.
pub const BASE: usize = 0x0c00_0000;
pub const PRIORITY: usize = PLIC_BASE + 0x0;
pub const PENDING: usize = PLIC_BASE + 0x1000;
/// Enable registers. Write to them to toggle interrupts, which are represented by bits in the
/// register (a bitset). The interrupt's ID is the bit index.
pub const MACH_ENABLE_BASE: usize = PLIC_BASE + 0x2000;
pub const SUP_ENABLE_BASE: usize = PLIC_BASE + 0x2080;
/// Priority registers. Store interrupt thresholds and priorities.
pub const MACH_PRIORITY_BASE: usize = PLIC_BASE + 0x200000;
pub const SUP_PRIORITY_BASE: usize = PLIC_BASE + 0x201000;
/// Claim registers. Read from them to get pending interrupt and write to them to mark interrupts
/// as completed.
pub const MACH_CLAIM_BASE: usize = PLIC_BASE + 0x200004;
pub const SUP_CLAIM_BASE: usize = PLIC_BASE + 0x201004;

/// Retrieve the base of the registers for supervisor-mode claims.
pub const fn sup_claim_base(hart: usize) -> usize {
    PLIC_SUP_CLAIM_BASE + hart * 0x2000
}

/// Retrieves the base of the registers for machine-mode claims.
pub const fn mach_claim_base(hart: usize) -> usize {
    PLIC_MACH_CLAIM_BASE + hart * 0x2000
}

/// Retrieve the base of the supervisor-mode interrupt-enabled bitset.
pub const fn sup_enable_base(hart: usize) -> usize {
    PLIC_SUP_ENABLE_BASE + hart * 0x100
}

/// Retrieve the base of the machine-mode interrupt-enabled bitset.
pub const fn mach_enable_base(hart: usize) -> usize {
    PLIC_MACH_ENABLE_BASE + hart * 0x100
}

/// Retrieve the next available interrupt. This is by a "claim" process, where the PLIC will
/// give us the ID of the highest-priority interrupt after sorting them.
pub fn next(&mut self) -> Option<u32> {
    let claim_register = sup_claim_base() as *const u32;
    let claim_num = unsafe { claim_register.read_volatile() };

    // The 0-interrupt tells us that there is no interrupt to claim.
    if claim_num == 0 {
        None
    } else {
        Some(claim_num)
    }
}

/// Complete a pending interrupt by its ID. The ID should come from the [`next`] function.
pub fn complete(id: u32) {
    let complete_register = sup_claim_base() as *mut u32;
    unsafe {
        complete_register.write_volatile(id);
    }
}

/// Enable an interrupt based on its ID.
pub fn enable(id: u32) {
    let enables = sup_enable_base() as *const u32;
    let actual_id = 1 << id;
    unsafe {
        enables.write_volatile(enables.read_volatile() | actual_id);
    }
}
