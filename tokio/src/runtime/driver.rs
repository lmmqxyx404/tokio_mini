use std::io;

#[derive(Debug)]
pub(crate) struct Driver {}

#[derive(Debug)]
pub(crate) struct Handle {}

pub(crate) struct Cfg {
    pub(crate) enable_io: bool,
    pub(crate) nevents: usize,
}

impl Driver {
    pub(crate) fn new(cfg: Cfg) -> io::Result<(Self, Handle)> {
        let (io_stack, io_handle, signal_handle) = create_io_stack(cfg.enable_io, cfg.nevents)?;

        todo!()
    }
}

// ===== io driver =====

cfg_io_driver! {
  #[derive(Debug)]
  pub(crate) enum IoStack {}

  #[derive(Debug)]
  pub(crate) enum IoHandle {}

  fn create_io_stack(enabled: bool, nevents: usize) -> io::Result<(IoStack, IoHandle, SignalHandle)> {
    todo!()
  }
}

// ===== signal driver =====

cfg_signal_internal_and_unix! {
  pub(crate) type SignalHandle = Option<crate::runtime::signal::Handle>;
}