use crate::loom::cell::UnsafeCell;
use crate::loom::sync::Arc;

use std::fmt;

#[derive(Debug)]
pub struct Sender<T> {
    inner: Option<Arc<Inner<T>>>,
}

struct Inner<T> {
    /// The value. This is set by `Sender` and read by `Receiver`. The state of
    /// the cell is tracked by `state`.
    value: UnsafeCell<Option<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Inner<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::sync::atomic::Ordering::Relaxed;

        todo!()
    }
}

#[derive(Debug)]
pub struct Receiver<T> {
    inner: Option<Arc<Inner<T>>>,
}

#[track_caller]
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(Inner {
        // state: AtomicUsize::new(State::new().as_usize()),
        value: UnsafeCell::new(None),
        // tx_task: Task(UnsafeCell::new(MaybeUninit::uninit())),
        // rx_task: Task(UnsafeCell::new(MaybeUninit::uninit())),
    });

    let tx = Sender {
        inner: Some(inner.clone()),
    };

    let rx = Receiver { inner: Some(inner) };

    (tx, rx)
}
