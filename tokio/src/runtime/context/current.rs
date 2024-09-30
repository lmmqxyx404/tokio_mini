use crate::runtime::handle::TryCurrentError;
use crate::runtime::scheduler;
use crate::util::markers::SyncNotSend;

use std::marker::PhantomData;

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
    todo!()
    // CONTEXT.try_with(|ctx| ctx.set_current(handle)).ok()
}

pub(crate) fn with_current<F, R>(f: F) -> Result<R, TryCurrentError>
where
    F: FnOnce(&scheduler::Handle) -> R,
{
    todo!()
}
