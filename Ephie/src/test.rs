use crate::{system::Command, session::{Session, self}, trie::{Node, Trie}};
use std::sync::{Arc, Mutex};

fn test_system() -> Trie{
    Trie::new()

}

fn test_session() -> Session {
    let db = Arc::new(Mutex::new(test_system()));
    Session{user:"TestUser".to_owned(), working_dir:"/".to_owned(), file_system: db.clone()}
}

#[test]
fn test_ls_at_root(){
    println!("{:#?}", test_session());
    let mut session = test_session();
    let command = Command::LS;
    let expected = "/Documents|/Downloads";
    let out = session.resolve_command(command);
    println!("out is {:#?}", out);
    assert_eq!(expected, out);
}