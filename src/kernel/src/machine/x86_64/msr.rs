use core::arch::asm;

/// Write 64 bits to a model-specific register.
pub unsafe fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
}

/// Read 64 bits from a model-specific register.
pub unsafe fn rdmsr(msr: u32) -> u64 {
    let (high, low): (u32, u32);
    asm!("rdmsr", out("eax") low, out("edx") high, in("ecx") msr);
    ((high as u64) << 32) | (low as u64)
}
