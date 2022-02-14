use tokio::fs::File;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

use commons::http::{parse_http_request, HttpMethod, HttpRequest};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async {
            handle_new_connection(stream).await.unwrap();
        });
    }
}

async fn handle_new_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 2048];
    let mut reader = BufReader::new(&mut stream);
    reader.read(&mut buffer).await.unwrap();
    let request = parse_http_request(&buffer).unwrap();
    let response = create_response(request).await?;
    stream.write_all(&response).await?;

    Ok(())
}

async fn create_response(request: HttpRequest) -> std::io::Result<Vec<u8>> {
    match request.method {
        HttpMethod::GET => {
            let file_requested_path = ".".to_string() + &request.metadata.resource_path;
            let mut file = File::open(&file_requested_path).await?;
            let mut content = Vec::new();
            file.read_to_end(&mut content).await?;
            let header = if file_requested_path.ends_with(".wasm") {
                "Content-type:application/wasm\r\n".to_string()
            } else {
                "".to_string()
            };
            let mut resp = format!(
                "HTTP/1.1 200 OK\r\n{}Content-Length:{}\r\n\r\n",
                header,
                content.len()
            )
            .into_bytes();
            resp.append(&mut content);
            Ok(resp)
        }
        HttpMethod::POST => Ok(format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            request.content.len(),
            request.content,
        )
        .into_bytes()),
        _ => Ok("HTTP/1.1 400".to_string().into_bytes()),
    }
}
