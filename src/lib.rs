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
    pub books: Vec<Book>,
    pub publishers: Vec<Publisher>,
    pub series: Vec<Series>,
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){

    }
}