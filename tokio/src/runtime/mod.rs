mod blocking;
#[cfg_attr(target_os = "wasi", allow(unused_imports))]
pub(crate) use blocking::spawn_blocking;

pub(crate) mod task;

mod handle;
pub use handle::Handle;

mod scheduler;