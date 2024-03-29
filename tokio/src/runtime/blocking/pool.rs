use crate::runtime::task::{JoinHandle};
use crate::runtime::{Handle};

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
    todo!()
    // todo:1 handle is the start then is relative fn.
    // let rt = Handle::current();
    
    // rt.spawn_blocking(func)
}