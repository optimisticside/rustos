#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

mod machine;
mod filesys;
mod devices;
mod socket;
mod sync;
mod io;
mod utils;
mod unwind;
mod memory;

use utils::bootstrap::Bootstrap;
use self::unwind::*;

/// Architecture-independent kernel entry-point called by architecture-specific
/// code.
fn main(cpus: usize, bootstrap: Bootstrap) -> ! {
    loop {}
}
