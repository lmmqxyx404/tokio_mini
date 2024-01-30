use crate::runtime::task::Header;
use std::task::{Poll, Waker};

use std::ptr::NonNull;

/// Raw task handle
#[derive(Clone)]
pub(crate) struct RawTask {
    ptr: NonNull<Header>,
}

pub(super) struct Vtable {
    /// Polls the future.
    pub(super) poll: unsafe fn(NonNull<Header>),

    /// Schedules the task for execution on the runtime.
    pub(super) schedule: unsafe fn(NonNull<Header>),

    /// Deallocates the memory.
    pub(super) dealloc: unsafe fn(NonNull<Header>),

    /// Reads the task output, if complete.
    pub(super) try_read_output: unsafe fn(NonNull<Header>, *mut (), &Waker),

    /// The join handle has been dropped.
    pub(super) drop_join_handle_slow: unsafe fn(NonNull<Header>),

    /// An abort handle has been dropped.
    pub(super) drop_abort_handle: unsafe fn(NonNull<Header>),

    /// Scheduler is being shutdown.
    pub(super) shutdown: unsafe fn(NonNull<Header>),

    /// The number of bytes that the `trailer` field is offset from the header.
    pub(super) trailer_offset: usize,

    /// The number of bytes that the `scheduler` field is offset from the header.
    pub(super) scheduler_offset: usize,

    /// The number of bytes that the `id` field is offset from the header.
    pub(super) id_offset: usize,
}
