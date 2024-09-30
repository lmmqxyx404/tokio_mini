#[derive(Debug, Clone)]
pub(super) struct Sender {
    // _tx: Arc<oneshot::Sender<()>>,
}

#[derive(Debug)]
pub(super) struct Receiver {
    // rx: oneshot::Receiver<()>,
}

pub(super) fn channel() -> (Sender, Receiver) {
    todo!()
}
