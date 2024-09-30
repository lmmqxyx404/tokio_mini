mod pool;
pub(crate) use pool::{spawn_blocking, BlockingPool, Spawner};

use crate::runtime::Builder;

pub(crate) fn create_blocking_pool(builder: &Builder, thread_cap: usize) -> BlockingPool {
    BlockingPool::new(builder, thread_cap)
}

mod shutdown;
