use std::path::{Path, PathBuf};

fn main() {

}

type Id = usize;
type Date = ();
pub struct Book{
    titles: Vec<String>,
    authors_id: Vec<Id>,
    series_id: Option<Id>,
    publisher_id: Option<Id>,
    published: Option<Date>,
    book_file: Option<FileType>,

    book_type: Option<BookType>,
}

pub struct BookShelf{
    books: Vec<Book>,
    publishers: Vec<Publisher>,
    series: Vec<Series>,
}

pub struct Publisher{
    name: Option<String>,
    books: Option<Vec<Id>>,
}

pub enum FileType{
    Mp4(PathBuf),
    Mp3(PathBuf),
    M4b(PathBuf),
    Cue(PathBuf),
    Nfo(PathBuf),
}

pub struct Image{
    path: PathBuf
}
pub enum BookType{
    Audio{
        narators_id: Vec<Id>,
        deration: (),
    },
    Writen{
        pages: Option<u16>,
        words: Option<u32>
    },

    Graphic{

    }
}
pub struct Series{
    title: Option<String>,
    book_ids: Vec<usize>,
}