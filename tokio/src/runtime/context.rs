use crate::runtime::scheduler;

cfg_rt! {
  mod current;
  pub(crate) use current::{try_set_current, with_current, SetCurrentGuard};
}

struct Context {}

tokio_thread_local! {
    static CONTEXT: Context = const { Context {} }
}

impl Context {
    pub(super) fn set_current(&self, handle: &scheduler::Handle) -> SetCurrentGuard {
        todo!()
    }
}
