use async_std::prelude::*;

// to use block on instead of the attribute macro
use async_std::task;

use async_std::net::{TcpListener, TcpStream};
use async_std::io;

fn main() {
    let mut args = std::env::args();

    let _ = args.next();
    let addr = args.next().unwrap_or("127.0.0.1".to_string());
    let port = args.next().unwrap_or("8080".to_string());

    let now = std::time::Instant::now();
    task::block_on(run_benchmark(addr, port));
    println!("took: {}ms.", now.elapsed().as_millis());
}

async fn run_benchmark(addr: String, port: String) {
    let num_requests = 20000;
    let whole_addr = format!("{}:{}", addr, port);
    let mut handles = Vec::new();
    for _ in 1..=num_requests {
        let handle = task::spawn(make_request(whole_addr.clone()));
        handles.push(handle);
    }

    for handle in handles {
        handle.await;
    }
}

async fn make_request(addr: String) -> io::Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (mut reader, mut writer) = (&stream, &stream);
    let body = String::from("benchmark body");
    let request = format!("POST / HTTP/1.1\r\nContent-Size:{}\r\n\r\n{}\r\n", body.len(), body);
    writer.write_all(request.as_bytes()).await?;

    let mut buf = vec![0;1024];
    let n = reader.read(&mut buf).await?;
    let request_str = std::str::from_utf8(&buf).expect("invalid UTF-8");
    if request_str.contains("OK") {
        // do stuff
    } else {
        // do other stuff
    }

    Ok(())
}
