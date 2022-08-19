use core::arch::asm;

pub use self::start::*;

pub mod idt;
pub mod gdt;
pub mod apic;
pub mod ctrlregs;
pub mod dtables;
pub mod fence;
//pub mod io;
pub mod irq;
pub mod msr;
pub mod segmentation;
//pub mod task;
pub mod time;
pub mod tlb;
pub mod start;

/// x86 Protection levels
///
/// # Note
/// This should not contain values larger than 2 bits, otherwise
/// segment descriptor code needs to be adjusted accordingly.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Ring {
    Ring0 = 0b00,
    Ring1 = 0b01,
    Ring2 = 0b10,
    Ring3 = 0b11,
}

/// Stops instruction execution and places processor in a HALT state.
///
/// An enabled interrupt (including NMI and SMI), a debug exception, the BINIT#
/// signal, the INIT# signal, or the RESET# signal will resume execution. If an
/// interrupt (including NMI) is used to resume execution after a HLT instruction,
/// the saved instruction pointer (CS:EIP) points to the instruction following
/// the HLT instruction.
///
/// # Safety
/// Will cause a general protection fault if used outside of ring 0.
#[inline(always)]
pub unsafe fn halt() {
    asm!("hlt", options(nomem, nostack));
}
