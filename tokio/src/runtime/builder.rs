use std::io;
use std::time::Duration;

use crate::{
    runtime::{
        blocking, config::Config, driver, scheduler::current_thread::CurrentThread, Runtime,
    },
    util::rand::{RngSeed, RngSeedGenerator},
};

use super::HistogramBuilder;

use crate::runtime::handle::Handle;

pub struct Builder {
    /// Runtime type
    kind: Kind,
    /// Cap on thread usage.
    max_blocking_threads: usize,
    /// Customizable keep alive timeout for `BlockingPool`
    pub(super) keep_alive: Option<Duration>,
    /// Whether or not to enable the I/O driver
    enable_io: bool,
    nevents: usize,
    /// Whether or not the clock should start paused.
    start_paused: bool,
    /// Whether or not to enable the time driver
    enable_time: bool,
    /// Specify a random number generator seed to provide deterministic results
    pub(super) seed_generator: RngSeedGenerator,
    /// When true, enables task poll count histogram instrumentation.
    pub(super) metrics_poll_count_histogram_enable: bool,
    /// How many ticks before pulling a task from the global/remote queue?
    ///
    /// When `None`, the value is unspecified and behavior details are left to
    /// the scheduler. Each scheduler flavor could choose to either pick its own
    /// default value or use some other strategy to decide when to poll from the
    /// global queue. For example, the multi-threaded scheduler uses a
    /// self-tuning strategy based on mean task poll times.
    pub(super) global_queue_interval: Option<u32>,
}

impl Builder {
    /// Returns a new builder with the current thread scheduler selected.
    ///
    /// Configuration methods can be chained on the return value.
    ///
    /// To spawn non-`Send` tasks on the resulting runtime, combine it with a
    /// [`LocalSet`].
    ///
    /// [`LocalSet`]: crate::task::LocalSet
    pub fn new_current_thread() -> Builder {
        #[cfg(loom)]
        const EVENT_INTERVAL: u32 = 4;
        // The number `61` is fairly arbitrary. I believe this value was copied from golang.
        #[cfg(not(loom))]
        const EVENT_INTERVAL: u32 = 61;

        Builder::new(Kind::CurrentThread, EVENT_INTERVAL)
    }

    /// Returns a new runtime builder initialized with default configuration
    /// values.
    ///
    /// Configuration methods can be chained on the return value.
    pub(crate) fn new(kind: Kind, event_interval: u32) -> Builder {
        Builder {
            kind,
            max_blocking_threads: 512,

            keep_alive: None,

            // I/O defaults to "off"
            enable_io: false,
            nevents: 1024,
            // The clock starts not-paused
            start_paused: false,

            // Time defaults to "off"
            enable_time: false,

            seed_generator: RngSeedGenerator::new(RngSeed::new()),

            metrics_poll_count_histogram_enable: false,

            // Defaults for these values depend on the scheduler kind, so we get them
            // as parameters.
            global_queue_interval: None,
        }
    }

    pub fn build(&mut self) -> io::Result<Runtime> {
        match &self.kind {
            Kind::CurrentThread => self.build_current_thread_runtime(),
        }
    }

    fn build_current_thread_runtime(&mut self) -> io::Result<Runtime> {
        use crate::runtime::runtime::Scheduler;
        use crate::runtime::scheduler::{self};

        let (driver, driver_handle) = driver::Driver::new(self.get_cfg(1))?;

        // Blocking pool
        let blocking_pool = blocking::create_blocking_pool(self, self.max_blocking_threads);
        let blocking_spawner = blocking_pool.spawner().clone();

        // Generate a rng seed for this runtime.
        let seed_generator_1 = self.seed_generator.next_generator();
        let seed_generator_2 = self.seed_generator.next_generator();

        let (scheduler, handle) = CurrentThread::new(
            driver,
            driver_handle,
            blocking_spawner,
            seed_generator_2,
            Config {
                metrics_poll_count_histogram: self.metrics_poll_count_histogram_builder(),
                global_queue_interval: self.global_queue_interval,
            },
        );

        let handle = Handle {
            inner: scheduler::Handle::CurrentThread(handle),
        };

        Ok(Runtime::from_parts(
            Scheduler::CurrentThread(scheduler),
            handle,
            blocking_pool,
        ))
    }

    fn get_cfg(&self, workers: usize) -> driver::Cfg {
        driver::Cfg {
            enable_io: self.enable_io,
            nevents: self.nevents,
            enable_pause_time: match self.kind {
                Kind::CurrentThread => true,
            },
            start_paused: self.start_paused,
            enable_time: self.enable_time,
            workers,
        }
    }

    fn metrics_poll_count_histogram_builder(&self) -> Option<HistogramBuilder> {
        if self.metrics_poll_count_histogram_enable {
            todo!() // Some(self.metrics_poll_count_histogram.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    CurrentThread,
}
