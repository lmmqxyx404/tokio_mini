use std::future::Future;

use crate::runtime::blocking::BlockingPool;
use crate::runtime::scheduler::CurrentThread;
use crate::runtime::Handle;

use super::BOX_FUTURE_THRESHOLD;

#[derive(Debug)]
pub struct Runtime {}

impl Runtime {
    #[track_caller]
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        if std::mem::size_of::<F>() > BOX_FUTURE_THRESHOLD {
            todo!()
        } else {
            println!("START BLOCK ON ELSE BRANCH");
            todo!()
        }
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
