mod book;
mod file;
mod publisher;
mod series;
mod narator;
pub mod share;

use share::*;

use book::*;
use publisher::*;
use series::*;
use narator::*;
use file::*;

pub struct BookShelf{
    books: Vec<Book>,
    publishers: Vec<Publisher>,
    series: Vec<Series>,
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){

    }
}