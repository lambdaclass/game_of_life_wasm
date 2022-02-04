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
        if let Ok(addr) = stream.peer_addr() {
            println!("Incoming connection from: {:?}", addr);
        } else {
            println!("Incoming connection");
        }
        task::spawn(handle_connection(stream));
    }

    Ok(())
}

fn get_resource_path_from_request(buf: &String) -> String {
    let mut lines = buf.lines();
    let request_line = lines.next().unwrap();
    let request_line : Vec<&str> = request_line.split(" ").collect();
    println!("{:?}", request_line);
    format!(".{}", request_line[1])
}

async fn handle_connection(stream: TcpStream) {
    let mut stream = stream;
    
    // read incoming data (currently blocking)
    let mut buf = vec![0; 2048];
    let n = stream.read(&mut buf).await.unwrap();

    // echo back whatever was sent
    if n > 0 {
        match std::str::from_utf8(&mut buf) {
            Ok(request) if request.starts_with("GET") => {
                let resource_path = get_resource_path_from_request(&String::from(request));
                println!("resource: {}", resource_path);
                let contents = async_std::fs::read_to_string(resource_path).await.unwrap(); 

                let response = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", contents.len(), contents);
                stream.write(&mut response.as_bytes()).await.unwrap();
            }
            Ok(request) if request.starts_with("POST") => {
                let response = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", request.len(), request);
                stream.write(&mut response.as_bytes()).await.unwrap();
            }
            _ => {
                println!("Request could not be parsed");
            }
        }
    }
}
