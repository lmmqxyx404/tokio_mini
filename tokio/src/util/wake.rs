use crate::loom::sync::Arc;

/// Simplified waking interface based on Arcs.
pub(crate) trait Wake: Send + Sync + Sized + 'static {}

/// A `Waker` that is only valid for a given lifetime.
#[derive(Debug)]
pub(crate) struct WakerRef {}

/// Creates a reference to a `Waker` from a reference to `Arc<impl Wake>`.
pub(crate) fn waker_ref<W: Wake>(wake: &Arc<W>) -> WakerRef {
    todo!()
}
