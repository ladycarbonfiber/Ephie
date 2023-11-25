/*
Special case trie that splits on "/" backed with hashmap
 */ 
use std::{collections::HashMap}; 

#[derive(Debug, Clone)]
pub enum Node {
    DirectoryLike {children: HashMap<String, Node>},
    FileLike {data:Vec<u8>},
}
impl Node {
    pub fn new(data:Option<Vec<u8>>)-> Self {
        match data {
            Some(data) =>{
                Node::FileLike { data }
            },
            None =>{
                Node::DirectoryLike { children: HashMap::new() }
            }
            
        }
        
    }
}
#[derive(Debug, Clone)]
pub struct Trie {
    root: Node
}
impl Trie {
    pub fn new() -> Self {
        // Root is always a directory /
        Trie {root: Node::new(None)}
    }
    fn insert(&mut self, abs_path:String, data:Option<Vec<u8>>) {
        let mut corrected_path = abs_path.clone();
        if abs_path.ends_with("/"){
            corrected_path = corrected_path[0..corrected_path.len()-1].to_string();
        }
        let mut cur_node = &mut self.root;
        let parts = corrected_path.split("/").collect::<Vec<&str>>();
        let l = parts.len();
        for part in parts.into_iter().take(l -1){
            if part.len() > 0 {
                match cur_node {
                    Node::DirectoryLike { children }=> {
                    },
                    Node::FileLike { data } => {

                    }
                }
            } else {
              println!("Weird part  {}", part)
            }
        }
    }
}