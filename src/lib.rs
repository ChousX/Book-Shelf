use chrono::Duration;
use eframe::egui::Ui;
use egui_extras::RetainedImage;
use image::open;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
mod app;
pub use app::*;

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

#[derive(Default)]
pub struct Books {
    data: Vec<Book>,
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
    pub duration: Option<Duration>,
    pub series: Option<(String, u8)>,
    pub image_path: Option<PathBuf>,
    pub image: Option<RetainedImage>,
}

impl Book {
    pub fn set_image(&mut self) -> bool {
        if let Some(path) = &self.image_path {
            let image_bytes = match open(path) {
                Ok(file) => file.to_rgb8().into_raw(),
                _ => return false,
            };
            if let Ok(image) = RetainedImage::from_image_bytes(&self.title, &image_bytes) {
                self.image = Some(image);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub type Id = usize;
pub type Data = Option<Id>;

#[derive(Default)]
pub struct StordBook {
    pub authour: Data,
    pub narrator: Data,
    pub path: Option<PathBuf>,
    pub description: Option<String>,
    pub duration: Option<Duration>,
    pub series: Data,
    pub series_number: u8,
    pub image_path: Option<PathBuf>,
}

pub enum Search<'a> {
    Title(&'a str),
}
