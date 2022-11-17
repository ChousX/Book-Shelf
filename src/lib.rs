mod book;
pub mod file;
mod person;
mod publisher;
mod series;
pub mod share;

use std::path::PathBuf;

use share::*;

use book::*;
use person::*;
use publisher::*;
use series::*;
/*
    to sleepy to do this but I think we should revam the book shelf 
    convert every thing to hashtables and use the unquice data as the key
*/
pub struct BookShelf {
    books: Vec<Book>,
    publishers: Vec<Publisher>,
    series: Vec<Series>,
    narator: Vec<Person>,
    author: Vec<Person>,
}

impl BookShelf {
    pub fn add_book(
        &mut self,
        title: Option<String>,
        authors: Option<Person>,
        series: Option<Series>,
        publisher: Option<Publisher>,
        published: Option<Date>,
        book_file: Option<PathBuf>,
        book_type: Option<BookType>,
    ) -> Id {
        //
        todo!()
    }

    fn add_publisher(&mut self, publisher: Publisher) -> Id {
        Self::register(&mut self.publishers, publisher)
    }
    fn add_series(&mut self, series: Series) -> Id {
        Self::register(&mut self.series, series)
    }
    fn add_narator(&mut self, narator: Person) -> Id {
        Self::register(&mut self.narator, narator)
    }
    fn add_author(&mut self, author: Person) -> Id {
        Self::register(&mut self.author, author)
    }
    fn register<T>(v: &mut Vec<T>, subject: T) -> Id
    where
        T: PartialEq,
    {
        //check if its there already
        if let Some(entry) = v.iter().position(|x| x == &subject) {
            // check if there are any new information
            //Todo
            entry as Id
        } else {
            let len = v.len();
            v.push(subject);
            len as Id
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
