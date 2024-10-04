use std::future::Future;

use crate::runtime::blocking::BlockingPool;
use crate::runtime::scheduler::CurrentThread;
use crate::runtime::Handle;

#[derive(Debug)]
pub struct Runtime {}

impl Runtime {
    #[track_caller]
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        todo!()
    }

    pub(super) fn from_parts(
        scheduler: Scheduler,
        handle: Handle,
        blocking_pool: BlockingPool,
    ) -> Runtime {
        Runtime {
            /*  scheduler,
            handle,
            blocking_pool, */
        }
    }
}

/// The runtime scheduler is either a multi-thread or a current-thread executor.
#[derive(Debug)]
pub(super) enum Scheduler {
    /// Execute all tasks on the current-thread.
    CurrentThread(CurrentThread),
}
