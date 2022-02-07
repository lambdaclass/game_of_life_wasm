use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod thread_pool {
    use job::Job;
    use std::sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    };
    use worker::{create_worker, Worker};

    pub enum Message {
        NewJob(Job),
        Terminate,
    }

    pub fn create_thread_pool(size: usize) -> (Sender<Message>, Vec<Worker>) {
        assert!(size > 0);
        let (sender, receiver) = channel();
        let workers = create_workers(receiver, size);
        (sender, workers)
    }

    fn create_workers(receiver: Receiver<Message>, size: usize) -> Vec<Worker> {
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|_| create_worker(Arc::clone(&receiver)))
            .collect();
        workers
    }

    pub fn execute_job(sender: &Sender<Message>, job: Job) {
        sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn shut_down_workers(sender: Sender<Message>, mut workers: Vec<Worker>) {
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
            if let Some(thread) = worker.take() {
                thread.join().unwrap();
            }
        }
    }

    mod worker {
        use super::Message;
        use std::{
            sync::{mpsc::Receiver, Arc, Mutex},
            thread,
        };
        pub type Worker = Option<thread::JoinHandle<()>>;

        pub fn create_worker(receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => job(),
                    Message::Terminate => break,
                }
            });

            Some(thread)
        }
    }

    mod job {
        pub type Job = Box<dyn FnOnce() + Send + 'static>;
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Serving on http://127.0.0.1:8080");

    let (sender, workers) = thread_pool::create_thread_pool(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool::execute_job(&sender, Box::new(|| handle_connection(stream)));
    }

    thread_pool::shut_down_workers(sender, workers);
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
