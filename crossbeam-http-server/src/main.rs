use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => job(),
                Message::Terminate => break
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Serving on http://127.0.0.1:8080");

    let (sender, workers) = create_thread_pool(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        execute(&sender, Box::new(|| handle_connection(stream)));
    }

    shut_down_workers(sender, workers);
}

fn create_thread_pool(size: usize) -> (Sender<Message>, Vec<Worker>) {
    assert!(size > 0);

    let (sender, receiver) = channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for _ in 0..size {
        workers.push(Worker::new(Arc::clone(&receiver)));
    }

    (sender, workers)
}

fn execute(sender: &Sender<Message>, job: Job) {
    sender.send(Message::NewJob(job)).unwrap();
}

fn shut_down_workers(sender: Sender<Message>, mut workers: Vec<Worker>) {
    send_terminate_to_workers(sender, &workers);
    hold_workers_until_finished(&mut workers);
}

fn send_terminate_to_workers(sender: Sender<Message>, workers: &[Worker]) {
    for _ in workers {
        sender.send(Message::Terminate).unwrap();
    }
}

fn hold_workers_until_finished(workers: &mut Vec<Worker>) {
    for worker in workers {
        if let Some(thread) = worker.thread.take() {
            thread.join().unwrap();
        }
    }
}

#[allow(clippy::unused_io_amount)]
fn handle_read(stream: &mut TcpStream, buffer: &mut [u8]) {
    stream.read(buffer).unwrap();
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
