mod machine;
//mod filesys;
mod devices;
//mod socket;
//mod utils;
mod io;

use utils::bootstrap::Bootstrap;

/// Architecture-independent kernel entry-point called by architecture-specific
/// code.
fn main(cpus: usize, bootstrap: Bootstrap) -> ! {
    
}
