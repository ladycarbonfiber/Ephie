use tokio::net::{TcpStream};
use tokio::io::{AsyncWriteExt};
#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").await.unwrap();

    let result = stream.write(b"hello\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());
}