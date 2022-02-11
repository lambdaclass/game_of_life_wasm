use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
use commons::http::{parse_http_request, HttpMethod, HttpRequest};
use commons::work_stealing_scheduler::WorkStealingScheduler;


fn main() {
    crossbeam::scope(|scope| {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        let thread_count = 4;
        let work_stealing_scheduler = WorkStealingScheduler::new(scope, thread_count);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let job = Box::new(|| {
                handle_connection(stream);
            });
            work_stealing_scheduler.push_job(Box::new(|| {
                handle_connection(stream);
            }));
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
