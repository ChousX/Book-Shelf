use std::{convert::TryFrom, fs};

use crate::file::Extention;

use book_shelf::{file::Librarian, *};
fn main() {
    let mut book_shelf = BookShelf::default();
    let mut l = Librarian::new();
    l.add(r"V:\Local-Books").run(&mut book_shelf);
    for author in book_shelf.get_authors().into_iter(){
        println!("{author}");
    }
    
}
