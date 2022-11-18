use std::path::PathBuf;

use book_shelf::{file::Librarian, *};
use chrono::Duration;
fn main() {
    let mut book_shelf = BookShelf::default();
    let mut l = Librarian::new();
    l.add(r"V:\Local-Books").run(&mut book_shelf);
    for author in book_shelf.get_authors().into_iter() {
        println!("{author}");
    }
    if cfg!(cli) {
        println!("woop")
    }
}
