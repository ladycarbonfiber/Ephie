use crate::system::{FileSystem, Command};
#[derive(Debug)]
pub struct Session {
    //TODO resolve ownership to be more efficient. 
    pub working_dir: String,
    pub user: String,
    pub file_system:FileSystem
}
impl Session {
    
    pub fn resolve_command(&mut self, command:Command) -> String{
        match command {
            Command::CD(s) => {
                self.working_dir = s.to_owned();
                return self.working_dir.clone();
    
            }
            Command::LS => {
                let fs = self.file_system.lock().unwrap();
                return "Unimplemented".to_owned();
            },
            Command::MKDIR(s) =>{
                return "Unimplemented".to_owned();
            },
            Command::PWD =>{
                return self.working_dir.clone();
            },
            Command::WHO =>{
                return self.user.clone();
            }
        }
    }
}