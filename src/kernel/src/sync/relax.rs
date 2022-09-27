use sync::RelaxStrategy;

/// A strategy that yields the current time slice to the scheduler in favour of other threads or
/// processes.
///
/// This is generally used as a strategy for minimising power consumption and for events that take
/// a long time such as I/O or messages from other processes.
pub struct Yield;

impl RelaxStrategy for Yield {
    #[inline(always)]
    fn relax() {
        // TODO: Implement this.
    }
}
