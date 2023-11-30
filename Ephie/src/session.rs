use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::system::FileSystem;
use crate::trie::FsLike::{self, DirectoryLike, FileLike};
// Creates new path from target and ref
fn append_to_path(path: impl Into<OsString>, s: impl AsRef<OsStr>) -> PathBuf {
    let mut p = path.into();
    p.push(s);
    p.into()
}
#[derive(Debug)]
pub struct Session {
    //TODO resolve ownership to be more efficient.
    working_dir: PathBuf,
    user: String,
    pub file_system: FileSystem,
}
impl Session {
    pub fn new(user: String, fs: FileSystem) -> Self {
        Self {
            working_dir: PathBuf::from("/"),
            user,
            file_system: fs,
        }
    }
    //TODO support ls outside of working dir
    pub fn list(&self) -> HashSet<String> {
        let fs = self.file_system.lock().unwrap();
        match fs.get(PathBuf::from(&self.working_dir)) {
            Some(node) => {
                match node {
                    DirectoryLike { children } => {
                        return children
                            .keys()
                            .into_iter()
                            .map(|path| path.clone().into_os_string().into_string().unwrap())
                            .collect::<HashSet<String>>()
                    }
                    _ => {
                        //Shouldn't be possible
                        println!(
                            "Something went wrong reading {:?}",
                            self.working_dir.as_os_str()
                        );
                        return HashSet::new();
                    }
                }
            }
            None => {
                return HashSet::new();
            }
        }
    }
    pub fn current_dir(&self) -> &Path {
        return &self.working_dir;
    }
    pub fn current_user(&self) -> &str {
        return &self.user;
    }
    // Replaces .. with the parent of the current working directory for path navigation
    // TODO support nested relative .. in a path
    // probably would require stepping the working directory
    fn adjust_target(&self, target: &str) -> Result<String, &'static str> {
        let parent = self.working_dir.parent();
        if target.contains("..") {
            if target.starts_with("/") {
                return Err("Parent dir isn't supported in absolute paths");
            }
            match parent {
                Some(path) => Ok(target.replace("..", path.as_os_str().to_str().unwrap())),
                None => return Err("Cannot adjust for .. with no parent"),
            }
        } else {
            Ok(target.to_string())
        }
    }
    pub fn change_dir(&mut self, target: String) -> Result<(), &'static str> {
        let fs = self.file_system.lock().unwrap();
        let mut destination_dir = self.working_dir.clone();
        let adjusted_target = self.adjust_target(&target)?;

        // Pushing a relative path extends it, pushing an absolute path replaces
        destination_dir.push(PathBuf::from(adjusted_target));
        let maybe_new_dir = fs.get(PathBuf::from(&destination_dir));
        match maybe_new_dir {
            Some(node) => match node {
                DirectoryLike { .. } => self.working_dir = destination_dir,
                FileLike { .. } => return Err("Can't change working directory to a file"),
            },
            None => return Err("Directory not found"),
        };

        Ok(())
    }
    pub fn make_dir(&mut self, target: String) -> Result<(), &'static str> {
        let mut fs = self.file_system.lock().unwrap();
        let mut destination_dir = self.working_dir.clone();
        let adjusted_target = self.adjust_target(&target)?;

        // Pushing a relative path extends it, pushing an absolute path replaces
        destination_dir.push(PathBuf::from(adjusted_target));
        fs.insert(PathBuf::from(&destination_dir), FsLike::new())?;

        Ok(())
    }
    pub fn remove(&mut self, target: String) -> Result<(), &'static str> {
        let mut fs = self.file_system.lock().unwrap();
        let mut destination_dir = self.working_dir.clone();
        let adjusted_target = self.adjust_target(&target)?;
        destination_dir.push(PathBuf::from(adjusted_target));
        fs.remove(destination_dir)
    }
    pub fn touch(&mut self, target: String) -> Result<(), &'static str> {
        let mut fs = self.file_system.lock().unwrap();
        let mut destination_dir = self.working_dir.clone();
        let adjusted_target = self.adjust_target(&target)?;
        destination_dir.push(PathBuf::from(adjusted_target));
        fs.insert(destination_dir, FileLike { data: Vec::new() })
    }
    pub fn read_file(&self, target: String) -> Result<Vec<u8>, &'static str> {
        let mut fs = self.file_system.lock().unwrap();
        let mut destination_dir = self.working_dir.clone();
        let adjusted_target = self.adjust_target(&target)?;
        destination_dir.push(PathBuf::from(adjusted_target));
        match fs.get(destination_dir) {
            Some(node) => match node {
                DirectoryLike { .. } => return Err("Can only read files"),
                FileLike { data } => return Ok(data.clone()),
            },
            None => return Err("File not found"),
        }
    }
    pub fn write_file(&self, target: String, content: String) -> Result<(), &'static str> {
        let mut fs = self.file_system.lock().unwrap();
        let mut destination_dir = self.working_dir.clone();
        let adjusted_target = self.adjust_target(&target)?;
        destination_dir.push(PathBuf::from(adjusted_target));
        fs.insert(
            destination_dir,
            FileLike {
                data: content.into_bytes(),
            },
        )
    }
}
