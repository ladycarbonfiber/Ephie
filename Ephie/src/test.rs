use crate::{
    session::Session,
    system::Command,
    trie::{FsLike, FsLike::FileLike},
};
use std::path::PathBuf;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

fn test_system() -> FsLike {
    let mut system = FsLike::new();
    system
        .insert(PathBuf::from("/"), FsLike::new())
        .expect("Failed to insert");
    system
        .insert(PathBuf::from("/Documents/"), FsLike::new())
        .expect("Failed to insert");
    system
        .insert(PathBuf::from("/Documents/projects"), FsLike::new())
        .expect("Failed to insert");
    system
        .insert(PathBuf::from("/Documents/paperwork"), FsLike::new())
        .expect("Failed to insert");
    system
        .insert(PathBuf::from("/Downloads/"), FsLike::new())
        .expect("Failed to insert");
    system
        .insert(
            PathBuf::from("/Downloads/test.hello"),
            FileLike {
                data: String::from("hello world").into_bytes(),
            },
        )
        .expect("Failed to insert");
    system
}

fn test_session() -> Session {
    let db = Arc::new(Mutex::new(test_system()));
    Session {
        user: "TestUser".to_owned(),
        working_dir: PathBuf::from("/"),
        file_system: db.clone(),
    }
}

#[test]
fn test_ls_at_root() {
    let session = test_session();

    println!("{:#?}", &test_session());

    let expected: HashSet<String> = vec!["Documents".to_string(), "Downloads".to_string()]
        .into_iter()
        .collect();
    let out = session.list();
    println!("out is {:#?}", out);
    assert_eq!(expected, out);
}
#[test]
fn test_pwd_at_root() {
    let session = test_session();
    let expected = PathBuf::from("/");
    let out = session.current_dir();
    assert_eq!(out, expected);
}
#[test]
fn test_cd() {
    let mut session = test_session();
    session
        .change_dir("Documents/".to_string())
        .expect("Dir not found");
    let expected = PathBuf::from("/Documents/");
    let out = session.current_dir();
    assert_eq!(out, expected);
    session
        .change_dir("paperwork/".to_string())
        .expect("Not Found");
    let expected_second = PathBuf::from("/Documents/paperwork/");
    let out_second = session.current_dir();
    println!("{:#?}", &test_session());
    assert_eq!(out_second, expected_second);
}
#[test]
fn test_mkdir_absolute() {
    let mut session = test_session();
    session
        .make_dir("/Pictures".to_string())
        .expect("Root not found");
    let out = session.list();
    assert!(out.contains("Pictures"));
}
#[test]
fn test_mkdir_relative() {
    let mut session = test_session();
    session.change_dir("Documents".to_string()).unwrap();
    session
        .make_dir("Pictures".to_string())
        .expect("Root not found");
    let out = session.list();
    assert!(out.contains("Pictures"));
}
