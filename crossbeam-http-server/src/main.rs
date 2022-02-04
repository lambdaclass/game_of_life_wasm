use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Serving on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        thread::spawn(|| {
            let stream = stream.unwrap();
            handle_connection(stream);
        });
    }
}

fn handle_read(stream: &mut TcpStream, buffer: &mut [u8]) {
    stream.read(buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn handle_write(mut stream: TcpStream, buffer: &[u8]) {
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "/Users/ivanlitteri/Lambda/rust-wasm-playground/crossbeam-http-server/templates/get.html")
    } else {
        ("HTTP/1.1 404 ERROR", "/Users/ivanlitteri/Lambda/rust-wasm-playground/crossbeam-http-server/templates/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!("{}\r\n\r\n{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    handle_read(&mut stream, &mut buffer);
    handle_write(stream, &buffer);
}
