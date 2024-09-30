use crate::runtime::{context, scheduler};

use std::{fmt, marker::PhantomData};

use crate::runtime::task::JoinHandle;

/// Handle to the runtime.
///
/// The handle is internally reference-counted and can be freely cloned. A handle can be
/// obtained using the [`Runtime::handle`] method.
///
/// [`Runtime::handle`]: crate::runtime::Runtime::handle()
#[derive(Debug, Clone)]
// When the `rt` feature is *not* enabled, this type is still defined, but not
// included in the public API.
pub struct Handle {
    pub(crate) inner: scheduler::Handle,
}

/// Runtime context guard.
///
/// Returned by [`Runtime::enter`] and [`Handle::enter`], the context guard exits
/// the runtime context on drop.
///
/// [`Runtime::enter`]: fn@crate::runtime::Runtime::enter
#[derive(Debug)]
#[must_use = "Creating and dropping a guard does nothing"]
pub struct EnterGuard<'a> {
    _guard: context::SetCurrentGuard,
    _handle_lifetime: PhantomData<&'a Handle>,
}

impl Handle {
    /// Enters the runtime context. This allows you to construct types that must
    /// have an executor available on creation such as [`Sleep`] or
    /// [`TcpStream`]. It will also allow you to call methods such as
    /// [`tokio::spawn`] and [`Handle::current`] without panicking.
    ///
    /// # Panics
    ///
    /// When calling `Handle::enter` multiple times, the returned guards
    /// **must** be dropped in the reverse order that they were acquired.
    /// Failure to do so will result in a panic and possible memory leaks.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// let rt = Runtime::new().unwrap();
    ///
    /// let _guard = rt.enter();
    /// tokio::spawn(async {
    ///     println!("Hello world!");
    /// });
    /// ```
    ///
    /// Do **not** do the following, this shows a scenario that will result in a
    /// panic and possible memory leak.
    ///
    /// ```should_panic
    /// use tokio::runtime::Runtime;
    ///
    /// let rt1 = Runtime::new().unwrap();
    /// let rt2 = Runtime::new().unwrap();
    ///
    /// let enter1 = rt1.enter();
    /// let enter2 = rt2.enter();
    ///
    /// drop(enter1);
    /// drop(enter2);
    /// ```
    ///
    /// [`Sleep`]: struct@crate::time::Sleep
    /// [`TcpStream`]: struct@crate::net::TcpStream
    /// [`tokio::spawn`]: fn@crate::spawn
    pub fn enter(&self) -> EnterGuard<'_> {
        EnterGuard {
            _guard: match context::try_set_current(&self.inner) {
                Some(guard) => guard,
                None => panic!("{}", crate::util::error::THREAD_LOCAL_DESTROYED_ERROR),
            },
            _handle_lifetime: PhantomData,
        }
    }

    /// 2 TODO: add the doc
    #[track_caller]
    pub fn current() -> Self {
        Handle {
            inner: scheduler::Handle::current(),
        }
    }

    #[track_caller]
    pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        todo!()
    }
    // todo: add more fn.
}

/// Error returned by `try_current` when no Runtime has been started
#[derive(Debug)]
pub struct TryCurrentError {
    // kind: TryCurrentErrorKind,
}

impl fmt::Display for TryCurrentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
