use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Default)]
pub struct BookShelf{
    books: HashMap<String, StordBook>,
}
//I need a way to store data
//data.add(book) -> Id,
//data.get(Id) -> Book
 
pub struct Container<T>{
    map: HashMap<Id, T>,
}

impl <T: PartialEq> Container<T>{
    pub fn add(&mut self, input: T) -> Id{
        //this is slow like O(N) slow but adding should not be super common soooo it should be fine
        if let Some(id) = self.find_by_value(&input){
            id
        } else {
            let n_id = self.map.len();
            self.map.insert(n_id, input);
            n_id
        }
    }

    pub fn get(&self, id: Id) -> Option<&T>{
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T>{
        self.map.get_mut(&id)
    }

    fn find_by_value(&self, input: &T) -> Option<Id>{
        self.map.iter().find_map(|(key, val)| if val == input{ return Some(*key)} else {return None})
    }
}

impl<T: Default> Default for Container<T>{
    fn default() -> Self {
        Self { map: HashMap::default() }
    }
}

impl BookShelf{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn add(&mut self, book: Book){
        let mut in_book = StordBook::default();

        // assining path to stored book
        if book.path.is_some(){
            in_book.path = book.path;
        }

        // inserting stored book into self
        self.books.insert(book.title, in_book);
        
    }

    pub fn get(&self, search: Search) -> Option<Book>{
        todo!()
    }
}

#[derive(Default)]
pub struct Book{
    pub title: String,
    pub path: Option<PathBuf>,
}

pub type Id = usize;

#[derive(Default)]
pub struct StordBook{
    pub path: Option<PathBuf>,
}

pub enum Search<'a>{
    Title(&'a str),
}