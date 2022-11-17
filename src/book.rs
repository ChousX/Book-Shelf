use std::{path::PathBuf, default};

use crate::share::*;
use chrono::Duration;
use nfo::Nfo;

#[derive(Debug, Default)]
pub struct Book {
    title: Option<String>,
    authors_id: Option<Id>,
    
    series_id: Option<Id>,
    publisher_id: Option<Id>,
    published: Option<Date>,
    book_file: Option<PathBuf>,

    book_type: Option<BookType>,
}

/// Only comparing titales
impl PartialEq for Book{
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[derive(Debug, Default)]
pub enum BookType {
    
    Audio {
        narators_id: Option<Id>,
        deration: Duration,
    },
    Writen {
        pages: Option<u16>,
        words: Option<u32>,
    },

    Graphic {},

    #[default]
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
