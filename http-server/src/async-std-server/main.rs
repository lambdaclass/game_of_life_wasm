use async_std::prelude::*;

use http::{
    HttpMethod,
};

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
        if let Ok(addr) = stream.peer_addr() {
            println!("Incoming connection from: {:?}", addr);
        } else {
            println!("Incoming connection");
        }
        task::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) -> std::io::Result<()> {
    let mut stream = stream;
    
    // read incoming data (currently blocking)
    let mut buf = [0; 2048];
    let n = stream.read(&mut buf).await?;

    // echo back whatever was sent
    if n > 0 {
        let parsed_http = http::parse_http_request(&buf)?;
        match parsed_http.method {
            HttpMethod::GET => {
                let resource_path = &parsed_http.metadata.resource_path;
                let fs_path = format!(".{}", resource_path);
                println!("resource: {}", &fs_path);
                let contents = async_std::fs::read_to_string(fs_path).await?; 

                let response = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", contents.len(), contents);
                stream.write(&mut response.as_bytes()).await?;
                println!("Sent:\n{}", &response);
            }
            HttpMethod::POST => {
                let response = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", parsed_http.content.len(), parsed_http.content);
                stream.write(&mut response.as_bytes()).await?;
            }
            _ => {
                println!("Request could not be parsed");
            }
        }
    }

    Ok(())
}
