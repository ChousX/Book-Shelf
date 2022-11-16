use crate::BookShelf;
use crate::share::*;
use std::fs;
use std::path::PathBuf;

/// A builder ish styled object that will catilog files comming in
pub struct Librarian{
    roots: Vec<PathBuf>
}

impl Librarian{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn add<T: Into<PathBuf>>(&mut self, dir: T) -> &mut Self{
        self.roots.push(dir.into());
        self
    }

    /// Will make a vector containing book paths 
    pub fn get_all(&self) -> Vec<PathBuf>{
            let mut out = Vec::new();
        for root in self.roots.iter(){
            let mut queue = {
                let dirs = match fs::read_dir(root){
                    Ok(root) => root,
                    Err(_) => {
                        warn!("error in reading in root files");
                        continue
                    }
                };
                let mut queue = std::collections::VecDeque::new();
                for p in dirs{
                    if let Ok(entry) = p{
                        queue.push_back(entry.path());
                    }
                }
                queue
            };
            
            while let Some(path) = queue.pop_front() {
                if path.is_dir(){
                    if let Some(mut dirs) = dir_containing_dir(&path){
                        for dir in dirs{
                            queue.push_back(dir);
                        }
                    } else {
                        out.push(path);
                    }
                }
            }
        }
    out
    }
    pub fn run(&self, book_shelf: &mut BookShelf){
        for path in self.get_all().into_iter(){
            if path.is_dir(){
                // look for .nfo
            } else {
                //examin meta data

            }
        }
    }
}

/*
    Dir{
        .m4b
    }

    Dir{
        .m4b
        .nfo
    }

    Dir{
        .cue
        .jpg
        .m4b
        .nfo
    }

    

    Dir{
        .jpg
        .jpg
        .cue
        .m4b
        .nfo
    }
 */

fn dir_containing_dir(dir: &PathBuf) -> Option<Vec<PathBuf>>{
    debug_assert!(dir.is_dir());

    let mut out = Vec::new();

    let dirs = match fs::read_dir(dir){
        Ok(root) => root,
        Err(_) => {info!(""); return None}
    };

    for path in dirs{
        let path = match path {
            Ok(path) => path,
            Err(_) => {
                warn!("path entry failed to unwrap");
                continue
            }
        };
        if path.path().is_dir(){
            out.push(path.path())
        }
    }

    if out.is_empty(){
        None
    } else{
        Some(out)
    }
}

enum FileType{
    Dir(PathBuf),
    File(PathBuf)
}

impl Default for Librarian{
    fn default() -> Self {
        Self { roots: Vec::default() }
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){
    
    }
}