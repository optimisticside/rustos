use crate::device::serial::uart_16550::SerialPort;
use crate::io::PortIo;
use crate::machine::{gdt, idt};

/// Passed to the kernel entry-point. Same format as the bootloader for Redux OS.
#[repr(packed)]
pub struct KernelArgs {
    kernel_base: u64,
    kernel_size: u64,
    stack_base: u64,
    stack_size: u64,
    env_base: u64,
    env_size: u64,

    /// The base 64-bit pointer to an array of saved RSDPs. It's up to the kernel (and possibly
    /// userspace), to decide which RSDP to use. The buffer will be a linked list containing a
    /// 32-bit relative (to this field) next, and the actual struct afterwards.
    ///
    /// This field can be NULL, and if so, the system has not booted with UEFI or in some other way
    /// retrieved the RSDPs. The kernel or a userspace driver will thus try searching the BIOS
    /// memory instead. On UEFI systems, BIOS-like searching is not guaranteed to actually work though.
    acpi_rsdps_base: u64,
    /// The size of the RSDPs region.
    acpi_rsdps_size: u64,

    areas_base: u64,
    areas_size: u64,

    /// The physical base 64-bit pointer to the contiguous bootstrap/initfs.
    bootstrap_base: u64,
    /// Size of contiguous bootstrap/initfs physical region, not necessarily page aligned.
    bootstrap_size: u64,
    /// Entry point the kernel will jump to.
    bootstrap_entry: u64,
}

/// Kernel entry-point for x86_64. Everything that is architecture-specific must be initialized
/// here, before calling architecutre-independent kernel code.
#[no_mangle]
pub unsafe extern "C" fn _start(/*args_ptr: *const KernelArgs*/) -> ! {
    // let args = args_ptr.read();

    // Set up GDT and IDT before initializing paging.
    // gdt::init();
    // idt::init();

    // Set up serial communication.
    let mut serial_port = SerialPort::<PortIo<u8>>::new(0x3F8);
    for character in "Hello world".as_bytes().iter() {
        serial_port.put_char(*character);
    }

    // TODO: this is temporary.
    loop {}
    // crate::kmain(1, bootstrap)
}
