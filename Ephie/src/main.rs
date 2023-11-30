mod session;
mod system;
mod test;
mod trie;
use session::Session;
use std::path::PathBuf;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use transport_layer::command::Command;
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
    let mut session = Session::new("TestUser".to_string(), db.clone());

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        process(socket, &mut session).await;
    }
}

async fn process(mut socket: TcpStream, session: &mut Session) {
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
        println!("command:{command}\n {out}");
        let parsed_command = Command::from((command, out));
        let message = match parsed_command {
            Command::CD(target) => match session.change_dir(target) {
                Err(message) => message.to_string(),
                Ok(()) => "".to_string(),
            },
            Command::PWD => session.current_dir().to_str().unwrap().to_string(),
            Command::MKDIR(target) => match session.make_dir(target) {
                Err(message) => message.to_string(),
                Ok(()) => "".to_string(),
            },
            Command::WHO => session.current_user().to_string(),
            Command::LS => {
                let out = session
                    .list()
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join(" | ");
                println!("out is {:#?}", out);
                out.clone()
            }
            Command::RM(target) => match session.remove(target) {
                Err(message) => message.to_string(),
                Ok(()) => "".to_string(),
            },
            Command::UNKNOWN => "Unknown Command".to_string(),
        };
        let mut payload = Vec::new();
        payload.push(message.len() as u8);
        payload.extend(message.as_bytes());
        socket.write_all(&payload).await.unwrap()
    } else {
        println!("no data")
    }
}
