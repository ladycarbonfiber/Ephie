mod session;
mod system;
mod test;
mod trie;

use std::path::PathBuf;
use std::str;
use session::Session;
use system::FileSystem;
use session::{Session};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use trie::FsLike;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    
    let mut system = FsLike::new();
    system
        .insert(PathBuf::from("/"), FsLike::new())
        .expect("Failed to insert");
    let db = Arc::new(Mutex::new(system));
    
    //TODO hashmap of sessions
    let session = Session {
        user: "TestUser".to_owned(),
        working_dir: PathBuf::from("/"),
        file_system: db.clone(),
    };


    loop {
        let (socket, _) = listener.accept().await.unwrap();

        process(socket).await;
    }
}

async fn process(mut socket: TcpStream) {
    println!("Processing");
    let mut buff = [0; 1];
    let mut out = String::new();
    let command_code = socket
        .read_exact(&mut buff)
        .await
        .expect("Failed to read data from socket");
    if command_code > 0 {
        let command = buff[0];
        println!("read command:{}", &command);
        let read_payload = socket
            .read_exact(&mut buff)
            .await
            .expect("Failed to read data from socket");
        if read_payload > 0 {
            let payload_len = buff[0];
            println!("read payload length:{}", &payload_len);
            let mut payload_buffer = vec![0u8; payload_len as usize];
            let mut read_data_len = socket
                .read_exact(&mut payload_buffer)
                .await
                .expect("Failed to read data from socket");
            if read_data_len > 0 {
                let s = match str::from_utf8(&payload_buffer[0..payload_len as usize]) {
                    Ok(v) => v,
                    Err(_) => "error",
                };
                out.push_str(s);
            }
        }
        let message = "success";
        println!("command:{command}\n {out}");
        let mut payload = Vec::new();
        payload.push(message.len() as u8);
        payload.extend(message.as_bytes());
        socket.write_all(&payload).await.unwrap()
    } else {
        println!("no data")
    }
}
