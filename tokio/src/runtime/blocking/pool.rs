use crate::loom::sync::Arc;
use crate::runtime::blocking::shutdown;
use crate::runtime::task::JoinHandle;
use crate::runtime::{Builder, Handle};

use std::time::Duration;

/// Runs the provided function on an executor dedicated to blocking operations.
/// Tasks will be scheduled as non-mandatory, meaning they may not get executed
/// in case of runtime shutdown.
#[track_caller]
#[cfg_attr(target_os = "wasi", allow(dead_code))]
pub(crate) fn spawn_blocking<F, R>(func: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    // todo:1 handle is the start then is relative fn.
    let rt = Handle::current();
    rt.spawn_blocking(func)
}

#[derive(Clone)]
pub(crate) struct Spawner {
    inner: Arc<Inner>,
}

// ===== impl Spawner =====

impl Spawner {
    #[track_caller]
    pub(crate) fn spawn_blocking<F, R>(&self, rt: &Handle, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        todo!()
    }
}

struct Inner {}

pub(crate) struct BlockingPool {
    spawner: Spawner,
}

// ===== impl BlockingPool =====

impl BlockingPool {
    pub(crate) fn new(builder: &Builder, thread_cap: usize) -> BlockingPool {
        let (shutdown_tx, shutdown_rx) = shutdown::channel();
        let keep_alive = builder.keep_alive.unwrap_or(KEEP_ALIVE);

        BlockingPool {
            spawner: Spawner {
                inner: Arc::new(Inner {}),
            },
        }
    }

    pub(crate) fn spawner(&self) -> &Spawner {
        &self.spawner
    }
}

const KEEP_ALIVE: Duration = Duration::from_secs(10);
