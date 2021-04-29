use std::path::{Path, PathBuf};

// mod nbook;
// use nbook::*;
mod book_shelf;
use book_shelf::*;

const SAVE_PATH: &str = "saves/test.json";
fn main() {
    let mut bs = if let Some(book_shelf) = BookShelf::load(Path::new(SAVE_PATH)) {
        book_shelf
    } else {
        BookShelf::new(Config {
            archives: vec![PathBuf::from("/run/media/aggelwick/Big Black/BB-Books")],
            user_directed: false,
            ..Default::default()
        })
    };
    println!("{}", bs);
    bs.save(Path::new(SAVE_PATH));

    // let mut bs = BookShelf::default();
    // bs.add_shelve(PathBuf::from("/run/media/aggelwick/Big Black/BB-Books"));
    // bs.init();
    // // let p = Path::new("/home/aggelwick/Code/Rust/book_shelf/saves/test.bs");
    // // let mut bs = BookShelf::load(p);
    // // let mut b = Book::new(PathBuf::from("/run/media/aggelwick/Big Black/BB-Books/NewBook/Aurora Rising by Amie Kaufman & Jay Kristoff"));
    // // b.init()
    // // bs.save(p);
    // // bs.user_fix();
    // // let b1 = "01_book";
    // // let b2 = "02_book";
    // // println!("{}", aux(b1, b2));
}
