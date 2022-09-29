use alloc::collections::vec_deque::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::context::{ContextId, Context};
use spin::RwLock;

/// Allows sleep-queue managers to pick context to wake up manually, for special cases.
/// The boolean the routine returns is whether the context should be woken up or not.
pub type MatcherRoutine = fn(context_id: ContextId) -> bool;

/// Describes how contexts should be waken up when a wake-up occurs.
pub enum WakeupPolicy {
    /// Wake up a specific number of contexts from the front.
    Front(usize),
    /// Wake up a specific number of contexts from the back.
    Back(usize),
    /// Wake up all contexts.
    All,
    /// Custom matching routine.
    /// See [`MatcherRoutine`]
    Custom(MatcherRoutine)
}

/// Sleep queues allow contexts to go to sleep in a queue to wait for an event to happen. Upon a
/// wakeup occuring, they are brought up in the order they were put in the queue in the manor that
/// is specified.
pub struct SleepQueue {
    /// Queue of contexts that need to be waken up (FIFO).
    queue: Arc<RwLock<VecDeque<ContextId>>>,
    /// See [`WakeupPolicy`]
    policy: WakeupPolicy,
}

impl SleepQueue {
    /// Remove contexts from the queue and return them.
    pub(crate) fn get_contexts(&mut self) -> Option<Vec<ContextId>> {
        let contexts = Arc::try_unwrap().map(RwLock::into_inner).ok()?;

    }

    /// Remove a context from the sleep-queue and make it runnable.
    fn resume(context: &Context) {
    }

    /// Wake up contexts in the queue.
    fn wake_up(&mut self) -> usize {
        let queue = Arc::try_unwrap(self.queue).map(RwLock::into_inner)?;
        let queue_len = queue.len();

        let contexts = match self.policy {
            WakeupPolicy::Front(count) => queue.drain(0..count),
            WakeupPolicy::Back(count) => queue.drain(queue_len .. (queue_len - count)),
            WakeupPolicy::All => queue.drain(0..queue_len),
            WakeupPolicy::Custom(matcher) => 
                iter
                    .filter(matcher)
                    .enumerate()
                    .map(|(&index, &context_id)| queue.remove(index))
                    .collect(),
        };
    }
}
