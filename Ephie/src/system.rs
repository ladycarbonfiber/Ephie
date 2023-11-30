use crate::trie::FsLike;
use std::sync::{Arc, Mutex};

pub type FileSystem = Arc<Mutex<FsLike>>;
