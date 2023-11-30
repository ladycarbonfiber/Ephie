/*
Special case trie that splits on "/" backed with hashmap
 */
use std::{
    collections::HashMap,
    error::Error,
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub enum FsLike {
    DirectoryLike { children: HashMap<PathBuf, FsLike> },
    FileLike { data: Vec<u8> },
    //TODO Symlinks
}
impl FsLike {
    pub fn new() -> Self {
        return Self::DirectoryLike {
            children: HashMap::new(),
        };
    }
    //Insert new directory
    pub fn insert(&mut self, path: impl AsRef<Path>, node: Self) -> Result<(), &'static str> {
        let mut iter = path.as_ref().iter();
        let Some(node_name) = iter.next_back().map(Path::new) else {
            *self = node;
            return Ok(());
        };
        // Checks path to confirm it is a path of directories
        let mut tree = self;
        for path_part in iter {
            match tree {
                FsLike::DirectoryLike { .. } => {}
                FsLike::FileLike { .. } => return Err("Cannot insert into a non directory"),
            }
            // If parent not found create it
            match tree.get_mut(path_part) {
                Some(..) => {}
                None => {
                    tree.insert(
                        path_part,
                        FsLike::DirectoryLike {
                            children: HashMap::new(),
                        },
                    )?;
                }
            };
            tree = if let Some(tree) = tree.get_mut(path_part) {
                tree
            } else {
                return Err("Nested Dir creation failed");
            }
        }
        match tree {
            FsLike::FileLike { .. } => return Err("Parent node isn't directory"),
            FsLike::DirectoryLike { children } => {
                children.insert(node_name.into(), node);
            }
        }
        Ok(())
    }
    // Returns fs node at path, or none if doesn't exist

    pub fn get(&self, path: impl AsRef<Path>) -> Option<&Self> {
        let path = path.as_ref();
        let (first, rest) = {
            let mut iter = path.iter();
            let first: Option<&Path> = iter.next().map(OsStr::as_ref);
            (first, iter.as_path())
        };
        let Some(first) = first else {
            return Some(self);
        };
        if rest == Path::new(".") {
            return self.get(rest);
        }
        self.children()?
            .get(first)
            .and_then(|child| child.get(rest))
    }
    pub fn get_mut(&mut self, path: impl AsRef<Path>) -> Option<&mut Self> {
        let path = path.as_ref();
        let (first, rest) = {
            let mut iter = path.iter();
            let first: Option<&Path> = iter.next().map(OsStr::as_ref);
            (first, iter.as_path())
        };
        let Some(first) = first else {
            return Some(self);
        };
        if rest == Path::new(".") {
            return self.get_mut(rest);
        }
        self.children_mut()?
            .get_mut(first)
            .and_then(|child| child.get_mut(rest))
    }
    pub fn remove(&mut self, path: PathBuf) -> Result<(), &'static &str> {
        Ok(())
    }
    pub fn children(&self) -> Option<&HashMap<PathBuf, FsLike>> {
        match &self {
            Self::DirectoryLike { children } => Some(children),
            _ => None,
        }
    }
    pub fn children_mut(&mut self) -> Option<&mut HashMap<PathBuf, FsLike>> {
        match self {
            Self::DirectoryLike { children } => Some(children),
            _ => None,
        }
    }
}
