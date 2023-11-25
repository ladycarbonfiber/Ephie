use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt};
use std::str;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(mut socket:TcpStream){
    println!("Processing");
    let mut buff = vec![0;5];
    let mut out = String::new();
    let  mut data = socket.read(&mut buff).await.expect("Failed to read data from socket");
    if data > 0{
        while data != 0{
            let s = match str::from_utf8(&buff[0..data]){
                Ok(v) => v,
                Err(_) => "error"
            };
            out.push_str(s);
            data = socket.read(&mut buff).await.expect("Failed to read data from socket");
        }
        println!("{out}")

    }
    else{
        println!("no data")
    }
}
