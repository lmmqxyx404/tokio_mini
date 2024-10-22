use std::cell::Cell;

use runtime::EnterRuntime;

#[cfg(any(feature = "rt", feature = "macros", feature = "time"))]
use crate::util::rand::FastRand;

cfg_rt! {

mod current;
pub(crate) use current::{try_set_current, with_current, SetCurrentGuard};

mod runtime;
pub(crate) use runtime::enter_runtime;

mod blocking;
pub(crate) use blocking::BlockingRegionGuard;

use crate::runtime::{scheduler};

mod scoped;
use scoped::Scoped;
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
    #[cfg(any(feature = "rt", feature = "macros", feature = "time"))]
    rng: Cell<Option<FastRand>>,

    /// Handle to the scheduler's internal "context"
    #[cfg(feature = "rt")]
    scheduler: Scoped<scheduler::Context>,
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
            #[cfg(any(feature = "rt", feature = "macros", feature = "time"))]
            rng: Cell::new(None),

            // Tracks the current scheduler internal context
            #[cfg(feature = "rt")]
            scheduler: Scoped::new(),
    } }
}

cfg_rt! {
    pub(super) fn set_scheduler<R>(v: &scheduler::Context, f: impl FnOnce() -> R) -> R {
        CONTEXT.with(|c| c.scheduler.set(v, f))
    }
}
