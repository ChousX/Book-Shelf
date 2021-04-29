use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{collections::HashSet, fmt::format};
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
        let mut possible_file_locations = config.get_possible_file_location();

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
    fn init_as_needed(&mut self){
        let config = self.config.clone();
        let mut possible_file_location = config.get_possible_file_location();
        for p_book in possible_file_location{
            if !self.books.iter().any(|b|{
                b.path == p_book 
            }){
                if let Some(book) = Book::build(p_book, &config, &self.authors, &self.narrators) {
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
    }
    pub fn load(path: &Path) -> Option<Self> {
        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(_) => return None,
        };
        let mut bs: BookShelf = serde_json::from_slice(&data).expect("failed to deserialize");
        bs.init_as_needed();
        Some(bs)
    }
    pub fn save(&self, path: &Path) {
        let mut s = serde_json::to_string(self).expect("failed to serialize");
        std::fs::write(path, s).expect("failed to write to file");
    }
}

impl std::fmt::Display for BookShelf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acum = String::new();
        if !self.books.is_empty() {
            acum.push_str("Book:\n");
            for book in self.books.iter() {
                let f = format!("{}\n", &book);
                acum.push_str(&f);
            }
            acum.push_str("\n");
        }
        write!(f, "{}", acum)
    }
}
