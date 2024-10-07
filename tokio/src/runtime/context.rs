use crate::runtime::scheduler;

cfg_rt! {
  mod current;
  pub(crate) use current::{try_set_current, with_current, SetCurrentGuard};
}

struct Context {
    /// Handle to the runtime scheduler running on the current thread.
    #[cfg(feature = "rt")]
    current: current::HandleCell,
}

tokio_thread_local! {
    static CONTEXT: Context = const { Context {
            // Tracks the current runtime handle to use when spawning,
            // accessing drivers, etc...
            #[cfg(feature = "rt")]
            current: current::HandleCell::new(),
    } }
}

