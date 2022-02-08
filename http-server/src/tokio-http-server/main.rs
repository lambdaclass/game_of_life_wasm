use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

use http::{parse_http_request, HttpMethod, HttpRequest};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async {
            handle_new_connection(stream).await;
        });
    }
}

async fn handle_new_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let (read_part, mut write_part) = stream.split();
    let mut reader = BufReader::new(read_part);
    reader.read(&mut buffer).await.unwrap();
    let request = parse_http_request(&buffer).unwrap();
    let response = create_response(request).await;
    write_part.write_all(response.as_bytes()).await.unwrap();
}

async fn create_response(request: HttpRequest) -> String {
    match request.method {
        HttpMethod::GET => "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK".to_string(),
        HttpMethod::POST => {
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                request.content.len(),
                request.content,
            )
        }
        _ => "HTTP/1.1 400".to_string(),
    }
}
