use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

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
    let request = String::from_utf8(buffer.to_vec()).unwrap();
    let response = create_response(request).await;
    write_part.write_all(response.as_bytes()).await.unwrap();
}

async fn create_response(request: String) -> String {
    let response: String;
    if request.contains("GET") {
        response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK".to_string();
    } else if request.contains("POST") {
        let body = get_body_from_request(request.to_string()).await;
        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body,
        );
    } else {
        response = "HTTP/1.1 400".to_string();
    }
    response
}

async fn get_body_from_request(request: String) -> String {
    let request = request.trim_matches(char::from(0));
    let splitted_request = request.split("\r\n");
    let vec: Vec<&str> = splitted_request.collect();
    vec[vec.len() - 1].to_string()
}
