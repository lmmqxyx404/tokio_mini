use std::io;

use crate::runtime::park::{ParkThread, UnparkThread};

#[derive(Debug)]
pub(crate) struct Driver {}

#[derive(Debug)]
pub(crate) struct Handle {}

pub(crate) struct Cfg {
    pub(crate) enable_io: bool,
    pub(crate) nevents: usize,
    pub(crate) enable_pause_time: bool,
    pub(crate) start_paused: bool,
}

impl Driver {
    pub(crate) fn new(cfg: Cfg) -> io::Result<(Self, Handle)> {
        let (io_stack, io_handle, signal_handle) = create_io_stack(cfg.enable_io, cfg.nevents)?;

        let clock = create_clock(cfg.enable_pause_time, cfg.start_paused);

        todo!()
    }
}

// ===== io driver =====

cfg_io_driver! {
  #[derive(Debug)]
  pub(crate) enum IoStack {
    Disabled(ParkThread),

  }

  #[derive(Debug)]
  pub(crate) enum IoHandle {
    Disabled(UnparkThread),

  }


  fn create_io_stack(enabled: bool, nevents: usize) -> io::Result<(IoStack, IoHandle, SignalHandle)> {
    #[cfg(loom)]
    assert!(!enabled);

    let ret = if enabled {
       todo!()
    } else {
        let park_thread = ParkThread::new();
        let unpark_thread = park_thread.unpark();
        (IoStack::Disabled(park_thread), IoHandle::Disabled(unpark_thread), Default::default())
    };

    Ok(ret)
}

}

// ===== signal driver =====

cfg_signal_internal_and_unix! {
  pub(crate) type SignalHandle = Option<crate::runtime::signal::Handle>;
}

// ===== time driver =====

cfg_time! {
  pub(crate) type Clock = crate::time::Clock;

  fn create_clock(enable_pausing: bool, start_paused: bool) -> Clock {
    crate::time::Clock::new(enable_pausing, start_paused)
  }
}