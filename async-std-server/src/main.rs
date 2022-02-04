use async_std::prelude::*;

// to use block on instead of the attribute macro
use async_std::task;

use async_std::net::{TcpListener, TcpStream};

fn main() {
    let fut = connections_loop();
    if let Err(e) = task::block_on(fut) {
        eprintln!("{}", e);
    }
}

async fn connections_loop() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    let mut stream = stream;
    
    // read incoming data (currently blocking)
    let mut buf = vec![0; 2048];
    let n = stream.read(&mut buf).await.unwrap();

    // echo back whatever was sent
    if n > 0 {
        stream.write_all(&mut buf[..n]).await.unwrap();
    }

    if let Ok(addr) = stream.peer_addr() {
        println!("Incoming connection from: {:?}", addr);
    } else {
        println!("Incoming connection");
    }
}
