use async_std::prelude::*;

use commons::{ http,
    http::{
        HttpMethod,
        HttpRequest,
    },
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

async fn build_response(req: &HttpRequest) -> String {
    match req.method {
        HttpMethod::GET => {
            let resource_path = &req.metadata.resource_path;
            let fs_path = format!(".{}", resource_path);
            println!("resource: {}", &fs_path);
            let contents = async_std::fs::read(fs_path).await.unwrap(); 

            format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", 
                contents.len(), 
                contents
            )
        }
        HttpMethod::POST => {
            format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", 
                req.content.len(), 
                req.content
            )
        }
        _ => {
            println!("Request could not be parsed");
            String::from("HTTP/1.1 404 NOT FOUND")
        }
    }
}

async fn handle_connection(stream: TcpStream) -> std::io::Result<()> {
    let mut stream = stream;
    
    // read incoming data (currently blocking)
    let mut buf = [0; 2048];
    stream.read(&mut buf).await?;

    // echo back whatever was sent
    let parsed_http = http::parse_http_request(&buf)?;
    let response =  build_response(&parsed_http).await;
    stream.write_all(&mut response.as_bytes()).await?;

    Ok(())
}
