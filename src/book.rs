use std::{default, path::PathBuf};

use crate::share::*;
use chrono::Duration;
use nfo::Nfo;

#[derive(Debug, Default)]
pub struct InerBook {
    pub author_id: Option<Id>,
    pub series_id: Option<Id>,
    pub publisher_id: Option<Id>,
    pub published: Option<Date>,
    pub book_file: Option<PathBuf>,
}

pub struct AudioBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub series: Option<String>,
    pub publisher: Option<String>,
    pub copyright: Option<String>,
    pub book_file: Option<PathBuf>,
    pub narator: Option<String>,
    pub duration: Option<Duration>,
}

impl From<Nfo> for AudioBook {
    fn from(value: Nfo) -> Self {
        Self {
            title: value.general.title,
            author: value.general.author,
            series: None,
            publisher: value.general.publisher,
            copyright: value.general.copyright,
            book_file: None,
            narator: value.general.read_by,
            duration: value.general.duration,
        }
    }
}

impl Book for AudioBook {
    fn book_type(&self) -> BookType {
        BookType::Audio
    }
}

#[derive(Debug, Default)]
pub enum BookType {
    Audio,
    #[default]
    None
}

pub trait Book {
    fn title(&self) -> Option<String>{
        None
    }
    fn author(&self) -> Option<String>{
        None
    }
    fn series(&self) -> Option<String>{
        None
    }
    fn publisher(&self) -> Option<String>{
        None
    }
    fn copyright(&self) -> Option<String>{
        None
    }
    fn path(&self) -> Option<PathBuf>{
        None
    }
    fn narator(&self) -> Option<String>{
        None
    }
    fn duration(&self) -> Option<Duration>{
        None
    }
    fn book_type(&self) -> BookType;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
