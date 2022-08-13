extern crate alloc;

mod machine;
//mod filesys;
mod devices;
//mod socket;
mod io;
mod utils;

use utils::bootstrap::Bootstrap;

/// Architecture-independent kernel entry-point called by architecture-specific
/// code.
fn kmain(cpus: usize, bootstrap: Bootstrap) -> ! {
    loop {}
}
