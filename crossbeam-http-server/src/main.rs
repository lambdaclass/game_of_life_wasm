use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Serving on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        thread::spawn(move || {
            let stream = stream.unwrap();
            handle_connection(stream);
        });
    }
}

fn handle_read(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn handle_write(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    handle_read(&mut stream);
    handle_write(stream);
}
