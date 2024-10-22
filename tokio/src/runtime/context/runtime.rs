use super::{BlockingRegionGuard, CONTEXT};

use crate::runtime::scheduler;

/// Marks the current thread as being within the dynamic extent of an
/// executor.
#[track_caller]
pub(crate) fn enter_runtime<F, R>(handle: &scheduler::Handle, allow_block_in_place: bool, f: F) -> R
where
    F: FnOnce(&mut BlockingRegionGuard) -> R,
{
    let maybe_guard: Option<EnterRuntimeGuard> = CONTEXT.with(|c| {
        if c.runtime.get().is_entered() {
            todo!()
        } else {
            // Set the entered flag
            c.runtime.set(EnterRuntime::Entered {
                allow_block_in_place,
            });

            todo!()

            /* // Generate a new seed
            let rng_seed = handle.seed_generator().next_seed();

            // Swap the RNG seed
            let mut rng = c.rng.get().unwrap_or_else(FastRand::new);
            let old_seed = rng.replace_seed(rng_seed);
            c.rng.set(Some(rng));

            Some(EnterRuntimeGuard {
                blocking: BlockingRegionGuard::new(),
                handle: c.set_current(handle),
                old_seed,
            }) */
        }
    });

    todo!()
}

#[derive(Debug, Clone, Copy)]
#[must_use]
pub(crate) enum EnterRuntime {
    /// Currently in a runtime context.
    #[cfg_attr(not(feature = "rt"), allow(dead_code))]
    Entered { allow_block_in_place: bool },

    /// Not in a runtime context **or** a blocking region.
    NotEntered,
}

impl EnterRuntime {
    pub(crate) fn is_entered(self) -> bool {
        matches!(self, EnterRuntime::Entered { .. })
    }
}

/// Guard tracking that a caller has entered a runtime context.
#[must_use]
pub(crate) struct EnterRuntimeGuard {
    /// Tracks that the current thread has entered a blocking function call.
    pub(crate) blocking: BlockingRegionGuard,
}
