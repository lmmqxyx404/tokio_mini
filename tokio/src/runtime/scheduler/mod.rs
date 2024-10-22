cfg_rt! {
    pub(crate) mod current_thread;
    pub(crate) use current_thread::CurrentThread;
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
    use crate::runtime::{blocking};
    use crate::util::RngSeedGenerator;

    macro_rules! match_flavor {
        ($self:expr, $ty:ident($h:ident) => $e:expr) => {
            match $self {
                $ty::CurrentThread($h) => $e,

                #[cfg(feature = "rt-multi-thread")]
                $ty::MultiThread($h) => $e,

                #[cfg(all(tokio_unstable, feature = "rt-multi-thread"))]
                $ty::MultiThreadAlt($h) => $e,
            }
        }
    }

    impl Handle {
        #[track_caller]
        pub(crate) fn current() -> Handle {
            match context::with_current(Clone::clone) {
                Ok(handle) => handle,
                Err(e) => panic!("{}", e),
            }
        }

        pub(crate) fn blocking_spawner(&self) -> &blocking::Spawner {
            todo!() // match_flavor!(self, Handle(h) => &h.blocking_spawner)
        }
        /// used for `enter_runtime`
        pub(crate) fn seed_generator(&self) -> &RngSeedGenerator {
            match_flavor!(self, Handle(h) => &h.seed_generator)
        }

        pub(crate) fn as_current_thread(&self) -> &Arc<current_thread::Handle> {
            match self {
                Handle::CurrentThread(handle) => handle,
                #[cfg(feature = "rt-multi-thread")]
                _ => panic!("not a CurrentThread handle"),
            }
        }
    }
}
