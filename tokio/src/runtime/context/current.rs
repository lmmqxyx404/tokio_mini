use crate::runtime::handle::TryCurrentError;
use crate::runtime::scheduler;
use crate::util::markers::SyncNotSend;

use std::marker::PhantomData;

use super::{Context, CONTEXT};

use std::cell::{Cell, RefCell};

#[derive(Debug)]
#[must_use]
pub(crate) struct SetCurrentGuard {
    // The previous handle
    prev: Option<scheduler::Handle>,

    // The depth for this guard
    depth: usize,

    // Don't let the type move across threads.
    _p: PhantomData<SyncNotSend>,
}

/// Sets this [`Handle`] as the current active [`Handle`].
///
/// [`Handle`]: crate::runtime::scheduler::Handle
pub(crate) fn try_set_current(handle: &scheduler::Handle) -> Option<SetCurrentGuard> {
    CONTEXT.try_with(|ctx| ctx.set_current(handle)).ok()
}

pub(crate) fn with_current<F, R>(f: F) -> Result<R, TryCurrentError>
where
    F: FnOnce(&scheduler::Handle) -> R,
{
    todo!()
}

pub(super) struct HandleCell {
    /// Current handle
    handle: RefCell<Option<scheduler::Handle>>,
    /* /// Tracks the number of nested calls to `try_set_current`.
    depth: Cell<usize>, */
}

impl HandleCell {
    pub(super) const fn new() -> HandleCell {
        HandleCell {
            handle: RefCell::new(None),
            /* depth: Cell::new(0), */
        }
    }
}

impl Context {
    pub(super) fn set_current(&self, handle: &scheduler::Handle) -> SetCurrentGuard {
        let old_handle = self.current.handle.borrow_mut().replace(handle.clone());

        todo!()
    }
}
