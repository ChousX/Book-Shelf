use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
mod book;
mod config;
pub use book::*;
pub use config::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct BookShelf {
    config: Config,
    books: Vec<Book>,
    authors: HashSet<String>,
    narrators: HashSet<String>,
}
impl Default for BookShelf {
    fn default() -> Self {
        Self {
            config: Config::default(),
            books: vec![],
            authors: HashSet::new(),
            narrators: HashSet::new(),
        }
    }
}
impl BookShelf {
    pub fn new(config: Config) -> Self {
        let mut output = Self {
            config,
            ..Default::default()
        };
        output.init();
        output
    }
    fn init(&mut self) {
        let config = self.config.clone();
        let mut possible_file_locations = vec![];
        for path in config.archives {
            possible_file_locations.append(&mut get_dir(path.as_path()));
        }
        for p_book in possible_file_locations {
            if let Some(book) = Book::build(p_book, &self.config, &self.authors, &self.narrators) {
                book.authors.iter().for_each(|x| {
                    self.authors.insert(String::from(x));
                });
                book.narrators.iter().for_each(|x| {
                    self.narrators.insert(String::from(x));
                });
                self.books.push(book);
            }
        }
    }
    pub fn load(path: &Path) -> Self{
        unimplemented!()
    }
    pub fn save(&self, path: &Path){
        
    }
}
fn get_dir(root: &Path) -> Vec<PathBuf> {
    fn aux(root: &Path, output: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in std::fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                aux(path.as_path(), output);
                output.push(path);
            }
        }
        Ok(())
    }
    let mut output = vec![];
    match aux(root, &mut output) {
        _ => {}
    }
    output
}
