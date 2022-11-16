use std::{convert::TryFrom, fs};

use crate::file::Extention;

use book_shelf::{file::Librarian, *};
fn main() {
    let mut l = Librarian::new();
    l.add(r"V:\Local-Books").run();
}
