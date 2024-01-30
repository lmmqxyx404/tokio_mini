mod atomic_usize;

pub(crate) mod sync {
    pub(crate) mod atomic {
        pub(crate) use crate::loom::std::atomic_usize::AtomicUsize;
    }
}
