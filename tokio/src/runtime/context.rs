use std::cell::Cell;

use runtime::EnterRuntime;

use crate::runtime::scheduler;

cfg_rt! {

mod current;
pub(crate) use current::{try_set_current, with_current, SetCurrentGuard};

mod runtime;
pub(crate) use runtime::enter_runtime;

mod blocking;
pub(crate) use blocking::BlockingRegionGuard;
}

struct Context {
    /// Handle to the runtime scheduler running on the current thread.
    #[cfg(feature = "rt")]
    current: current::HandleCell,

        /// Tracks if the current thread is currently driving a runtime.
    /// Note, that if this is set to "entered", the current scheduler
    /// handle may not reference the runtime currently executing. This
    /// is because other runtime handles may be set to current from
    /// within a runtime.
    #[cfg(feature = "rt")]
    runtime: Cell<EnterRuntime>,

}

tokio_thread_local! {
    static CONTEXT: Context = const { Context {
            // Tracks the current runtime handle to use when spawning,
            // accessing drivers, etc...
            #[cfg(feature = "rt")]
            current: current::HandleCell::new(),
            // Tracks if the current thread is currently driving a runtime.
            // Note, that if this is set to "entered", the current scheduler
            // handle may not reference the runtime currently executing. This
            // is because other runtime handles may be set to current from
            // within a runtime.
            #[cfg(feature = "rt")]
            runtime: Cell::new(EnterRuntime::NotEntered),

    } }
}
