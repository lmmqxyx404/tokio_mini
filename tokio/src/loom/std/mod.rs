mod atomic_usize;
mod unsafe_cell;

/// 1.std::sync
pub(crate) mod sync {
    pub(crate) use std::sync::Arc;

    pub(crate) mod atomic {
        pub(crate) use crate::loom::std::atomic_usize::AtomicUsize;
    }
}

/// 2.std::cell
pub(crate) mod cell {
    pub(crate) use super::unsafe_cell::UnsafeCell;
}
