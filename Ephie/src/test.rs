use crate::{
    session::{self, Session},
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
    Session::new("TestUser".to_string(), db.clone())
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
fn test_pwd_at_folder() {
    let mut session = test_session();
    let expected = PathBuf::from("/Documents/projects");
    session
        .change_dir("/Documents/projects".to_string())
        .expect("Dir not found");
    let out = session.current_dir();
    assert_eq!(out, expected);
}
#[test]
fn test_pwd_at_folder_step_into() {
    let mut session = test_session();
    let expected = PathBuf::from("/Documents/projects");
    session
        .change_dir("Documents".to_string())
        .expect("Dir not found");
    session
        .change_dir("projects".to_string())
        .expect("Dir not found");
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
fn test_cd_parent() {
    let mut session = test_session();
    session
        .change_dir("Documents".to_string())
        .expect("not Found");
    session.change_dir("..".to_string()).expect("not found");
    let out = session.current_dir();
    assert_eq!(out, PathBuf::from("/"))
}
#[test]
fn test_cd_parent_multi() {
    let mut session = test_session();
    session
        .change_dir("Documents/paperwork".to_string())
        .expect("not Found");
    session
        .change_dir("../projects".to_string())
        .expect("not found");
    let out = session.current_dir();
    assert_eq!(out, PathBuf::from("/Documents/projects"))
}
// We don't support this yet
#[test]
#[should_panic]
fn test_cd_parent_nested() {
    let mut session = test_session();
    session
        .change_dir("Documents/paperwork".to_string())
        .expect("not Found");
    session.change_dir("../..".to_string()).expect("not found");
    let out = session.current_dir();
    assert_eq!(out, PathBuf::from("/"))
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
#[test]
fn test_mkdir_parent() {
    let mut session = test_session();
    session.change_dir("Documents".to_string()).unwrap();
    session.make_dir("../Pictures".to_string()).unwrap();
    session.change_dir("..".to_string()).unwrap();
    let out = session.list();
    assert!(out.contains("Pictures"))
}
#[test]
fn test_mkdir_absolute_nested() {
    let mut session = test_session();
    session
        .make_dir("/Pictures/Mexico".to_string())
        .expect("Root not found");
    session
        .change_dir("Pictures".to_string())
        .expect("not found");
    let out = session.list();
    assert!(out.contains("Mexico"));
}
#[test]
fn test_mkdir_relative_nested() {
    let mut session = test_session();
    session
        .make_dir("Pictures/Mexico".to_string())
        .expect("Root not found");
    session
        .change_dir("Pictures".to_string())
        .expect("not found");
    let out = session.list();
    assert!(out.contains("Mexico"));
}
#[test]
fn test_rm_directory_present() {
    let mut session = test_session();
    session.remove("/Downloads".to_string()).unwrap();
    let out = session.list();
    assert!(!out.contains("Downloads"))
}
#[test]
fn test_rm_file_nested() {
    let mut session = test_session();
    session.write_file("Downloads/new.file".to_string(), "test content".to_string());
    session.remove("/Downloads/new.file".to_string()).unwrap();
    session.change_dir("Downloads".to_string()).unwrap();
    let out = session.list();
    assert!(!out.contains("new.file"))
}
#[test]
fn test_rm_file_nested_relative() {
    let mut session = test_session();
    session
        .write_file("Downloads/new.file".to_string(), "test content".to_string())
        .unwrap();
    session.remove("Downloads/new.file".to_string()).unwrap();
    session.change_dir("Downloads".to_string()).unwrap();
    let out = session.list();
    assert!(!out.contains("new.file"))
}
#[test]
fn test_rm_directory_nested() {
    let mut session = test_session();
    session.write_file(
        "Downloads/test/new.file".to_string(),
        "test content".to_string(),
    );
    session.remove("/Downloads/test".to_string()).unwrap();
    session.change_dir("Downloads".to_string()).unwrap();
    let out = session.list();
    assert!(!out.contains("test"))
}
#[test]
#[should_panic]
fn test_rm_directory_not_present() {
    let mut session = test_session();
    session.remove("/Missing".to_string()).unwrap();
}
#[test]
fn test_rm_file_presnt() {
    let mut session = test_session();
    session.remove("Downloads/test.hello".to_string()).unwrap();
    session.change_dir("Downloads".to_string()).unwrap();
    let out = session.list();
    assert!(!out.contains("test.hello"))
}
#[test]
#[should_panic]
fn test_rm_file_not_present() {
    let mut session = test_session();
    session
        .remove("Downloads/test.missing".to_string())
        .unwrap();
}
#[test]
fn test_touch_relative() {
    let mut session = test_session();
    session
        .touch("Documents/Files/file.txt".to_string())
        .unwrap();
    session.change_dir("Documents/Files".to_string());
    let out = session.list();
    assert!(out.contains("file.txt"))
}
#[test]
fn test_touch_existing_dir() {
    let mut session = test_session();
    // should be a no op since this is a directory
    session.touch("Documents/paperwork".to_string()).unwrap();
    session
        .change_dir("Documents/paperwork".to_string())
        .unwrap();
    assert!(session.list().is_empty())
}
#[test]
fn test_touch_existing_file_doesnt_overwrite() {
    let mut session = test_session();
    // should be a no op since this is a directory
    session.touch("Downloads/test.hello".to_string()).unwrap();
    let out = session
        .read_file("Downloads/test.hello".to_string())
        .unwrap();
    assert_eq!(out, "hello world".as_bytes())
}
#[test]
fn test_read_file_exists() {
    let session = test_session();
    let out = session
        .read_file("Downloads/test.hello".to_string())
        .unwrap();

    assert_eq!(out, "hello world".as_bytes())
}
#[test]
#[should_panic]
fn test_read_file_missing() {
    let session = test_session();
    let out = session
        .read_file("Downloads/missing.hello".to_string())
        .unwrap();
}
#[test]
#[should_panic]
fn test_read_file_not_file() {
    let session = test_session();
    let out = session.read_file("Downloads".to_string()).unwrap();
}
#[test]
fn test_write_file() {
    let session = test_session();
    let expected = "test contents";
    session
        .write_file(
            "Documents/test.file".to_string(),
            "test contents".to_string(),
        )
        .unwrap();
    let out = session.read_file("Documents/test.file".into()).unwrap();
    assert_eq!(out, expected.as_bytes())
}
#[test]
fn test_write_file_should_overwrite() {
    let session = test_session();
    let expected = "test contents";
    session
        .write_file(
            "Downloads/test.hello".to_string(),
            "test contents".to_string(),
        )
        .unwrap();
    let out = session.read_file("Downloads/test.hello".into()).unwrap();
    assert_eq!(out, expected.as_bytes())
}
#[test]
fn test_find_local_present() {
    let session = test_session();
    let out = session.find_local("Do".to_string()).unwrap();
    assert!(out.contains(&"Downloads".to_string()));
    assert!(out.contains(&"Documents".to_string()));
}
#[test]
fn test_find_local_none() {
    let session = test_session();
    let out = session.find_local("Missing".to_string()).unwrap();
    assert!(out.is_empty())
}
#[test]
fn test_find_local_file() {
    let mut session = test_session();
    session.change_dir("Downloads".to_string()).unwrap();
    let out = session.find_local("test".to_string()).unwrap();
    assert_eq!(out, vec!["test.hello"])
}
#[test]
fn test_cp_file(){
    let mut session = test_session();
    session.copy("Downloads/test.hello".to_string(), "Documents/test.hello".to_string()).unwrap();
    session.change_dir("/Documents".to_string()).unwrap();
    let out = session.read_file("test.hello".to_string()).unwrap();
    assert_eq!(out, "hello world".as_bytes())
}
#[test]
fn test_cp_to_here(){
    let mut session = test_session();
    session.copy("Downloads/test.hello".to_string(), "test.hello".to_string()).unwrap();
    let out: Vec<u8> = session.read_file("test.hello".to_string()).unwrap();
    assert_eq!(out, "hello world".as_bytes())
}
