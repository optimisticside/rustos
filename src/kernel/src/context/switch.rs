use crate::context::{Context, Status};
use crate::machine;

/// Determines if a context is able to be run by the specified CPU.
unsafe fn runnable(context; &Context, cpu_id: usize) -> bool {
    !context.running && context.status == Status::Runnable && context.cpu_id == Some(cpu_id)
}

/// Switch to the next context. By the time this function will return, the current context will
/// have been restored by someone else and a lot of time might have passed.
///
/// # Safety
/// Do not call this when holding locks.
pub unsafe fn switch() -> bool {

}
