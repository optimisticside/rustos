/// Platform-Level Interrupt Controller.
///
/// The PLIC is shared by all HARTs and allows for sending and recieving prioritized interruupts
/// through claims.
pub struct Plic {
    base: *mut u64,
    priority: *mut u64,
    mem_enable_base: *mut u64,

}

impl Plic {
    const fn new(addr: usize) -> Self {
        Self {
            base: addr as *mut u64,
            priority: (addr + 0x1000) as *mut u64,
            menable_base: (addr + 0x2000) as *mut u64,
            senable_base: (addr + 0x2080) as *mut u64,
            mpriority_base: (addr + 0x200000) as *mut u64,
            spriority_base: (addr + 0x201000) as *mut u64,
            mclaim_base: (addr + 0x200004) as *mut u64,
            sclaim_base: (addr + 0x201004) as *mut u64,
        }
    }

    /// Retrieve the next available interrupt. This is by a "claim" process, where the PLIC will
    /// give us the ID of the highest-priority interrupt after sorting them.
    pub fn next(&mut self) -> Option<u32> {
        let claim_register = ();
        let claim_num = unsafe { claim_register.read_volatile() };
    }
}
