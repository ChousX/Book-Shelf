use crate::share::Id;
use std::path::PathBuf;

pub struct Librarian{
    roots: Vec<PathBuf>
}

impl Librarian{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn add<T: ToString>(&mut self, dir: T) {
        let dir = PathBuf::from(dir.to_string());
        self.roots.push(dir);
    }

    pub fn run(&self){
        for root in self.roots.iter(){

        }
    }
}

impl Default for Librarian{
    fn default() -> Self {
        Self { roots: Vec::default() }
    }
}