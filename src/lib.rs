mod book;
pub mod file;
mod narator;
mod publisher;
mod series;
pub mod share;

use share::*;

use book::*;
use narator::*;
use publisher::*;
use series::*;

pub struct BookShelf {
    pub books: Vec<Book>,
    pub publishers: Vec<Publisher>,
    pub series: Vec<Series>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
