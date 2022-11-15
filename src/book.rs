use crate::share::*;

pub struct Book{
    titles: Vec<String>,
    authors_id: Vec<Id>,
    series_id: Option<Id>,
    publisher_id: Option<Id>,
    published: Option<Date>,
    book_file: Option<()>,

    book_type: Option<BookType>,
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