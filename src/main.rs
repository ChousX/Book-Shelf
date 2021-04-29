use std::path::{Path, PathBuf};

// mod nbook;
// use nbook::*;
mod book_shelf;
use book_shelf::*;

const SAVE_PATH: &str = "saves/test.json";
fn main() {
    let config = Config {
            archives: vec![PathBuf::from("/run/media/aggelwick/Big Black/BB-Books")],
            user_directed: false,
            ..Default::default()
        };
    let bs = if let Some(book_shelf) = BookShelf::load_and_config(Path::new(SAVE_PATH), config.clone()) {
        book_shelf
    } else {
        BookShelf::new(config)
    };
    println!("{}", bs);
    bs.save(Path::new(SAVE_PATH));
}
