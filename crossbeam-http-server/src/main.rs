use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crossbeam::channel::{unbounded, Receiver, Sender};

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
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn create_response(buffer: &[u8]) -> String {
    let (status_line, filename) = if buffer.starts_with(b"GET") {
        ("HTTP/1.1 200 OK", "/Users/ivanlitteri/Lambda/rust-wasm-playground/crossbeam-http-server/templates/get.html")
    } else if buffer.starts_with(b"POST") {
        ("HTTP/1.1 200 OK", "/Users/ivanlitteri/Lambda/rust-wasm-playground/crossbeam-http-server/templates/post.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "/Users/ivanlitteri/Lambda/rust-wasm-playground/crossbeam-http-server/templates/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    response
}

fn handle_write(mut stream: TcpStream, buffer: &[u8]) {
    stream
        .write_all(create_response(buffer).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    handle_read(&mut stream, &mut buffer);
    handle_write(stream, &buffer);
}
