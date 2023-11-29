use crate::trie::FsLike;
use std::sync::{Arc, Mutex};

pub enum Command {
    CD(String),
    MKDIR(String),
    LS,
    PWD,
    WHO,
}

pub type FileSystem = Arc<Mutex<FsLike>>;
