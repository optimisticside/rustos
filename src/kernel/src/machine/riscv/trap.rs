use crate::machine::{self, plic, hart};

/// Each interrupt handler is provided the interrupt ID of the interrupt, and must return whether
/// the interrupt was processed (which is used to notify the PLIC).
pub type InterruptHandler = fn(u8) -> bool;

/// Interrupt stub that is called when a surpious interrupt is called.
pub fn surpious(interrupt: u8) -> bool {
    println!("Surpious interrupt {}", interrupt);
    true
}

/// Array of functions that are mapped to interrupt IDs. This is not a hash-map because it has
/// numeric indices and there only a maximum of 255 interrupt IDs (so space is not an issue).
static mut INTERRUPT_HANDLERS: [InterruptHandler; 255] = [surpious; 255];

/// Main interrupt handling routine. Called by assembly-routine in between saving and restoring
/// context. Responsible for calling any registered interrupt-routines.
#[no_mangle]
pub extern "C" fn trap() {
    if let Some(interrupt) = plic::next() {
        if INTERRUPT_HANDLERS[interrupt](interrupt) {
            plic::complete(interrupt);
        }
    }
}
