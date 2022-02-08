use crossbeam::channel::{unbounded, Receiver, Sender};
use http::{parse_http_request, HttpMethod, HttpRequest};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(workers_count: usize) -> ThreadPool {
        assert!(workers_count > 0);
        let (sender, receiver) = unbounded();
        let workers = ThreadPool::create_workers(receiver, workers_count);
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, closure_to_execute: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(closure_to_execute);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    fn create_workers(receiver: Receiver<Message>, size: usize) -> Vec<Worker> {
        (0..size).map(|_| Worker::new(receiver.clone())).collect()
    }

    fn tell_workers_to_terminate(&self) {
        (0..self.workers.len()).for_each(|_| self.sender.send(Message::Terminate).unwrap());
    }

    fn hold_on_until_all_workers_are_done(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.tell_workers_to_terminate();
        self.hold_on_until_all_workers_are_done();
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Receiver<Message>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    println!("Serving on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
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
