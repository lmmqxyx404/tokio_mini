use std::future::Future;

#[derive(Debug)]
pub struct Runtime {}

impl Runtime {
    #[track_caller]
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        todo!()
    }
}
