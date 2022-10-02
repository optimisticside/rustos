/// A clock is the interface all timers must implement to interact with the kernel. A clock is
/// something that allows you to access the time.
pub trait Clock {
}

/// Interrupt clocks are clocks that let you calculate the time by firing interrupts (called ticks)
/// in a given frequency. The time can be calculated by the number of ticks.
pub trait InterruptClock {
    /// Retrieve the number of ticks per second.
    fn frequency(&self) -> usize,
    /// Register a clock handler, that will be called upon each tick interrupt to update the time
    /// of the clock.
    fn register_handler(&mut self, handler: fn()) -> bool;
}
