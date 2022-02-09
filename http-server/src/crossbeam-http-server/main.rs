use commons::http::{parse_http_request, HttpMethod, HttpRequest};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    iter,
};

use crossbeam::deque::{Injector, Stealer, Worker, Steal};

type Job = Box<dyn FnOnce() + Send + 'static>;

fn main() {
    crossbeam::scope(|scope| {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        let thread_count = 4;
        let injector = Arc::new(Injector::<Job>::new());
        let stealers: Arc<Mutex<Vec<Stealer<Job>>>> = Arc::new(Mutex::new(vec![]));
        let mut workers = Vec::new();
    
        for _ in 0..thread_count {
            let global = injector.clone();
            let stealers = stealers.clone();
            let thread = scope.spawn(move |_| {
                let global = global;
                let stealers = stealers;
                let local = Worker::new_fifo();
                stealers.lock().unwrap().push(local.stealer());
                loop {
                    if let Some(job) = get_work(&local, &global, &stealers).take() {
                        job();
                    }
                }
            });
            workers.push(thread);
        }
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let job = Box::new(|| {
                handle_connection(stream);
            });
            injector.push(job);
        }
    }).unwrap();
}

fn get_work(
    local: &Worker<Job>,
    global: &Arc<Injector<Job>>,
    stealers: &Arc<Mutex<Vec<Stealer<Job>>>>,
) -> Option<Job> {
    local.pop().or_else(|| {
        iter::repeat_with(|| {
            global.steal_batch_and_pop(local).or_else(|| {
                stealers
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|stealer| stealer.steal())
                    .collect()
            })
        })
        .find(|steal| !steal.is_retry())
        .and_then(|steal| steal.success())
    })
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
