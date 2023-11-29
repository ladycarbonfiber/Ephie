use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use crate::system::{Command, FileSystem};
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
    pub working_dir: String,
    pub user: String,
    pub file_system: FileSystem,
}
impl Session {
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
                        println!("Something went wrong reading {}", &self.working_dir);
                        return HashSet::new();
                    }
                }
            }
            None => {
                return HashSet::new();
            }
        }
    }
    pub fn current_dir(&self) -> &str {
        return &self.working_dir;
    }
    pub fn current_user(&self) -> &str {
        return &self.user;
    }
    //TODO handle absolute paths
    pub fn change_dir(&mut self, target: String) -> Result<(), &'static str> {
        let fs = self.file_system.lock().unwrap();
        let current_dir = fs.get(PathBuf::from(&self.working_dir));
        match current_dir {
            None => {
                //Another session might have removed the current directory; We should reset to root in this case
                self.working_dir = "/".to_string();
                return Err("Working dir appears to no long to be valid, resetting to root");
            }
            //TODO handle multi paths
            Some(dir) => {
                let maybe_new_dir = dir.get(PathBuf::from(&target));
                match maybe_new_dir {
                    Some(node) => {
                        match node {
                            DirectoryLike { .. } => {
                                self.working_dir = append_to_path(
                                    PathBuf::from(&self.working_dir),
                                    PathBuf::from(target),
                                )
                                .to_str()
                                .unwrap() //assumes this is all valid path things
                                .to_string();
                            }
                            FileLike { .. } => {
                                return Err("Can't change working directory to a file")
                            }
                        }
                    }
                    None => return Err("Directory not found"),
                };
            }
        }

        Ok(())
    }
    pub fn make_dir(&mut self, target: String) -> Result<(), &'static str> {
        let mut fs = self.file_system.lock().unwrap();
        //If absolute path
        if target.starts_with("/") {
            fs.insert(PathBuf::from(&target), FsLike::new())?;
        } else {
            let current_dir = fs.get_mut(PathBuf::from(&self.working_dir));
            match current_dir {
                Some(node) => match node {
                    DirectoryLike { .. } => {
                        node.insert(PathBuf::from(&target), FsLike::new())?;
                    }
                    _ => {
                        return Err("Cannot insert into not a directory");
                    }
                },
                None => {
                    return Err("Working dir no longer valid resetting to root");
                }
            };
        }

        Ok(())
    }
}
