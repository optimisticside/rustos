use alloc::collections::vec_deque::VecDeque;
use crate::context::{ContextId, Context};

/// Describes how contexts should be waken up when a wake-up occurs.
pub enum WakeupPolicy {
    /// Wake up a specific number of contexts from the front.
    Front(usize),
    /// Wake up a specific number of contexts from the back.
    Back(usize),
    /// Wake up all contexts.
    All,
}

/// Sleep queues allow contexts to go to sleep in a queue to wait for an event to happen. Upon a
/// wakeup occuring, they are brought up in the order they were put in the queue in the manor that
/// is specified.
pub struct SleepQueue {
    /// Queue of contexts that need to be waken up (FIFO).
    queue: VecDeque<ContextId>,
    /// See [`WakeupPolicy`]
    policy: WakeupPolicy,
}
