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
    pub book_type: BookType,
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
