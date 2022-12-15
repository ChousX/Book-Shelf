use super::Book;

#[derive(Default)]
pub struct Books {
    pub data: Vec<Book>,
}

impl Iterator for Books {
    type Item = Book;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}

impl From<Vec<Book>> for Books {
    fn from(data: Vec<Book>) -> Self {
        Self { data }
    }
}
