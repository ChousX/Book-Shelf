mod book;
pub mod file;
mod person;
mod publisher;
mod series;
pub mod share;

use share::*;

use book::*;
use person::*;
use publisher::*;
use series::*;

pub struct BookShelf {
    books: Vec<Book>,
    publishers: Vec<Publisher>,
    series: Vec<Series>,
    narator: Vec<Person>,
    author: Vec<Person>,
}

impl BookShelf{
    pub fn add_book(&mut self, book: Book) -> Id{
        Self::register(&mut self.books, book)
    }
    pub fn add_publisher(&mut self, publisher: Publisher) -> Id{
        Self::register(&mut self.publishers, publisher)
    }
    pub fn add_series(&mut self, series: Series) -> Id{
        Self::register(&mut self.series, series)
    }
    pub fn add_narator(&mut self, narator: Person) -> Id{
        Self::register(&mut self.narator, narator)
    }
    pub fn add_author(&mut self, author: Person) -> Id{
        Self::register(&mut self.author, author)
    }
    fn register<T>(v: &mut Vec<T>, subject: T) -> Id
    where T: PartialEq{
        //check if its there already
        if let Some(entry) = v.iter().position(|x|{x == &subject}){
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
