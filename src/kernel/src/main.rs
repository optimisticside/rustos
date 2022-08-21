#![no_std]
#![no_main]

extern crate alloc;

mod machine;
//mod filesys;
mod devices;
//mod socket;
mod io;
mod utils;

use core::panic::PanicInfo;
use utils::bootstrap::Bootstrap;

/// Panic routine that loops forever.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Architecture-independent kernel entry-point called by architecture-specific
/// code.
fn main(cpus: usize, bootstrap: Bootstrap) -> ! {
    loop {}
}
