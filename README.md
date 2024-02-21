# tokio_mini
learn tokio code

# learn step
## 1.learn the runtime.
tokio 会对 thread 进行分类。不同的 task 放到不同的thread去处理。当然也可以增加同一种thread的数量
主要有 blocking task.
## blocking_thread
 - 先研究 Shared in pool.rs
 - Spawner 进行任务分配
 - 宏观上使用全局的 queue 去存储所有这一类 task
 - 真正调度的其实是 closure 
 - 会将 closure 封装为 BlockingTask
