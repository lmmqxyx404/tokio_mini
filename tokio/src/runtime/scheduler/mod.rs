#[derive(Debug, Clone)]
pub(crate) enum Handle {
    #[cfg(feature = "rt")]
    CurrentThread(Arc<current_thread::Handle>),

    #[cfg(all(feature = "rt-multi-thread", not(target_os = "wasi")))]
    MultiThread(Arc<multi_thread::Handle>),

    #[cfg(all(tokio_unstable, feature = "rt-multi-thread", not(target_os = "wasi")))]
    MultiThreadAlt(Arc<multi_thread_alt::Handle>),

    // TODO: This is to avoid triggering "dead code" warnings many other places
    // in the codebase. Remove this during a later cleanup
    #[cfg(not(feature = "rt"))]
    #[allow(dead_code)]
    Disabled,
}
