use std::io;

use crate::runtime::{driver, Runtime};

pub struct Builder {
    /// Runtime type
    kind: Kind,
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
        Builder { kind }
    }

    pub fn build(&mut self) -> io::Result<Runtime> {
        match &self.kind {
            Kind::CurrentThread => self.build_current_thread_runtime(),
        }
    }

    fn build_current_thread_runtime(&mut self) -> io::Result<Runtime> {
        let (driver, driver_handle) = driver::Driver::new(self.get_cfg(1))?;

        todo!()
    }

    fn get_cfg(&self, workers: usize) -> driver::Cfg {
        driver::Cfg {}
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    CurrentThread,
}
