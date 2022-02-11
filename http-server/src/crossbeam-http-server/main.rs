use commons::http::{parse_http_request, HttpMethod, HttpRequest};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use commons::work_stealing_scheduler::WorkPool;

fn main() {
    crossbeam::scope(|scope| {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        let thread_count = 4;
        let work_pool = WorkPool::new();

        for _ in 0..thread_count {
            let work_pool = work_pool.clone();
            let thread = scope.spawn(move |_| {
                let work_pool = work_pool;
                loop {
                    if let Some(job) = work_pool.find_job().take() {
                        job();
                    }
                }
            });
        }

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let job = Box::new(|| {
                handle_connection(stream);
            });
            work_pool.push_job(job);
        }
    })
    .unwrap();
}

#[allow(clippy::unused_io_amount)]
fn handle_read(stream: &mut TcpStream, buffer: &mut [u8]) {
    stream.read(buffer).unwrap();
}

fn create_response(request: HttpRequest) -> String {
    match request.method {
        HttpMethod::GET => {
            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                "HTTP/1.1 200 OK",
                request.content.len(),
                request.content
            )
        }
        HttpMethod::POST => {
            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                "HTTP/1.1 200 OK",
                request.content.len(),
                request.content
            )
        }
        _ => "HTTP/1.1 404 NOT FOUND".to_string(),
    }
}

fn handle_write(mut stream: TcpStream, request: HttpRequest) {
    stream
        .write_all(create_response(request).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    handle_read(&mut stream, &mut buffer);
    let request = parse_http_request(&buffer).unwrap();
    handle_write(stream, request);
}
