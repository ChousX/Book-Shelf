mod book;
mod books;
mod container;
mod stored_book;
mod librarian;

pub use book::Book;
pub use books::Books;
pub use container::Container;
use std::collections::HashMap;
pub use stored_book::StordBook;

pub type Id = usize;
pub type Data = Option<Id>;

#[derive(Default)]
pub struct BookShelf {
    books: HashMap<String, StordBook>,
    authour: Container<String>,
    narrator: Container<String>,
    series: Container<String>,
}

impl BookShelf {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, book: Book) {
        //init the internal sorage_book
        let mut in_book = StordBook::default();

        //extract authour if exists
        if let Some(author) = book.authour {
            let id = self.authour.add(author);
            in_book.authour = Some(id);
        }

        //extract narrator if exists
        if let Some(narrator) = book.narrator {
            let id = self.narrator.add(narrator);
            in_book.narrator = Some(id);
        }

        if let Some((series, number)) = book.series {
            let id = self.series.add(series);
            in_book.narrator = Some(id);
            in_book.series_number = number;
        }

        // assining path to stored book
        if book.path.is_some() {
            in_book.path = book.path;
        }

        if book.description.is_some() {
            in_book.description = book.description;
        }

        if book.duration.is_some() {
            in_book.duration = book.duration;
        }

        if book.image_path.is_some() {
            in_book.image_path = book.image_path;
        }

        // inserting stored book into self
        self.books.insert(book.title, in_book);
    }

    fn to_book(&self, title: &str, stored_book: &StordBook) -> Book {
        let authour = if let Some(id) = stored_book.authour {
            if let Some(val) = self.authour.get(id) {
                Some(val.clone())
            } else {
                debug_assert!(false);
                None
            }
        } else {
            None
        };

        let narrator = if let Some(id) = stored_book.narrator {
            if let Some(val) = self.narrator.get(id) {
                Some(val.clone())
            } else {
                debug_assert!(false);
                None
            }
        } else {
            None
        };

        let series = if let Some(id) = stored_book.series {
            if let Some(val) = self.series.get(id) {
                Some((val.clone(), stored_book.series_number))
            } else {
                debug_assert!(false);
                None
            }
        } else {
            None
        };

        Book {
            title: title.to_string(),
            authour,
            narrator,
            path: stored_book.path.clone(),
            description: stored_book.description.clone(),
            duration: stored_book.duration.clone(),
            series,
            image_path: stored_book.image_path.clone(),
            image: None,
        }
    }

    pub fn get_book(&self, title: &str) -> Option<Book> {
        if let Some(stored_book) = self.books.get(title) {
            Some(self.to_book(title, stored_book))
        } else {
            None
        }
    }

    pub fn get_books(&self) -> Books {
        let mut data = Vec::with_capacity(self.books.len());
        for (title, stord_book) in self.books.iter() {
            data.push(self.to_book(title, stord_book));
        }
        Books { data }
    }

    pub fn search(&self, search: Search) -> Option<Book> {
        match search {
            Search::Title(title) => {
                if let Some(data) = self.books.get(title) {
                    Some(self.to_book(title, data))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub enum Search<'a> {
    Title(&'a str),
}
