macro_rules! cfg_rt {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "rt")]
          #[cfg_attr(docsrs, doc(cfg(feature = "rt")))]
          $item
      )*
  }
}

macro_rules! cfg_sync {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "sync")]
          #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
          $item
      )*
  }
}

macro_rules! cfg_not_sync {
  ($($item:item)*) => {
      $( #[cfg(not(feature = "sync"))] $item )*
  }
}

macro_rules! cfg_io_driver {
  ($($item:item)*) => {
      $(
          #[cfg(any(
              feature = "net",
              all(unix, feature = "process"),
              all(unix, feature = "signal"),
          ))]
          #[cfg_attr(docsrs, doc(cfg(any(
              feature = "net",
              all(unix, feature = "process"),
              all(unix, feature = "signal"),
          ))))]
          $item
      )*
  }
}

macro_rules! cfg_signal_internal {
  ($($item:item)*) => {
      $(
          #[cfg(any(feature = "signal", all(unix, feature = "process")))]
          #[cfg(not(loom))]
          $item
      )*
  }
}

macro_rules! cfg_signal_internal_and_unix {
  ($($item:item)*) => {
      #[cfg(unix)]
      cfg_signal_internal! { $($item)* }
  }
}

macro_rules! cfg_time {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "time")]
          #[cfg_attr(docsrs, doc(cfg(feature = "time")))]
          $item
      )*
  }
}

macro_rules! cfg_test_util {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "test-util")]
          #[cfg_attr(docsrs, doc(cfg(feature = "test-util")))]
          $item
      )*
  }
}

macro_rules! cfg_not_test_util {
  ($($item:item)*) => {
      $( #[cfg(not(feature = "test-util"))] $item )*
  }
}

macro_rules! cfg_not_unstable_metrics {
  ($($item:item)*) => {
      $(
          #[cfg(not(tokio_unstable))]
          $item
      )*
  }
}
