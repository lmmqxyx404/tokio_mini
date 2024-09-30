use crate::loom::sync::Arc;
use crate::sync::oneshot;

#[derive(Debug, Clone)]
pub(super) struct Sender {
    _tx: Arc<oneshot::Sender<()>>,
}

#[derive(Debug)]
pub(super) struct Receiver {
    rx: oneshot::Receiver<()>,
}

pub(super) fn channel() -> (Sender, Receiver) {
    let (tx, rx) = oneshot::channel();
    let tx = Sender { _tx: Arc::new(tx) };
    let rx = Receiver { rx };

    (tx, rx)
}
