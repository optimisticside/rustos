use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;

use core::cmp::Ordering;

use crate::filesys::{Vnode, FileDescriptor};
use crate::Error;

use crate::machine;
use crate::machine::context::{Context as MachineContext};
use crate::machine::interrupt::InterruptStack;

use spin::RwLock;

/// Status of context. Used for scheduling.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    /// On the run queue.
    Runnable,
    /// Blocked from running. This is due to the context going to sleep to wait for I/O to be
    /// complete.
    Blocked,
    /// Process is stopped (because of the provided signal number).
    Stopped(usize),
    /// The context has executed (with provided exit-code).
    Exited(usize),
}

/// A context describes the executions state of a process or thread (or any execution image that
/// can be loaded and saved on the CPU).
pub struct Context {
    /// Unique identifier.
    pub id: ContextId,
    /// Group ID.
    pub group_id: ContextId,
    /// ID of the parent context.
    pub parent_id: ContextId,
    /// Real user ID.
    pub real_user_id: u32,
    /// Real group ID.
    pub real_group_id: u32,
    /// Effective user ID.
    pub effective_user_id: u32,
    /// Effective group ID.
    pub effective_group_id: u32,
    /// Signal mask (what signals it can accept).
    pub signal_mask: [u64; 2],
    /// Status of the context.
    pub status: Status,
    pub status_reason: &'static str,
    /// Whether the context is running or not.
    pub running: bool,
    /// The CPU that the context is running on (if locked).
    pub cpu: Option<usize>,
    /// Number of timer ticks that have passed while the process was executing (to measure how long
    /// the process has been on the CPU for).
    pub ticks: usize,
    /// Context will wake up at this time.
    pub wakeup_time: Option<(usize, usize)>,
    /// Pending signals in the order that they will be handled.
    pub pending: VecDeque<u8>,
    /// Machine-specific data of the context (not including registers).
    pub machine: MachineContext,
    /// Kernel stack.
    pub kernel_stack: Option<Box<[u8]>>,
    /// Kernel FX. Used to store SIMD and FPU registers.
    pub kernel_fx: AlignedBox<[u8; machine::KERNFX_SIZE, {machine::KERNFX_ALIGN}]>,
    /// Address space containing a page table lock, and grants. Normally this will have a value,
    /// but it can be None while the context is being reaped or when a new context is created but
    /// has not yet had its address space changed. Note that these are only for user mappings, as
    /// kernel mappings are universal and independent on address spaces or contexts.
    pub addr_space: Option<Arc<RwLock<AddressSpace>>>,
    /// Name of this context (used mainly for debugging purposes).
    pub name: Arc<RwLock<Box<str>>>,
    /// Current working directory. Acts as a marker for where the process is currently in the
    /// file-system. Used for relative paths.
    pub current_dir: Arc<RwLock<Vnode>>,
    /// Open file-descriptors.
    pub files: Arc<RwLock<Vec<Option<FileDescriptor>>>>,
    /// Pointer to user-space registers, saved after certain interrupts.
    pub registers: Option<(usize, Unique<InterruptStack>)>
    /// Signal action handlers.
    pub signal_actions: Arc<RwLock<Vec<(SigAction, usize)>>>,
}

impl Context {
    /// Construct a new [`Context`].
    pub const fn new(id: ContextId) -> Self {
    }

    /// Retrieve the context's address space.
    pub fn addr_space(&self) -> Result<&Arc<RwLock<AddrSpace>>> {
        self.addr_space.as_ref().ok_or(Error)
    }
}
