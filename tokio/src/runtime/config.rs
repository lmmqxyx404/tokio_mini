pub(crate) struct Config {
    /// How to build poll time histograms
    pub(crate) metrics_poll_count_histogram: Option<crate::runtime::HistogramBuilder>,
    /// How many ticks before pulling a task from the global/remote queue?
    pub(crate) global_queue_interval: Option<u32>,
}
