use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Default)]
pub struct BookShelf{
    books: HashMap<String, StordBook>,
    authour: Container<String>,
    narrator: Container<String>,
}
 
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



    pub fn find_by_value(&self, input: &T) -> Option<Id>{
        self.map.iter().find_map(|(key, val)| if val == input{ return Some(*key)} else {return None})
    }
}

impl <T> Container<T>{
    pub fn get(&self, id: Id) -> Option<&T>{
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T>{
        self.map.get_mut(&id)
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
        //init the internal sorage_book
        let mut in_book = StordBook::default();

        //extract authour if exists
        if let Some(author) = book.authour{
            let id = self.authour.add(author);
            in_book.authour = Some(id);
        }

        //extract narrator if exists
        if let Some(narrator) = book.narrator{
            let id = self.narrator.add(narrator);
            in_book.narrator = Some(id);
        }
        
        // assining path to stored book
        if book.path.is_some(){
            in_book.path = book.path;
        }

        // inserting stored book into self
        self.books.insert(book.title, in_book);
        
    }

    fn to_book(&self, title: &str) -> Option<Book>{
        if let Some(stored_book) = self.books.get(title){
            let authour = if let Some(id) = stored_book.authour {
                if let Some(val) = self.authour.get(id){
                    Some(val.clone())
                } else {
                    debug_assert!(false);
                    None
                }
            } else {
                None
            };

            let narrator = if let Some(id) = stored_book.narrator {
                if let Some(val) = self.narrator.get(id){
                    Some(val.clone())
                } else {
                    debug_assert!(false);
                    None
                }
            } else {
                None
            };
            Some(Book{
                title: title.to_string(),
                authour,
                narrator,
                path: stored_book.path.clone(),
            })
        } else {
            None
        }
    }

    pub fn get(&self, search: Search) -> Option<Book>{
        todo!()
    }
}

#[derive(Default)]
pub struct Book{
    pub title: String,
    pub authour: Option<String>,
    pub narrator: Option<String>,
    pub path: Option<PathBuf>,
}

pub type Id = usize;
pub type Data = Option<Id>;
#[derive(Default)]
pub struct StordBook{
    pub authour: Data,
    pub narrator: Data,
    pub path: Option<PathBuf>,
}

pub enum Search<'a>{
    Title(&'a str),
}