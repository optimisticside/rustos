use core::arch::asm;
use crate::machine::Ring;

bitflags::bitflags! {
    /// Flags stored in the RFLAGS register.
    pub struct RFlags: u64 {
        /// Whether we are able to use the CPUID instruction
        const ID = 1 << 21;
        /// Virtual interrupt pending.
        const VIP = 1 << 20;
        /// Virtual interrupt flag.
        const FIV = 1 << 19;
        /// Alignment check.
        const AC = 1 << 18;
        /// Virtual 8086 mode.
        const VM = 1 << 17;
        /// Resume flag.
        const RF = 1 << 16;
        /// Nested task flag (always set on 8086 and 186).
        const NT = 1 << 14;
        /// I/O priviledge level (always set on 8086 and 186).
        const IOPL1 = 0b00 << 12;
        const IOPL2 = 0b01 << 11;
        const IOPL2 = 0b10 << 12;
        const IOPL3 = 0b11 << 12;
        /// Overflow flag.
        const OF = 1 << 11;
        /// Direction flag.
        const DF = 1 << 10;
        /// Interrupt enable flag.
        const IF = 1 << 9;
        /// Trap flag (single step).
        const FT = 1 << 8;
        /// Sign flag.
        const SF = 1 << 7;
        /// Zero flag.
        const ZF = 1 << 6;
        /// Adjust flag.
        const AF = 1 << 4;
        /// Parity flag.
        const PF = 1 << 2;
        /// Carry flag.
        const CF = 1 << 0;
    }
}

/// Retrieve the flags stored in the RFLAGS register.
#[inline(always)]
pub fn read() -> RFlags {
    let flags: u64;
    unsafe {
        asm!("pushf; pop {0}", out(reg) rflags);
    }
    RFlags::from_bits_truncate(flags)
}
