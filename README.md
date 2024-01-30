# tokio_mini
learn tokio code

# learn step
## 1.learn the runtime.
tokio 会对 thread 进行分类。不同的 task 放到不同的thread去处理。当然也可以增加同类的数量

## blocking_thread
 - 先研究 Shared in pool.rs
 - Spawner 进行任务分配

