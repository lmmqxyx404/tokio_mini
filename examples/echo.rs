use std::env;
use std::error::Error;
use tokio::net::TcpListener;

fn main() -> Result<(), Box<dyn Error>> {
    // 创建runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let a = 5;
    rt.block_on(async move {
        // Allow passing an address to listen on as the first argument of this
        // program, but otherwise we'll just set up our TCP listener on
        // 127.0.0.1:8080 for connections.
        let addr = env::args()
            .nth(1)
            .unwrap_or_else(|| "127.0.0.1:8080".to_string());

        // Next up we create a TCP listener which will listen for incoming
        // connections. This TCP listener is bound to the address we determined
        // above and must be associated with an event loop.
        let listener = TcpListener::bind(addr.clone()).await?;
        println!("Listening on: {}", addr);
        println!("hello  world,{}", a);
        Ok(())
    })
}
