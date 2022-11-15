use crate::BookShelf;
use crate::share::Id;
use std::path::PathBuf;

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

    pub fn run(&self, book_shelf: &mut BookShelf){
        for root in self.roots.iter(){
            
        }
    }
}

impl Default for Librarian{
    fn default() -> Self {
        Self { roots: Vec::default() }
    }
}
use std::{env, fs};

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){
        
        let mut alex = Librarian::new();
        alex.add("test_data");
        alex.run();
    }
}