use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct BookShelf {
    books: HashMap<String, StordBook>,
    authour: Container<String>,
    narrator: Container<String>,
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

        // assining path to stored book
        if book.path.is_some() {
            in_book.path = book.path;
        }

        if book.description.is_some() {
            in_book.description = book.description;
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

        Book {
            title: title.to_string(),
            authour,
            narrator,
            path: stored_book.path.clone(),
            description: stored_book.description.clone(),
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

pub struct Books {
    data: Vec<Book>,
}

impl Iterator for Books {
    type Item = Book;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}

pub struct Container<T> {
    map: HashMap<Id, T>,
}

impl<T: PartialEq> Container<T> {
    pub fn add(&mut self, input: T) -> Id {
        //this is slow like O(N) slow but adding should not be super common well running soooo it should be fine
        // it may make initing taxing
        if let Some(id) = self.find_by_value(&input) {
            id
        } else {
            let n_id = self.map.len();
            self.map.insert(n_id, input);
            n_id
        }
    }

    pub fn find_by_value(&self, input: &T) -> Option<Id> {
        self.map.iter().find_map(|(key, val)| {
            if val == input {
                return Some(*key);
            } else {
                return None;
            }
        })
    }
}

impl<T> Container<T> {
    pub fn get(&self, id: Id) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        self.map.get_mut(&id)
    }
}

impl<T: Default> Default for Container<T> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}

#[derive(Default)]
pub struct Book {
    pub title: String,
    pub authour: Option<String>,
    pub narrator: Option<String>,
    pub path: Option<PathBuf>,
    pub description: Option<String>,
}

pub type Id = usize;
pub type Data = Option<Id>;

#[derive(Default)]
pub struct StordBook {
    pub authour: Data,
    pub narrator: Data,
    pub path: Option<PathBuf>,
    pub description: Option<String>,
}

pub enum Search<'a> {
    Title(&'a str),
}
