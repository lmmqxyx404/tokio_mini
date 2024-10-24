use std::future::Future;

use crate::runtime::blocking::BlockingPool;
use crate::runtime::scheduler::CurrentThread;
use crate::runtime::Handle;

use super::handle::EnterGuard;
use super::BOX_FUTURE_THRESHOLD;

#[derive(Debug)]
pub struct Runtime {
    /// Handle to runtime, also contains driver handles
    handle: Handle,
    /// Task scheduler
    scheduler: Scheduler,
}

impl Runtime {
    #[track_caller]
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        if std::mem::size_of::<F>() > BOX_FUTURE_THRESHOLD {
            todo!()
        } else {
            println!("START BLOCK ON ELSE BRANCH");
            self.block_on_inner(future)
        }
    }

    pub(super) fn from_parts(
        scheduler: Scheduler,
        handle: Handle,
        blocking_pool: BlockingPool,
    ) -> Runtime {
        Runtime {
            handle,
            scheduler,
            /*
            blocking_pool, */
        }
    }

    #[track_caller]
    fn block_on_inner<F: Future>(&self, future: F) -> F::Output {
        let _enter = self.enter();

        match &self.scheduler {
            Scheduler::CurrentThread(exec) => exec.block_on(&self.handle.inner, future),
        }
    }

    pub fn enter(&self) -> EnterGuard<'_> {
        println!("Runtime: enter");
        self.handle.enter()
    }
}

/// The runtime scheduler is either a multi-thread or a current-thread executor.
#[derive(Debug)]
pub(super) enum Scheduler {
    /// Execute all tasks on the current-thread.
    CurrentThread(CurrentThread),
}
