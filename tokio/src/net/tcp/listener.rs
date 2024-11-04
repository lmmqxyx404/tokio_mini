// todo:add cfg!
use crate::net::ToSocketAddrs;
use std::io;

cfg_net! {
  /// A TCP socket server, listening for connections.
  ///
  /// You can accept a new connection by using the [`accept`](`TcpListener::accept`)
  /// method.
  ///
  /// A `TcpListener` can be turned into a `Stream` with [`TcpListenerStream`].
  ///
  /// [`TcpListenerStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.TcpListenerStream.html
  ///
  /// # Errors
  ///
  /// Note that accepting a connection can lead to various errors and not all
  /// of them are necessarily fatal ‒ for example having too many open file
  /// descriptors or the other side closing the connection while it waits in
  /// an accept queue. These would terminate the stream if not handled in any
  /// way.
  ///
  /// # Examples
  ///
  /// Using `accept`:
  /// ```no_run
  /// use tokio::net::TcpListener;
  ///
  /// use std::io;
  ///
  /// async fn process_socket<T>(socket: T) {
  ///     # drop(socket);
  ///     // do work with socket here
  /// }
  ///
  /// #[tokio::main]
  /// async fn main() -> io::Result<()> {
  ///     let listener = TcpListener::bind("127.0.0.1:8080").await?;
  ///
  ///     loop {
  ///         let (socket, _) = listener.accept().await?;
  ///         process_socket(socket).await;
  ///     }
  /// }
  /// ```
  pub struct TcpListener {
      // io: PollEvented<mio::net::TcpListener>,
  }
}

impl TcpListener {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        todo!()
    }
}
