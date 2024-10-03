use crate::loom::sync::Arc;
use crate::runtime::driver::{self, Driver};
use crate::runtime::{blocking, Config, WorkerMetrics};
use crate::util::rand::RngSeedGenerator;
use std::fmt;

/// Handle to the current thread scheduler
pub(crate) struct Handle {}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("current_thread::Handle { ... }").finish()
    }
}

/// Executes tasks on the current thread
pub(crate) struct CurrentThread {}

impl CurrentThread {
    pub(crate) fn new(
        driver: Driver,
        driver_handle: driver::Handle,
        blocking_spawner: blocking::Spawner,
        seed_generator: RngSeedGenerator,
        config: Config,
    ) -> (CurrentThread, Arc<Handle>) {
        let worker_metrics = WorkerMetrics::from_config(&config);

        todo!()
    }
}
