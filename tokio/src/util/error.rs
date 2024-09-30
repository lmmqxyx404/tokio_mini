/// Error string explaining that the Tokio context is not available because the
/// thread-local storing it has been destroyed. This usually only happens during
/// destructors of other thread-locals.
pub(crate) const THREAD_LOCAL_DESTROYED_ERROR: &str =
    "The Tokio context thread-local variable has been destroyed.";
