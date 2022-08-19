/// Internal data-structure to store the result of the CPUID instruction.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct CpuidResult {
    /// Value stored in the EAX register.
    eax: u32,
    /// Value stored in the EBX register.
    ebx: u32,
    /// Value stored in the ECX register.
    ecx: u32,
    /// Value stored in the EDX register.
    edx: u32,
}
