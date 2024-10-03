pub(crate) struct Config {
    /// How to build poll time histograms
    pub(crate) metrics_poll_count_histogram: Option<crate::runtime::HistogramBuilder>,
}
