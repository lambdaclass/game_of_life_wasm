use async_std::prelude::*;

// to use block on instead of the attribute macro
use async_std::task;

use async_std::net::TcpListener;

fn main() {
    let fut = connections_loop();
    task::block_on(fut);
}

async fn connections_loop() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        if let Ok(addr) = stream.peer_addr() {
            println!("Incoming connection from: {:?}", addr);
        } else {
            println!("Incoming connection");
        }
    }

    Ok(())
}
