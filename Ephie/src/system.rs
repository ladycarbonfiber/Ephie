use std::sync::{Arc, Mutex};
use crate::trie::{Trie};

pub enum Command {
    CD(String),
    MKDIR(String),
    LS,
    PWD,
    WHO,
}

pub type FileSystem = Arc<Mutex<Trie>>;
