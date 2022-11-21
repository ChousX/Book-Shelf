use std::{path::PathBuf, collections::{HashMap, hash_map::Keys}};

use chrono::Duration;

type Id = usize;

pub struct Book{
    pub title: Option<String>,
    pub author: Option<String>,
    pub series: Option<String>,
    pub publisher: Option<String>,
    pub copyright: Option<String>,
    pub book_file: Option<PathBuf>,
    pub narator: Option<String>,
    pub duration: Option<Duration>,
}

pub struct BookNode{

}


pub struct BookShelf{
    books: Container<BookNode>,
    publishers: Container<>
}



#[derive(Default, Debug)]
struct Container<T> {
    index: HashMap<String, usize>,
    data: Vec<T>,
}
impl<D> Container<D> {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            data: Vec::new(),
        }
    }

    pub fn get_all_keys(&self) -> Keys<String, usize> {
        self.index.keys()
    }

    pub fn add<Str>(&mut self, key: Str, data: D) -> Id
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

    pub fn get_by_key<Str>(&self, key: Str) -> Option<&D>
    where
        Str: ToString,
    {
        let key = key.to_string();
        if let Some(id)self
            .index
            .get(&key){
                self.data.get(id)
            } else {
                None
            }
    }

    pub fn get_by_key_mut<Str>(&mut self, key: Str) -> Option<&mut D>
    where
        Str: ToString,
    {
        let key = key.to_string();
        let id = self
            .index
            .get(&key)
            .expect("asked for data tjat does nto exist");
        self.get_by_id_mut(*id)
    }

    pub fn get_by_id(&self, id: Id) -> Option<&D> {
        self.data.get(id)
    }

    pub fn get_by_id_mut(&mut self, id: Id) -> Option<&mut D> {
        self.data.get_mut(id)
    }

    pub fn get_id<Str: ToString>(&self, key: Str) -> Option<usize> {
        let key = key.to_string();

        if let Some(id) = self.index.get(&key) {
            Some(*id)
        } else {
            None
        }
    }

    pub fn exists(&self, key: &str) -> bool {
        self.index.contains_key(key)
    }
}

impl<D> Container<D> {
    pub fn find<S: ToString>(&self, key: S) -> Option<&D> {
        if let Some(key) = self.get_id(key) {
            Some(self.get_by_id(key)?)
        } else {
            None
        }
    }

    pub fn fimd_mut<S: ToString>(&mut self, key: S) -> Option<&mut D> {
        if let Some(key) = self.get_id(key) {
            Some(self.get_by_id_mut(key)?)
        } else {
            None
        }
    }
}