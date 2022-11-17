use std::{default, path::PathBuf};

use crate::share::*;
use chrono::Duration;

#[derive(Debug, Default)]
pub struct Book {
    pub author_id: Option<Id>,
    pub series_id: Option<Id>,
    pub publisher_id: Option<Id>,
    pub published: Option<Date>,
    pub book_file: Option<PathBuf>,
    pub book_type: BookType,
}

#[derive(Debug, Default)]
pub enum BookType {
    Audio {
        narators_id: Option<Id>,
        duration: Option<Duration>,
    },
    Writen {
        pages: Option<u16>,
        words: Option<u32>,
    },

    Graphic {},

    #[default]
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
