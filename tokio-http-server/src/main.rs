use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    handle_new_connection(stream).await;
}

async fn handle_new_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
