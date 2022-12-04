#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

mod device;
mod filesys;
mod io;
mod ipc;
mod machine;
mod memory;
mod sync;
mod unwind;
mod utils;

use self::unwind::*;
use utils::bootstrap::Bootstrap;

/// Architecture-independent kernel entry-point called by architecture-specific
/// code.
fn main(cpus: usize, bootstrap: Bootstrap) -> ! {
    loop {}
}
