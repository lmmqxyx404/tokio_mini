use crate::runtime::task::RawTask;
use std::marker::PhantomData;

cfg_rt! {
    pub struct JoinHandle<T> {
        raw: RawTask,
        _p: PhantomData<T>,
    }
}

unsafe impl<T: Send> Send for JoinHandle<T> {}
unsafe impl<T: Send> Sync for JoinHandle<T> {}
