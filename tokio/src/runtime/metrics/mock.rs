use std::thread::ThreadId;

pub(crate) struct WorkerMetrics {}

impl WorkerMetrics {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn from_config(config: &crate::runtime::Config) -> Self {
        // Prevent the dead-code warning from being triggered
        let _ = &config.metrics_poll_count_histogram;
        Self::new()
    }
    pub(crate) fn set_thread_id(&self, _thread_id: ThreadId) {}

}

#[derive(Clone, Default)]
pub(crate) struct HistogramBuilder {}
