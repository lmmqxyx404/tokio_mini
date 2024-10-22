use crate::loom::sync::Arc;
use crate::runtime::driver::{self, Driver};
use crate::runtime::{blocking, Config, WorkerMetrics};
use crate::util::rand::RngSeedGenerator;
use std::future::Future;
use std::{fmt, thread};

use crate::util::atomic_cell::AtomicCell;

use crate::runtime::scheduler::{self};

/// Handle to the current thread scheduler
pub(crate) struct Handle {
    /// Current random number generator seed
    pub(crate) seed_generator: RngSeedGenerator,
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("current_thread::Handle { ... }").finish()
    }
}

/// Executes tasks on the current thread
pub(crate) struct CurrentThread {
    /// Core scheduler data is acquired by a thread entering `block_on`.
    core: AtomicCell<Core>,
}

/// Used if none is specified. This is a temporary constant and will be removed
/// as we unify tuning logic between the multi-thread and current-thread
/// schedulers.
const DEFAULT_GLOBAL_QUEUE_INTERVAL: u32 = 31;

impl CurrentThread {
    pub(crate) fn new(
        driver: Driver,
        driver_handle: driver::Handle,
        blocking_spawner: blocking::Spawner,
        seed_generator: RngSeedGenerator,
        config: Config,
    ) -> (CurrentThread, Arc<Handle>) {
        let worker_metrics = WorkerMetrics::from_config(&config);
        worker_metrics.set_thread_id(thread::current().id());

        // Get the configured global queue interval, or use the default.
        let global_queue_interval = config
            .global_queue_interval
            .unwrap_or(DEFAULT_GLOBAL_QUEUE_INTERVAL);

        let handle = Arc::new(Handle { seed_generator });

        let core = AtomicCell::new(Some(Box::new(Core {})));

        let scheduler = CurrentThread {
            core,
            /*
            notify: Notify::new(), */
        };

        (scheduler, handle)
    }

    #[track_caller]
    pub(crate) fn block_on<F: Future>(&self, handle: &scheduler::Handle, future: F) -> F::Output {
        println!("BLOCK ON");

        pin!(future);

        crate::runtime::context::enter_runtime(handle, false, |blocking| {
            println!("real enter_runtime code");
            let handle = handle.as_current_thread();
            // Attempt to steal the scheduler core and block_on the future if we can
            // there, otherwise, lets select on a notification that the core is
            // available or the future is complete.
            loop {
                if let Some(core) = self.take_core(handle) {
                    todo!()
                } else {
                    todo!()
                }
            }
        })
    }

    fn take_core(&self, handle: &Arc<Handle>) -> Option<CoreGuard> {
        let core = self.core.take()?;

        todo!()
    }
}

impl fmt::Debug for CurrentThread {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("CurrentThread").finish()
    }
}

/// Data required for executing the scheduler. The struct is passed around to
/// a function that will perform the scheduling work and acts as a capability token.
struct Core {}

// ===== CoreGuard =====

/// Used to ensure we always place the `Core` value back into its slot in
/// `CurrentThread`, even if the future panics.
struct CoreGuard {}
