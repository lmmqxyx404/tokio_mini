#![cfg_attr(not(feature = "rt"), allow(dead_code))]

cfg_test_util! {
  /// A handle to a source of time.
  #[derive(Debug)]
  pub(crate) struct Clock {
      // inner: Mutex<Inner>,
  }

  impl Clock {
    /// Returns a new `Clock` instance that uses the current execution context's
    /// source of time.
    pub(crate) fn new(enable_pausing: bool, start_paused: bool) -> Clock {
      let now = std::time::Instant::now();
      let clock=Clock{};
      if start_paused {
        todo!()
        /* if let Err(msg) = clock.pause() {
            todo!()// panic!("{}", msg);
        } */
      }

      clock

    }

    pub(crate) fn pause(&self) -> Result<(), &'static str> {
      todo!()
    }
  }
}

// pub(crate) struct Clock {}
