mod book;
mod books;
mod container;
mod librarian;
mod stored_book;

pub use book::Book;
pub use books::Books;
use chrono::Duration;
pub use container::Container;
pub use librarian::run;
use std::collections::HashMap;
pub use stored_book::StordBook;

pub type Id = usize;
pub type Data = Option<Id>;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct BookShelf {
    books: HashMap<String, StordBook>,
    authour: Container<String>,
    narrator: Container<String>,
    series: Container<String>,
    duration: Container<String>,
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

        if let Some(duration) = book.duration{
            let string = duration_to_string(&duration);
            let id = self.duration.add(string);
            in_book.duration = Some(id);
        }

        if book.image_path.is_some() {
            in_book.image_path = book.image_path;
        }

        // inserting stored book into self
        self.books.insert(book.title, in_book);
    }

    pub fn add_books(&mut self, books: Books) {
        for book in books {
            self.add(book);
        }
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
            duration: None,
            series,
            image_path: stored_book.image_path.clone(),
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

fn duration_to_string(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds();
    format!("{hours}:{minutes}:{seconds}")
}

fn string_to_duration(string: &str) -> Duration{
    let mut blah = string.split(":");
    let mut output = Duration::zero();
    if let Some(hours) = blah.next() {
        let hours: i64 = hours.parse().unwrap();
        output = output + Duration::hours(hours);
    }
    if let Some(min) = blah.next() {
        let min: i64 = min.parse().unwrap();
        output = output + Duration::minutes(min);
    }
    if let Some(sec) = blah.next() {
        let sec: i64 = sec.parse().unwrap();
        output = output + Duration::seconds(sec);
    }
    output
}