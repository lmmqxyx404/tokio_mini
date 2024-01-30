use crate::runtime::{scheduler};

/// Handle to the runtime.
///
/// The handle is internally reference-counted and can be freely cloned. A handle can be
/// obtained using the [`Runtime::handle`] method.
///
/// [`Runtime::handle`]: crate::runtime::Runtime::handle()
#[derive(Debug, Clone)]
// When the `rt` feature is *not* enabled, this type is still defined, but not
// included in the public API.
pub struct Handle {
    pub(crate) inner: scheduler::Handle,
}