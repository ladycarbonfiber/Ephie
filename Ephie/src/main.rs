mod session;
mod system;
mod test;
mod trie;
use dashmap::DashMap;
use session::Session;
use std::path::PathBuf;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use transport_layer::command::{Command, WRITE_DELIM};
use trie::FsLike;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();

    let mut system = FsLike::new();
    let mut sessions = DashMap::<String, Session>::new();
    system
        .insert(PathBuf::from("/"), FsLike::new())
        .expect("Failed to insert");
    let db = Arc::new(Mutex::new(system));
    let session = Session::new("TestUser".to_string(), db.clone());
    sessions.insert("TestUser".to_string(), session);
    let users = Arc::new(sessions);
    //TODO hashmap of sessions

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let local_sessions = users.clone();
        tokio::spawn(async move {
            process(socket, local_sessions).await;
        });
    }
}

async fn process(mut socket: TcpStream, session_ref: Arc<DashMap<String, Session>>) {
    println!("Processing");
    let mut session = session_ref.get_mut("TestUser").unwrap();
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
            Command::TOUCH(target) => match session.touch(target) {
                Err(message) => message.to_string(),
                Ok(()) => "".to_string(),
            },
            Command::READ(target) => match session.read_file(target) {
                Err(message) => message.to_string(),
                Ok(data) => match str::from_utf8(&data) {
                    Ok(v) => v.to_string(),
                    Err(_) => "Error Reading out bytes".to_string(),
                },
            },
            Command::WRITE(target) => {
                let parts = target.split(WRITE_DELIM).collect::<Vec<&str>>();
                if parts.len() == 2 {
                    match session.write_file(parts[0].to_string(), parts[1].to_string()) {
                        Err(message) => message.to_string(),
                        Ok(()) => "".to_string(),
                    }
                } else {
                    "Mismatched input".to_string()
                }
            }
            Command::CP(target) => {
                let parts = target.split(WRITE_DELIM).collect::<Vec<&str>>();
                if parts.len() == 2 {
                    match session.copy(parts[0].to_string(), parts[1].to_string()) {
                        Err(message) => message.to_string(),
                        Ok(()) => "".to_string(),
                    }
                } else {
                    "Mismatched input".to_string()
                }
            }
            Command::MV(target) => {
                let parts = target.split(WRITE_DELIM).collect::<Vec<&str>>();
                if parts.len() == 2 {
                    match session.mv(parts[0].to_string(), parts[1].to_string()) {
                        Err(message) => message.to_string(),
                        Ok(()) => "".to_string(),
                    }
                } else {
                    "Mismatched input".to_string()
                }
            }
            Command::FIND(target) => match session.find_local(target) {
                Err(mess) => mess.to_string(),
                Ok(list) => {
                    if list.is_empty() {
                        "pattern not found".to_string()
                    } else {
                        list.join(" | ")
                    }
                }
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
