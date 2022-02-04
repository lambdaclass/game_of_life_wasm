use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Serving on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established: {}", stream.peer_addr().unwrap());
    }
}
