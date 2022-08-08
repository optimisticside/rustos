/// Passed to the kernel entry-point.
#[repr(packed)]
pub struct KernelArgs {
    kernel_base: u64,
    kernel_size: u64,
    stack_base: u64,
    stack_size: u64,
}

/// Kernel entry-point for x86_64. Everything that is architecture-specific
/// must be initialized here, before calling architecutre-independent kernel
/// code.
#[no_mangle]
pub extern unsafe fn start(args_ptr: *const KernelArgs) -> ! {
    let args = args_ptr.read();

    // Set up GDT and IDT before initializing paging.
    gdt::init();
    idt::init();
}
