mod book;
pub mod file;
mod person;
mod publisher;
mod series;
pub mod share;

use std::{collections::{HashMap, hash_map::Keys}, path::{PathBuf, Path}};

use nfo::Nfo;
use share::*;

use book::*;
use person::*;
use publisher::*;
use series::*;

#[derive(Default, Debug)]
pub struct BookShelf {
    books: Container<Book>,
    publishers: Container<Publisher>,
    series: Container<Series>,
    narators: Container<Person>,
    authors: Container<Person>,
}

impl BookShelf{
    pub fn get_authors(&self) -> Vec<&String>{
        self.authors.get_all_keys().collect()
    }
}


impl BookShelf {
    pub fn add_book(
        &mut self,
        title: String,
        author: Option<Person>,
        series: Option<Series>,
        publisher: Option<Publisher>,
        published: Option<Date>,
        book_file: Option<PathBuf>,
        book_type: Option<BookType>,
    ) -> Id {
        todo!()
    }

    ///should only be used when you know book does not exist
    pub fn add_book_nfo(&mut self, mut nfo: Nfo, path: &Path) {
        let book_file = Some(path.into());
        if let Some(title) = nfo.general.title {
            let author = nfo.general.author;
            //missing series form nfo
            let publisher = nfo.general.publisher;
            let publised = nfo.general.copyright;
            let narator = nfo.general.read_by;
            let duration = nfo.general.duration;

            if let Some(book_id) = self.books.get_id(&title){
                //compare and update None feilds
                //author
                if self.books.get_by_id(book_id).author_id.is_none() && author.is_some(){
                    if let Some(author) = author{
                        let id = self.add_author(author, Person::default());
                        self.books.get_by_id_mut(book_id).author_id = Some(id);
                    }
                }
                
                //publisher
                if self.books.get_by_id(book_id).publisher_id.is_none() && publisher.is_some(){
                    if let Some(publisher) = publisher{
                        let id = self.add_publisher(publisher, Publisher::default());
                        self.books.get_by_id_mut(book_id).publisher_id = Some(id);
                    }
                }

                match self.books.get_by_id_mut(book_id).book_type{
                    BookType::Audio { mut narators_id, duration: mut duration_id } => {
                        if narators_id.is_none(){
                            if let Some(string) = narator{
                                let id = self.add_narator(string, Person::default());
                                narators_id = Some(id);
                            }
                        }
                        
                        if duration.is_none(){
                            duration_id = duration;
                        }

                        
                    },

                    BookType::Writen { pages, words } => {

                    },
                    BookType::Graphic {  } => {},
                    BookType::None => {}
                }
            } else {
                 
                let author_id = if let Some(author) = author{
                    Some(self.add_author(author, Person::default()))
                } else{
                    None
                };

                let publisher_id = if let Some(publisher) = publisher{
                    Some(self.add_publisher(publisher, Publisher::default()))
                } else{
                    None
                };

                let narators_id = if let Some(publisher) = narator{
                    Some(self.add_narator(publisher, Person::default()))
                } else{
                    None
                };

                let book_type = BookType::Audio { narators_id, duration};

                // So still missing Series and Published
                let book = Book{
                    author_id,
                    series_id: None,
                    publisher_id,
                    published: None,
                    book_file,
                    book_type,
                };
                
                let book_id = self.books.add(title, book);
                
                //now we have the book id we need to add it back the fields that refrence book
                if let Some(id) = author_id{
                    self.add_book_to_author(book_id, id);
                }

                if let Some(id) = publisher_id{
                    self.add_book_to_publisher(book_id, id)
                }

                if let Some(id) = narators_id{
                    self.add_book_to_narator(book_id, id);
                }

                
            }

            //for now lets assume its an audio book
            
        }
    }

    pub fn add_author<Str: ToString>(&mut self, name: Str,person: Person) -> Id{
        let key = name.to_string();
        self.authors.add(key, person)
    }

    pub fn add_narator<Str: ToString>(&mut self, name: Str, person: Person) -> Id{
        let key = name.to_string();
        self.narators.add(key, person)
    }

    pub fn add_publisher<Str: ToString>(&mut self, name: Str, publisher: Publisher) -> Id{
        let key = name.to_string();
        self.publishers.add(key, publisher)
    }

    pub fn add_series<Str: ToString>(&mut self, name: Str, series: Series) -> Id{
        let key = name.to_string();
        self.series.add(key, series)
    }

    fn add_book_to_author(&mut self, book: Id, author: Id){
        let person = self.authors.get_by_id_mut(author);
        //I don't think we will end up with multible book entryes so not going to mess with it for now
        person.works.push(book);
    }

    fn add_book_to_publisher(&mut self, book: Id, publisher: Id){
        let publisher = self.publishers.get_by_id_mut(publisher);
        publisher.works.push(book);
    }

    fn add_book_to_series(&mut self, book: Id, series: Id){
        let series = self.series.get_by_id_mut(series);
        series.book_ids.push(book)
    }

    fn add_book_to_narator(&mut self, book: Id, narator: Id){
        let narator = self.narators.get_by_id_mut(narator);
        narator.works.push(book)
    }
}

#[derive(Default, Debug)]
struct Container<T> {
    index: HashMap<String, usize>,
    data: Vec<T>,
}
impl<Data> Container<Data> {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            data: Vec::new(),
        }
    }

    pub fn get_all_keys(&self) -> Keys<String, usize>{
        self.index.keys()
    }
    pub fn add<Str>(&mut self, key: Str, data: Data) -> Id
    where
        Str: ToString,
    {
        let key = key.to_string();
        if let Some(entry) = self.index.get(&key) {
            //todo add missing fealds
            *entry
        } else {
            let len = self.data.len();
            self.index.insert(key, len);
            self.data.push(data);
            len
        }
    }

    pub fn get_by_key<Str>(&self, key: Str) -> &Data
    where
        Str: ToString,
    {
        let key = key.to_string();
        let id = self.index.get(&key).expect("asked for data tjat does nto exist");
        self.get_by_id(*id)
    }
    pub fn get_by_key_mut<Str>(&mut self, key: Str) -> &mut Data
    where
        Str: ToString,
    {
        let key = key.to_string();
        let id = self.index.get(&key).expect("asked for data tjat does nto exist");
        self.get_by_id_mut(*id)
    }

    pub fn get_by_id(&self, id: Id) -> &Data {
        &self.data[id as usize]
    }

    pub fn get_by_id_mut(&mut self, id: Id) -> &mut Data {
        &mut self.data[id as usize]
    }

    pub fn get_id<Str: ToString>(&self, key: Str) -> Option<usize>{
        let key = key.to_string();
        
        if let Some(id) = self.index.get(&key){
            Some(*id)
        } else {
            None
        }
    }

    pub fn exists(&self, key: &str) -> bool{
        self.index.contains_key(key)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
