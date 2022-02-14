use commons::http::{parse_http_request, HttpMethod, HttpRequest};
use commons::work_stealing_scheduler::WorkStealingScheduler;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    crossbeam::scope(|scope| {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        let thread_count = 4;
        let work_stealing_scheduler = WorkStealingScheduler::new(scope, thread_count);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
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

fn create_response(request: HttpRequest) -> std::io::Result<Vec<u8>> {
    match request.method {
        HttpMethod::GET => {
            let resource_path = &request.metadata.resource_path;
            let fs_path = format!(".{}", resource_path);
            let mut contents = std::fs::read(fs_path).unwrap();
            let header = if resource_path.ends_with(".wasm") {
                "Content-type:application/wasm\r\n"
            } else {
                ""
            };
            let mut response = format!(
                "HTTP/1.1 200 OK\r\n{}Content-Length:{}\r\n\r\n",
                &header,
                contents.len()
            )
            .into_bytes();
            response.append(&mut contents);
            Ok(response)
        }
        HttpMethod::POST => Ok(format!(
            "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
            request.content.len(),
            request.content
        )
        .into_bytes()),
        _ => {
            println!("Request could not be parsed");
            Ok(String::from("HTTP/1.1 404 NOT FOUND").into_bytes())
        }
    }
}

fn handle_write(mut stream: TcpStream, request: HttpRequest) {
    let mut response = create_response(request).unwrap();
    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    handle_read(&mut stream, &mut buffer);
    let request = parse_http_request(&buffer).unwrap();
    handle_write(stream, request);
}
