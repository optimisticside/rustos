use core::arch::asm;

bitflags::bitflags! {
    pub struct Cr0: usize {
        /// Toggles paging.
        const ENABLE_PAGING = 1 << 31;
        /// Globally toggles the memory cache.
        const CACHE_DISABLE = 1 << 30;
        /// Globally toggles write-through caching.
        const NOT_WRITE_THROUGH = 1 << 29;
        /// Alignment checks are enabled if set and privilege level is 3.
        const ALIGNMENT_MASK = 1 << 18;
        /// CPU cannot write to read-only pages when privilege level is 0 and this is set.
        const WRITE_PROTECT = 1 << 16;
        /// If set, enable internal x87 fpu error reporting. Else, enable PC-style x87 error
        /// detection.
        const NUMERIC_ERROR = 1 << 5;
        /// On 386, specifies whether the external math coprocessor is a 80287 or 80387.
        const EXTENSION_TYPE = 1 << 4;
        /// Allows saving x87 task context upon a task switch only after a x87 instruction is used.
        const TASK_SWITCHED = 1 << 3;
        /// x87 fpu present if set.
        const EMULATE_COPROCESSOR = 1 << 2;
        /// Controls interaction of `wait`/`fwait` instructions.
        const MONITOR_COPROCESSOR = 1 << 1;
        /// System is in protected mode if set, otherwise it is in real mode.
        const PROTECTED_MODE = 1 << 0;
    }
}

bitflags::bitflags! {
    pub struct Cr4: usize {
        /// Enables use of protection keys.
        const PROTECTION_KEY = 1 << 22;
        /// Toggles supervisor mode access protection.
        const SMAP = 1 << 21;
        /// Toggles supervisor mode execution protection.
        const SMEP = 1 << 20;
        /// Toggles XSAVE and processor extended states.
        const XSAVE_PES = 1 << 18;
        /// Toggles process-context identifiers (PCIDs).
        const PCID = 1 << 17;
        /// Toggles the instructions RDFSBASE, RDGSBASE, WRFSBASE, and WRGSBASE.
        const FSG_SAVE = 1 << 16;
        /// Enables safer mode execution (trusted execution technology).
        const SMX = 1 << 14;
        /// Toggles virtual machine extensions.
        const VMX = 1 << 13;
        /// Enables 5-level paging.
        const L5_PAGING = 1 << 12;
        /// Toggles user-mode execution prevention (the SGDT, SIDT, SLDT, SMSW, and STR
        /// instructions cannot be executed if CPL > 0).
        const UMIP = 1 << 1;
        /// Toggles unmasked SSE exceptions.
        const UNMASKED_SSE = 1 << 10;
        /// Toggles streaming SIMD extension instructions and fast FPU save and restore FXSAVE and
        /// FXSTOR isntructions.
        const SSE = 1 << 9;
        /// Toggles performance-monitoring counters.
        const PPMC = 1 << 8;
        /// Toggles shared (PDE or PTE) address translation between address spaces.
        const GLOBAL_PAGES = 1 << 7;
        /// Toggles machine check interrupts.
        const MACHINE_CHECK = 1 << 6;
        /// Toggles physical address extension (to allow the addressing of physical memory larger
        /// than 4 GiB).
        const PAE = 1 << 5;
        /// Toggles page size extensions (to allow for pages larger than the traditional 4 KiB
        /// size). Note that if this is used, the size of large pages is reduced from 4 MiB down to
        /// 2 MiB.
        const PSE = 1 << 4;
        /// Toggles debug register based breaks on I/O space access.
        const DEBUG_EXTENSIONS = 1 << 3;
        /// Disables the ability to take time-stamps.
        const DISABLE_TIME_STAMP = 1 << 2;
        /// Toggles support for virtual interrupt flag in protected mode.
        const VIRTUAL_INTERRUPTS = 1 << 1;
        /// Toggles support for the virtual interrupt flag in virtual-8086 mode.
        const VME = 1 << 0;
    }
}

/// Read from the CR0 control-register.
pub fn cr0() -> Cr0 {
    let value: usize;
    unsafe {
        asm!("mov cr0, {0}", in(reg) value);
    }
    Cr0::from_bits_truncate(value)
}

/// Write to the CR0 control-register.
pub unsafe fn write_cr0(value: Cr0) {
    asm!("mov cr0, {0}", in(reg) value.bits());
}

/// Read from the CR3 control-register.
pub fn cr3() -> usize {
    let value: usize;
    unsafe {
        asm!("mov cr3, {0}", in(reg) value);
    }
    value
}

/// Write to the CR3 control-register.
pub unsafe fn write_cr3(value: usize) {
    asm!("mov cr3, {0}", in(reg) value);
}
