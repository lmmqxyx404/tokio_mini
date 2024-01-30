use crate::loom::sync::atomic::AtomicUsize;

pub(super) struct State {
    val: AtomicUsize,
}
