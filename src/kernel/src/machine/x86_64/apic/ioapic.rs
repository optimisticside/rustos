/// I/O Advanced Programmable Interrupt Controller.
///
/// Used to distribute external interrupts in a more advanced way than the Programmable Interrupt
/// Controller (PIC) Interrupts can be distributed to physical or logical (clusters) of processors
/// and can be prioritized.
pub struct IoApic {
    register: *mut u32,
    data: *mut u32,
}

impl IoApic {
    /// Instantiates a new IoApic at the given addres.
    pub unsafe fn new(addr: usize) -> Self {
        IoApic {
            register: addr as *mut u32,
            data: (addr + 0x10) as *mut u32,
        }
    }

    /// Disables all interrupts.
    pub fn disable_all(&mut self) {
        /// Marks all interrupts as edge-triggered, active high, disabled, and not routed to any
        /// CPU.
    }
}
