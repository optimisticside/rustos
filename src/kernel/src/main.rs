mod machine;
mod filesys;
mod drivers;
mod process;
mod socket;
mod utils;

use utils::bootstrap::Bootstrap;

/// Architecture-independent kernel entry-point called by architecture-specific
/// code.
fn main(cpus: usize, bootstrap: Bootstrap) -> ! {
    
}
