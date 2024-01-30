use crate::loom::cell::UnsafeCell;
use crate::runtime::task::raw::Vtable;
use crate::runtime::task::state::State;

use std::num::NonZeroU64;
use std::ptr::NonNull;

/// Crate public as this is also needed by the pool.
#[repr(C)]
pub(crate) struct Header {
    /// Task state. todo 19:14
    pub(super) state: State,

    /// Pointer to next task, used with the injection queue.
    pub(super) queue_next: UnsafeCell<Option<NonNull<Header>>>,

    /// Table of function pointers for executing actions on the task.
    pub(super) vtable: &'static Vtable,

    /// This integer contains the id of the `OwnedTasks` or `LocalOwnedTasks`
    /// that this task is stored in. If the task is not in any list, should be
    /// the id of the list that it was previously in, or `None` if it has never
    /// been in any list.
    ///
    /// Once a task has been bound to a list, it can never be bound to another
    /// list, even if removed from the first list.
    ///
    /// The id is not unset when removed from a list because we want to be able
    /// to read the id without synchronization, even if it is concurrently being
    /// removed from the list.
    pub(super) owner_id: UnsafeCell<Option<NonZeroU64>>,

    /// The tracing ID for this instrumented task.
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(super) tracing_id: Option<tracing::Id>,
}
