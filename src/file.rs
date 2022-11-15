use crate::share::Id;
use std::path::PathBuf;

pub struct Librarian{
    roots: Vec<PathBuf>
}

impl Librarian{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn add<T: Into<PathBuf>>(&mut self, dir: T) {
        self.roots.push(dir.into());
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


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){

    }
}