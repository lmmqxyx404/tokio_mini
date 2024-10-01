use crate::loom::sync::Arc;

#[derive(Debug)]
pub(crate) struct ParkThread {
    inner: Arc<Inner>,
}

// ==== impl ParkThread ====

impl ParkThread {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                /* state: AtomicUsize::new(EMPTY),
                mutex: Mutex::new(()),
                condvar: Condvar::new(), */
            }),
        }
    }

    pub(crate) fn unpark(&self) -> UnparkThread {
        let inner = self.inner.clone();
        UnparkThread { inner }
    }
}

#[derive(Debug)]
struct Inner {}

/// Unblocks a thread that was blocked by `ParkThread`.
#[derive(Clone, Debug)]
pub(crate) struct UnparkThread {
    inner: Arc<Inner>,
}