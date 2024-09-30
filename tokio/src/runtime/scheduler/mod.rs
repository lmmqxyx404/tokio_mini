cfg_rt! {
    pub(crate) mod current_thread;
    // pub(crate) use current_thread::CurrentThread;
}

use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum Handle {
    // todo: change
    #[cfg(feature = "rt")]
    CurrentThread(Arc<current_thread::Handle>),

    #[cfg(all(feature = "rt-multi-thread", not(target_os = "wasi")))]
    MultiThread(Arc<multi_thread::Handle>),

    #[cfg(all(tokio_unstable, feature = "rt-multi-thread", not(target_os = "wasi")))]
    MultiThreadAlt(Arc<multi_thread_alt::Handle>),

    // TODO: This is to avoid triggering "dead code" warnings many other places
    // in the codebase. Remove this during a later cleanup
    #[cfg(not(feature = "rt"))]
    #[allow(dead_code)]
    Disabled,
}

cfg_rt! {
    use crate::runtime::context;

    impl Handle {
        #[track_caller]
        pub(crate) fn current() -> Handle {
            match context::with_current(Clone::clone) {
                Ok(handle) => handle,
                Err(e) => panic!("{}", e),
            }
        }
    }
}
